#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qvector.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qmap.h");
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(usize, offset)]
        #[qproperty(usize, total)]
        #[qproperty(usize, render_amount)]
        type MediaList = super::MediaListRust;

        #[qsignal]
        fn append(self: Pin<&mut MediaList>, media: QMap_QString_QVariant);

        #[qsignal]
        fn clear(self: Pin<&mut MediaList>);

        #[qinvokable]
        fn rerender(self: Pin<&mut MediaList>);
    }

    impl cxx_qt::Threading for MediaList {}
    impl cxx_qt::Constructor<()> for MediaList {}
}

use std::{cmp::min, pin::Pin, sync::{mpsc::channel, RwLock}, thread};

use qobject::{QMap_QString_QVariant, QString, QUrl};
use cxx_qt::Threading;

use crate::{config::CONFIG, media::TypeMetadata, server::listen, wait_for_config};

#[derive(Default)]
pub struct MediaListRust {
    media: RwLock<Vec<QMap_QString_QVariant>>,
    offset: usize,
    total: usize,
    render_amount: usize
}

impl qobject::MediaList {
    fn rerender(mut self: Pin<&mut Self>) {
        let list = self.media.read().unwrap();

        let start = self.offset;
        let end = min(self.offset + self.render_amount, list.len());
        let chunk: Vec<QMap_QString_QVariant> = list[start..end].iter().cloned().collect();
        drop(list);

        self.as_mut().clear();
        for media in chunk {
            self.as_mut().append(media.clone());
        }
    }
}

impl cxx_qt::Initialize for qobject::MediaList {
    fn initialize(mut self: Pin<&mut Self>) {
        let (tx, rx) = channel();
        let qthread = self.qt_thread();

        thread::spawn(move || listen(tx));

        wait_for_config();
        self.as_mut().set_render_amount(CONFIG.get().unwrap().render_amount);

        thread::spawn(move || {

            while let Ok(batch) = rx.recv() {
                qthread.queue(|mut media_list| {
                    let conv = |v: String| (&QString::from(&v)).into();

                    let mut new_media: Vec<QMap_QString_QVariant> = batch.into_iter().map(|media| {
                        let mut value = QMap_QString_QVariant::default();

                        value.insert("url".into(), (&QUrl::from(&media.url)).into());
                        value.insert("title".into(), conv(media.essential_metadata.title));
                        value.insert("creation".into(), (&media.essential_metadata.creation).into());

                        match media.type_metadata {
                            TypeMetadata::Image { width, height } => {
                                value.insert("width".into(), (&width).into());
                                value.insert("height".into(), (&height).into());
                                value.insert("type".into(), conv("image".to_string()));
                            },
                            TypeMetadata::PDF { author, summary } => {
                                value.insert("width".into(), conv(author));
                                value.insert("height".into(), conv(summary));
                                value.insert("type".into(), conv("pdf".to_string()));
                            }
                        };

                        if let Some(extra) = media.extra_metadata {
                            extra.into_iter().for_each(|(k, v)|  value.insert(k.into(), conv(v.to_string())));
                        }

                        value
                    }).collect();

                    let new_media_copy = new_media.clone();


                    let mut list = media_list.media.write().unwrap();
                    let len = list.len();
                    list.append(&mut new_media);
                    drop(list);

                    media_list.as_mut().set_total(len);
                    for (i, media) in new_media_copy.iter().enumerate() {
                        let position = len + i;
                        // if media position would be within the current viewport, add it to the viewport
                        if position >= media_list.offset && position < media_list.offset + media_list.render_amount {
                            media_list.as_mut().append(media.clone());
                        }
                    }
                }).expect("could not queue update");
            }
        });
    }
}

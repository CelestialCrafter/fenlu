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
        type MediaList = super::MediaListRust;

        #[qobject]
        #[qml_element]
        #[qproperty(QString, title)]
        #[qproperty(QUrl, url)]
        type QMedia = super::QMediaRust;

        #[qsignal]
        #[cxx_name = "recvNew"]
        fn recv_new(self: Pin<&mut MediaList>, media: QMap_QString_QVariant);
    }

    impl cxx_qt::Threading for MediaList {}
    impl cxx_qt::Constructor<()> for MediaList {}
}

use std::{pin::Pin, sync::mpsc::channel, thread};

use qobject::{QMap_QString_QVariant, QString, QUrl};
use cxx_qt::Threading;

use crate::{media::TypeMetadata, server::listen};

#[derive(Default)]
pub struct QMediaRust {
    url: QUrl,
    title: QString,
}

#[derive(Default)]
pub struct MediaListRust {}

impl cxx_qt::Initialize for qobject::MediaList {
    fn initialize(self: Pin<&mut Self>) {
        let (tx, rx) = channel();
        let qthread = self.qt_thread();

        thread::spawn(move || listen(tx));
        thread::spawn(move || {
            while let Ok(media) = rx.recv() {
                let mut value: QMap_QString_QVariant = QMap_QString_QVariant::default();
                let conv = |v: String| (&QString::from(&v)).into();

                value.insert("url".into(), conv(media.url));
                value.insert("type".into(), conv(media.type_metadata.to_string()));
                value.insert("title".into(), conv(media.essential_metadata.title));
                value.insert("creation".into(), (&media.essential_metadata.creation).into());

                match media.type_metadata {
                    TypeMetadata::Image { width, height } => {
                        value.insert("width".into(), (&width).into());
                        value.insert("height".into(), (&height).into());
                    },
                    TypeMetadata::PDF { author, summary } => {
                        value.insert("width".into(), conv(author));
                        value.insert("height".into(), conv(summary));
                    }
                };

                if let Some(extra) = media.extra_metadata {
                    extra.into_iter().for_each(|(k, v)|  value.insert(k.into(), conv(v.to_string())));
                }

                qthread.queue(move |mut media_list| media_list.as_mut().recv_new(value)).expect("could not queue update");
            }
        });
    }
}

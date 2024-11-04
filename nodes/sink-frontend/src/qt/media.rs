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

use cxx_qt_lib::{QMap, QMapPair_QString_QVariant};
use qobject::{QString, QUrl};
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
                let mut value: QMap<QMapPair_QString_QVariant> = QMap::default();

                value.insert("url".into(), (&QUrl::from(&media.url)).into());
                value.insert("title".into(), (&QString::from(&media.essential_metadata.title)).into());
                value.insert("type".into(), (&QString::from(&media.type_metadata.to_string())).into());
                if let TypeMetadata::Image { width, height } = media.type_metadata {
                    value.insert("width".into(), (&width).into());
                    value.insert("height".into(), (&height).into());
                }

                qthread.queue(move |mut media_list| media_list.as_mut().recv_new(value)).expect("could not queue update");
            }
        });
    }
}

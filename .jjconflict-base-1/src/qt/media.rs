#[cxx_qt::bridge(cxx_file_stem = "media")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(usize, total)]
        type FenluMedia = super::Media;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn item(self: &FenluMedia, index: usize) -> QString;
        #[qinvokable]
        fn open(self: &FenluMedia, url: QUrl);
        #[qinvokable]
        fn set_query(self: Pin<&mut FenluMedia>, key: QString, query: QString);
        #[qinvokable]
        fn handle_pipeline(self: Pin<&mut FenluMedia>);
    }

    impl cxx_qt::Threading for FenluMedia {}
    impl cxx_qt::Constructor<()> for FenluMedia {}
}

use std::{pin::Pin, time::Instant};

use cxx_qt_lib::{QString, QUrl};
use futures::executor::block_on;
use qobject::{FenluMedia, FenluMediaCxxQtThread};
use tokio::{sync::mpsc, task};

use crate::{config::CONFIG, pipeline::{Pipeline, Queries}};

#[derive(Default)]
pub struct Media {
    total: usize,
    items: Vec<QString>,
    queries: Queries,
    pipeline: Pipeline
}

fn render(thread: FenluMediaCxxQtThread, items: Vec<QString>) {
    thread.queue(move |mut media| {
        println!("rendering media");
        let amount = items.len();
        media.as_mut().cxx_qt_ffi_rust_mut().items = items;
        media.as_mut().set_total(amount);
    }).expect("could not queue update");
}

impl qobject::FenluMedia {
    pub fn item(&self, index: usize) -> QString {
        match self.items.get(index as usize) {
            Some(media) => media.clone(),
            None => QString::default()
        }
    }

    pub fn open(&self, url: QUrl) {
        open::that_detached(url.to_string()).expect("media should open");
    }

    pub fn set_query(self: Pin<&mut Self>, key: QString, query: QString) {
        *self.cxx_qt_ffi_rust_mut().queries.entry((&key).into()).or_default() = (&query).into();
    }

    pub fn handle_pipeline(self: Pin<&mut Self>) {
        let qthread = self.cxx_qt_ffi_qt_thread();

        let buffer_size = 48;
        let (tx, mut rx) = mpsc::channel(buffer_size);

        task::spawn(async move {
                let mut items = vec![];
                let mut last_update = Instant::now();

                qthread.queue(move |media| {
                    block_on(async {
                        media.pipeline.set_queries(&media.queries).await.expect("could not set queries");
                        media.pipeline.run(buffer_size, tx).await.expect("could not run pipeline");
                    });
                }).expect("could not queue pipeline run");

                while let Some(media) = rx.recv().await {
                    println!("media recieved: {:?}", media.uri.to_string());
                    let media = QString::from(&serde_json::to_string(&media).expect("media should encode to json"));
                    items.push(media);

                    // send items to qt every media_update_interval
                    if last_update.elapsed().as_millis() >= CONFIG.media_update_interval {
                        last_update = Instant::now();
                        render(qthread.clone(), items.clone());
                    }
                }

                render(qthread, items.clone());
            });
    }
}

impl cxx_qt::Initialize for FenluMedia {
    fn initialize(self: Pin<&mut Self>) {
        self.handle_pipeline();
    }
}


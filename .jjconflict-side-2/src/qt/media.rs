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

use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, RwLock},
    time::Instant,
};

use cxx_qt_lib::{QString, QUrl};
use futures::executor::block_on;
use qobject::{FenluMedia, FenluMediaCxxQtThread};
use tokio::{sync::mpsc, task};
use tracing::{debug, info, instrument};

use crate::{config::CONFIG, pipeline::Pipeline};

#[derive(Default)]
pub struct Media {
    total: usize,
    items: Arc<RwLock<Vec<QString>>>,
    queries: HashMap<String, String>,
    pipeline: Arc<Pipeline>,
}

fn render(thread: FenluMediaCxxQtThread, total: usize) {
    thread
        .queue(move |mut media| media.as_mut().set_total(total))
        .expect("could not queue update");
}

impl qobject::FenluMedia {
    pub fn item(&self, index: usize) -> QString {
        let items = self.items.read().unwrap();
        match items.get(index as usize) {
            Some(media) => media.clone(),
            None => QString::default(),
        }
    }

    pub fn open(&self, url: QUrl) {
        open::that_detached(url.to_string()).expect("could not open media");
    }

    pub fn set_query(self: Pin<&mut Self>, key: QString, query: QString) {
        *self
            .cxx_qt_ffi_rust_mut()
            .queries
            .entry((&key).into())
            .or_default() = (&query).into();
    }

    #[instrument(skip(self))]
    pub fn handle_pipeline(self: Pin<&mut Self>) {
        info!("starting pipeline");
        let qthread = self.cxx_qt_ffi_qt_thread();

        let (tx, mut rx) = mpsc::channel(CONFIG.buffer_size);
        let items = self.items.clone();

        let queries = self.queries.clone();
        let pipeline = self.pipeline.clone();

        task::spawn(async move {
            pipeline
                .set_queries(&queries)
                .await
                .expect("could not set queries");

            pipeline
                .run(CONFIG.buffer_size, tx)
                .await
                .expect("could not run pipeline");
            info!("pipeline finished");
        });

        task::spawn(async move {
            let mut last_update = Instant::now();
            loop {
                let mut batch = Vec::with_capacity(CONFIG.buffer_size);
                if rx.recv_many(&mut batch, CONFIG.buffer_size).await == 0 {
                    break;
                }

                debug!(amount = ?batch.len(), "received new batch");

                let mut serialized = batch
                    .into_iter()
                    .map(|media| {
                        QString::from(
                            &serde_json::to_string(&media).expect("media should encode to json"),
                        )
                    })
                    .collect();

                let mut items = items.write().unwrap();
                items.append(&mut serialized);

                let now = Instant::now();
                if now.duration_since(last_update).as_millis() > CONFIG.media_update_interval.into()
                {
                    last_update = now;
                    render(qthread.clone(), items.len());
                }
            }

            let items = items.read().unwrap();
            render(qthread.clone(), items.len());
        });
    }
}

impl cxx_qt::Initialize for FenluMedia {
    fn initialize(mut self: Pin<&mut Self>) {
        let mut pipeline = Pipeline::default();
        block_on(pipeline.populate()).expect("could not populate pipeline");
        self.as_mut().cxx_qt_ffi_rust_mut().pipeline = Arc::new(pipeline);
        self.handle_pipeline();
    }
}

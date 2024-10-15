#[cxx_qt::bridge(cxx_file_stem = "media")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qset.h");
        type QSet_QString = cxx_qt_lib::QSet<QString>;
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
        fn queryable_scripts(self: &FenluMedia) -> QSet_QString;
        #[qinvokable]
        fn set_query(self: &FenluMedia, script: QString, query: QString);
        #[qinvokable]
        fn handle_pipeline(self: Pin<&mut FenluMedia>);
    }

    impl cxx_qt::Threading for FenluMedia {}
    impl cxx_qt::Constructor<()> for FenluMedia {}
}

use std::{
    pin::Pin,
    sync::{Arc, RwLock},
    time::Instant,
};

use cxx_qt_lib::QString;
use futures::executor::block_on;
use qobject::{FenluMedia, FenluMediaCxxQtThread, QSet_QString};
use tokio::{sync::mpsc, task};
use tracing::{debug, info, instrument, Instrument};

use crate::{
    config::CONFIG,
    pipeline::Pipeline,
    protocol::{messages::Request, query},
    utils,
};

#[derive(Default)]
pub struct Media {
    total: usize,
    items: Arc<RwLock<Vec<QString>>>,
    pipeline: Arc<Pipeline>,
}

fn render(thread: FenluMediaCxxQtThread, total: usize) {
    debug!(total = ?total, "rendering media");
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

    pub fn queryable_scripts(&self) -> QSet_QString {
        let mut set = QSet_QString::default();
        self.pipeline
            .scripts
            .iter()
            .filter(|(_, script)| script.capabilities.query.query)
            .map(|(name, _)| QString::from(name))
            .for_each(|name| set.insert(name));
        set
    }

    pub fn set_query(&self, script: QString, query: QString) {
        info!(script = ?script, query = ?query,"setting query");
        block_on(
            self.pipeline
                .scripts
                .get(&script.to_string())
                .expect("script should exist")
                .request(Request {
                    id: utils::generate_id(),
                    method: query::QUERY_METHOD.to_string(),
                    params: serde_json::to_value(query::QueryRequest {
                        query: query.to_string(),
                    })
                    .expect("could not set query"),
                }),
        );
    }

    #[instrument(skip(self), name = "pipeline")]
    pub fn handle_pipeline(self: Pin<&mut Self>) {
        let qthread = self.cxx_qt_ffi_qt_thread();

        let (tx, mut rx) = mpsc::channel(CONFIG.buffer_size);
        let items = self.items.clone();

        let pipeline = self.pipeline.clone();

        task::spawn(async move {
            pipeline
                .start(CONFIG.buffer_size, tx)
                .await
                .expect("could not run pipeline");
        });

        task::spawn(
            async move {
                let mut last_update = Instant::now();
                loop {
                    let mut batch = Vec::with_capacity(CONFIG.buffer_size);
                    if rx.recv_many(&mut batch, CONFIG.buffer_size).await == 0 {
                        break;
                    }

                    info!(amount = ?batch.len(), "received new batch");

                    let mut serialized = batch
                        .into_iter()
                        .map(|media| {
                            QString::from(
                                &serde_json::to_string(&media)
                                    .expect("media should encode to json"),
                            )
                        })
                        .collect();

                    let mut items = items.write().unwrap();
                    items.append(&mut serialized);

                    let now = Instant::now();
                    if now.duration_since(last_update).as_millis()
                        > CONFIG.media_update_interval.into()
                    {
                        last_update = now;
                        render(qthread.clone(), items.len());
                    }
                }

                let items = items.read().unwrap();
                render(qthread.clone(), items.len());
            }
            .in_current_span(),
        );
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

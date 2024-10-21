#[cxx_qt::bridge(cxx_file_stem = "pipeline")]
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
        #[qproperty(bool, running)]
        type FenluPipeline = super::Media;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn items(self: &FenluPipeline, from: usize) -> QSet_QString;
        #[qinvokable]
        fn queryable_scripts(self: &FenluPipeline) -> QSet_QString;
        #[qinvokable]
        fn set_query(self: &FenluPipeline, script: QString, query: QString);
        #[qinvokable]
        fn run_pipeline(self: Pin<&mut FenluPipeline>);
    }

    impl cxx_qt::Threading for FenluPipeline {}
    impl cxx_qt::Constructor<()> for FenluPipeline {}
}

use std::{
    pin::Pin,
    sync::{Arc, RwLock},
    time::Instant,
};

use cxx_qt_lib::QString;
use futures::executor::block_on;
use qobject::{FenluPipeline, FenluPipelineCxxQtThread, QSet_QString};
use tokio::{sync::mpsc, task};
use tracing::{debug, info, instrument, Instrument};

use crate::{
    config::CONFIG,
    pipeline::{Pipeline, GLOBAL_PIPELINE},
    protocol::{messages::Request, query},
    utils,
};

#[derive(Default)]
pub struct Media {
    total: usize,
    running: bool,
    items: Arc<RwLock<Vec<QString>>>,
    pipeline: Arc<Pipeline>,
}

fn render(thread: &FenluPipelineCxxQtThread, total: usize) {
    debug!(total = ?total, "rendering media");
    thread
        .queue(move |mut media| media.as_mut().set_total(total))
        .expect("could not queue update");
}

fn set_running(thread: &FenluPipelineCxxQtThread, running: bool) {
    if running {
        info!("pipeline started");
    } else {
        info!("pipeline finished");
    }

    thread
        .queue(move |mut media| media.as_mut().set_running(running))
        .expect("could not queue update");
}

impl qobject::FenluPipeline {
    pub fn items(&self, at: usize) -> QSet_QString {
        let mut set = QSet_QString::default();
        let items = self.items.read().unwrap();

        for item in &items[at..items.len()] {
            set.insert(item.clone());
        }

        set
    }

    pub fn queryable_scripts(&self) -> QSet_QString {
        let mut set = QSet_QString::default();
        self.pipeline
            .scripts
            .iter()
            .filter(|(_, script)| script.capabilities.query.set)
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
                    method: query::QUERY_SET_METHOD.to_string(),
                    params: serde_json::to_value(query::QueryRequest {
                        query: query.to_string(),
                    })
                    .expect("could not set query"),
                }),
        );
    }

    #[instrument(skip(self), name = "pipeline")]
    pub fn run_pipeline(self: Pin<&mut Self>) {
        let qthread = self.cxx_qt_ffi_qt_thread();

        let (tx, mut rx) = mpsc::channel(CONFIG.buffer_size);
        let items = self.items.clone();

        {
            let qthread = qthread.clone();
            task::spawn(async move {
                // @TODO probably should use RAII for set_running
                set_running(&qthread, true);
                GLOBAL_PIPELINE
                    .start(CONFIG.buffer_size, tx)
                    .await
                    .expect("could not run pipeline");
                set_running(&qthread, false);
            });
        };

        task::spawn(
            async move {
                let mut last_update = Instant::now();

                loop {
                    // take in new batches
                    let mut batch = Vec::with_capacity(CONFIG.buffer_size);
                    if rx.recv_many(&mut batch, CONFIG.buffer_size).await == 0 {
                        break;
                    }
                    let amount = batch.len();

                    // serialize new batch
                    let mut serialized = batch
                        .into_iter()
                        .map(|media| {
                            // @PERF qstring::from is extremely memory intensive (~15% of total memory usage)
                            QString::from(
                                &serde_json::to_string(&media)
                                .expect("media should encode to json"),
                            )
                        })
                    .collect();

                    let mut items = items.write().unwrap();
                    items.append(&mut serialized);

                    // tell frontend to update if media_update_interval has passed
                    let now = Instant::now();
                    if now.duration_since(last_update).as_millis()
                        > CONFIG.media_update_interval.into()
                    {
                        last_update = now;
                        render(&qthread, items.len());
                    }

                    info!(amount = ?amount, "processed batch");
                }

                let items = items.read().unwrap();
                render(&qthread, items.len());
            }
        .in_current_span(),
        );
    }
}

impl cxx_qt::Initialize for FenluPipeline {
    fn initialize(self: Pin<&mut Self>) {
        self.run_pipeline();
    }
}

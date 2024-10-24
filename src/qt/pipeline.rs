// @FIX this entire file is super fucked and needs to be redone who thought of this stupid type
// sharing system what is even going on i dont get it please someone save me
// (option 1. figure out a better data sharing system)
// (option 2. remove json.parse's and replace them with cxx_qt types)

#[cxx_qt::bridge(cxx_file_stem = "pipeline")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qlist.h");
        type QList_QString = cxx_qt_lib::QList<QString>;
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
        fn items(self: &FenluPipeline, from: usize) -> QList_QString;
        #[qinvokable]
        fn queryable_scripts(self: &FenluPipeline) -> QList_QString;
        #[qinvokable]
        fn set_query(self: &FenluPipeline, script: QString, query: QString);
        #[qinvokable]
        fn actions_for_scripts(self: &FenluPipeline, history: QList_QString) -> QList_QString;
        #[qinvokable]
        fn run_action(self: Pin<&mut FenluPipeline>, media: QString, action: QString);
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
use qobject::{FenluPipeline, FenluPipelineCxxQtThread, QList_QString};
use tokio::{sync::mpsc, task::{self}};
use tracing::{debug, info, instrument, Instrument};

use crate::{
    config::CONFIG, pipeline::GLOBAL_PIPELINE, protocol::{actions, messages::Request, query}, utils::{self, generate_id}
};

#[derive(Default)]
pub struct Media {
    total: usize,
    running: bool,
    items: Arc<RwLock<Vec<QString>>>,
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
    pub fn items(&self, at: usize) -> QList_QString {
        let items = self.items.read().unwrap();
        let (_, split) = items.split_at(at);
        QList_QString::from(split)
    }

    pub fn queryable_scripts(&self) -> QList_QString {
        QList_QString::from(GLOBAL_PIPELINE
            .scripts
            .iter()
            .filter(|(_, script)| script.capabilities.query.set)
            .map(|(name, _)| QString::from(name))
            .collect::<Vec<QString>>())
    }

    pub fn set_query(&self, script: QString, query: QString) {
        info!(script = ?script, query = ?query,"setting query");
        task::spawn(async move {
            GLOBAL_PIPELINE
                .scripts
                .get(&script.to_string())
                .expect("script should exist")
                .request(Request {
                    id: utils::generate_id(),
                    method: query::QUERY_SET_METHOD.to_string(),
                    params: serde_json::to_value(query::QueryRequest {
                        query: query.to_string(),
                    })
                    .expect("could not encode query as json"),
                }).await.result().expect("could not set query");
        });
    }

    pub fn actions_for_scripts(&self, history: QList_QString) -> QList_QString {
        QList_QString::from(history
            .iter()
            .filter_map(|name| GLOBAL_PIPELINE.scripts.get_key_value(&name.to_string()))
            .map(|(name, script)| 
                script
                .capabilities
                .actions
                .iter().map(move |action| (name, action))
            )
            .flatten()
            .map(|(name, action)| QString::from(&format!("{}/{}", name, action)))
            .collect::<Vec<QString>>())
    }

    pub fn run_action(self: Pin<&mut FenluPipeline>, media: QString, action: QString) {
        let action = action.to_string();
        let media = media.to_string();

        let split = action.split_once('/');
        let (script_name, action_name) = split.expect("action was not formatted as \"script/name\"");
        let action_name = action_name.to_string();

        let script = GLOBAL_PIPELINE.scripts.get(script_name).expect("no matching scripts for action");

        task::spawn(async move {
            script.request(Request {
                id: generate_id(),
                method: (actions::ACTION_BASE_METHOD.to_string() + &action_name).to_string(),
                params: serde_json::from_str(media.as_str()).unwrap()
            })
            .await.result().expect("could not run action");
        });
    }

    #[instrument(skip(self), name = "pipeline")]
    pub fn run_pipeline(self: Pin<&mut Self>) {
        let qthread = self.cxx_qt_ffi_qt_thread();

        let (tx, mut rx) = mpsc::channel(CONFIG.buffer_size);
        let items = self.items.clone();

        // clear previous run
        {
            self.set_total(0);
            let mut items = items.write().unwrap();
            items.clear();
        }

        {
            let qthread = qthread.clone();
            task::spawn(async move {
                // @FIX probably should use RAII for set_running
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

                    debug!(amount = ?amount, "processed batch");
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

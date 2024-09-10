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
        #[qproperty(usize, total)]
        type FenluMedia = super::Media;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn item(self: &FenluMedia, index: usize) -> QUrl;
        #[qinvokable]
        fn open(self: &FenluMedia, url: QUrl);
        #[qinvokable]
        fn set_query(self: Pin<&mut FenluMedia>, key: QString, query: QString);
        #[qinvokable]
        fn run_pipeline(self: Pin<&mut FenluMedia>);
    }

    impl cxx_qt::Threading for FenluMedia {}
    impl cxx_qt::Constructor<()> for FenluMedia {}
}

use std::{collections::HashMap, pin::Pin, thread, time::Instant};
use cxx_qt_lib::{QString, QUrl};
use qobject::{FenluMedia, FenluMediaCxxQtThread};
use tokio::runtime::Runtime;

use crate::{config::CONFIG, pipeline::{run_pipeline, PipelineOpts}};

#[derive(Default)]
pub struct Media {
    total: usize,
    items: Vec<QUrl>,
    queries: HashMap<String, String>
}

impl qobject::FenluMedia {
    pub fn item(&self, index: usize) -> QUrl {
        match self.items.get(index as usize) {
            Some(url) => url.clone(),
            None => QUrl::default()
        }
    }

    pub fn open(&self, url: QUrl) {
        open::that_detached(url.to_string()).expect("media should open");
    }

    pub fn set_query(self: Pin<&mut Self>, key: QString, query: QString) {
        *self.cxx_qt_ffi_rust_mut().queries.entry((&key).into()).or_default() = (&query).into();
    }

    pub fn run_pipeline(self: Pin<&mut Self>) {
        let qthread = self.cxx_qt_ffi_qt_thread();
        let queries = self.queries.clone();

        thread::spawn(move || {
            let rt = Runtime::new().expect("runtime should be created");
            rt.block_on(handle_media(qthread, queries));
        });
    }
}

fn render(thread: FenluMediaCxxQtThread, items: Vec<QUrl>) {
        thread.queue(move |mut media| {
            println!("rendering media");
            let amount = items.len();
            media.as_mut().cxx_qt_ffi_rust_mut().items = items;
            media.as_mut().set_total(amount);
        }).expect("should be able to queue update");
}

async fn handle_media(thread: FenluMediaCxxQtThread, queries: HashMap<String, String>) {
    let mut items = vec![];
    let mut last_update = Instant::now();

    for media in run_pipeline(PipelineOpts {
        save: false,
        load: false,
        queries
    })
    .await
    .expect("pipeline should succeed")
    .into_iter() {
        println!("media recieved: {:?}", media.uri.to_string());
        let url = QUrl::from(&media.uri.to_string());

        items.push(url);

        // send items to qt every media_update_interval
        if last_update.elapsed().as_millis() >= CONFIG.media_update_interval {
            last_update = Instant::now();
            render(thread.clone(), items.clone());
        }
    }

    render(thread, items.clone());
}

impl cxx_qt::Initialize for FenluMedia {
    fn initialize(self: Pin<&mut Self>) {
        self.run_pipeline();
    }
}


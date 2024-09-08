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
    }

    impl cxx_qt::Threading for FenluMedia {}
    impl cxx_qt::Constructor<()> for FenluMedia {}
}

use std::{pin::Pin, thread, time::Instant};
use cxx_qt_lib::QUrl;
use qobject::FenluMediaCxxQtThread;
use tokio::runtime::Runtime;

use crate::pipeline::run_pipeline;

#[derive(Default)]
pub struct Media {
    total: usize,
    items: Vec<QUrl>,
}

impl qobject::FenluMedia {
    pub fn item(&self, index: usize) -> QUrl {
        match self.items.get(index as usize) {
            Some(url) => url.clone(),
            None => QUrl::default()
        }
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

async fn handle_media(thread: FenluMediaCxxQtThread) {
    let mut items = vec![];
    let mut last_update = Instant::now();

    for metadata in run_pipeline(false, false).await.expect("pipeline should succeed").into_iter(){
        println!("media recieved: {:?}", metadata.uri.to_string());
        let url = QUrl::from(&metadata.uri.to_string());

        items.push(url);

        // send items to qt every 500ms
        if last_update.elapsed().as_millis() >= 500 {
            last_update = Instant::now();
            render(thread.clone(), items.clone());
        }
    }

    render(thread, items.clone());
}

impl cxx_qt::Initialize for qobject::FenluMedia {
    fn initialize(self: Pin<&mut Self>) {
        // read items from stdin and send them to qt
        let thread = self.cxx_qt_ffi_qt_thread();
        thread::spawn(move || {
            let rt = Runtime::new().expect("runtime should be created");
            rt.block_on(handle_media(thread));
        });
    }
}


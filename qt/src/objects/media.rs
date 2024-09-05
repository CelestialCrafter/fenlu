#[cxx_qt::bridge]
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

use std::{io::{BufRead, BufReader}, pin::Pin, process::{Command, Stdio}, thread, time::Instant};
use fenlu_cli::metadata::Metadata;
use cxx_qt_lib::QUrl;

use crate::config::CONFIG;

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

impl cxx_qt::Initialize for qobject::FenluMedia {
    fn initialize(self: Pin<&mut Self>) {
        // @TODO make this not reliant on fenlu-cli, and able to use any command via options
        let mut cmd = Command::new("sh");
        let cmd = cmd.args(vec!["-c", &CONFIG.generation_script]).stdout(Stdio::piped());
        let mut child = cmd.spawn().expect("program should execute");
        let stdout = child.stdout.take().expect("could not take stdout");

        // read items from stdin and send them to qt
        let thread = self.cxx_qt_ffi_qt_thread();
        thread::spawn(move || {
            let mut items = vec![];
            let mut last_update = Instant::now();

            let render = |items: Vec<QUrl>| {
                    thread.queue(move |mut media| {
                        let amount = items.len();
                        media.as_mut().cxx_qt_ffi_rust_mut().items = items;
                        media.as_mut().set_total(amount);
                    }).expect("should be able to queue update");
            };

            for line in BufReader::new(stdout).lines() {
                let line = line.expect("should be able to read line");
                let line = line.as_str();
                let metadata: Metadata = serde_json::from_str(line).expect("media should parse into Metadata");
                let url = QUrl::from(&metadata.uri.to_string());

                println!("parsed metadata: {}", metadata.uri);
                items.push(url);

                // send items to qt every 500ms
                if last_update.elapsed().as_millis() >= 500 {
                    last_update = Instant::now();
                    render(items.clone());
                }
            }

            render(items);
        });
    }
}


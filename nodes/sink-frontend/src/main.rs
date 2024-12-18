pub mod protocol;
pub mod qt;
pub mod server;
pub mod media;
pub mod initialize;
pub mod config;
pub mod sink;

use std::{io::{self, Read}, thread, time};

use config::CONFIG;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

pub fn wait_for_config() {
        while let None = CONFIG.get() {
            thread::sleep(time::Duration::from_millis(50));
        }
}

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/sinkfrontend/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }

    let _ = io::stdin().read_to_end(&mut vec![]).expect("could not read to end of stdin");
}

pub mod protocol;
pub mod qt;
pub mod server;
pub mod media;
pub mod initialize;
pub mod config;
pub mod sink;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/sinkfrontend/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}

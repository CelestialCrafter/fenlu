pub mod qt;
pub mod config;
pub mod pipeline;
pub mod metadata;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/fenlu/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}

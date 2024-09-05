pub mod objects;
pub mod config;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/io.github/CelestialCrafter/fenlu/src/qml/media.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}

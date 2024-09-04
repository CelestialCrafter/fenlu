use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "com.github.CelestialCrafter.fenlu",
            rust_files: &["src/objects/media.rs"],
            qml_files: &["src/qml/media.qml", "src/qml/CustomScroll.qml"],
            ..Default::default()
        })
        .build();
}

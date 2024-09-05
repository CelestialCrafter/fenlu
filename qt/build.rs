use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "io.github.CelestialCrafter.fenlu",
            rust_files: &["src/objects/media.rs", "src/objects/toml.rs"],
            qml_files: &["src/qml/media.qml", "src/qml/CustomScrollGridView.qml"],
            ..Default::default()
        })
        .build();
}

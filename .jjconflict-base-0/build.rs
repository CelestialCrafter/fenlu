use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "fenlu",
            rust_files: &["src/qt/media.rs"],
            qml_files: &["qml/main.qml", "qml/CustomScrollGridView.qml"],
            ..Default::default()
        })
        .build();
}


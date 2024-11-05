use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "sinkfrontend",
            rust_files: &["src/qt/media.rs", "src/qt/actions.rs"],
            qml_files: &["qml/main.qml", "qml/Media.qml", "qml/MediaBackground.qml", "qml/CustomScrollGridView.qml", "qml/ActionsContextMenu.qml"],
            ..Default::default()
        })
        .build();
}

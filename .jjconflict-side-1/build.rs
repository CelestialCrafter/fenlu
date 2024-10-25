use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "fenlu",
            rust_files: &["src/qt/pipeline.rs"],
            qml_files: &[
                "qml/main.qml",
                "qml/CustomScrollGridView.qml",
                "qml/ActionsContextMenu.qml",
                "qml/Topbar.qml",
                "qml/MediaList.qml",
                "qml/MediaDetails.qml",
                "qml/MediaBackground.qml",
                "qml/Media.qml",
            ],
            ..Default::default()
        })
        .build();
}

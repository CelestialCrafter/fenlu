import QtQuick 2.12
import QtQuick.Controls 2.12
import fenlu

Menu {
    required property var selected

    MenuItem {
        text: "Open"
        onTriggered: {
            Qt.openUrlExternally(selected.uri);
        }
    }

    Repeater {
        model: FenluPipeline.actionsForScripts(selected.history)
        MenuItem {
            required property string modelData
            text: modelData
            onTriggered: FenluPipeline.runAction(JSON.stringify(selected), modelData)
        }
    }
}

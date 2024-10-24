import QtQuick
import QtQuick.Controls
import fenlu

Menu {
    property var current

    MenuItem {
        text: "Open"
        onTriggered: {
            Qt.openUrlExternally(current.uri);
        }
    }

    Repeater {
        id: repeater
        model: []
        MenuItem {
            required property string modelData
            text: modelData
            onTriggered: FenluPipeline.runAction(JSON.stringify(current), modelData)
        }
    }

    onAboutToShow: {
        repeater.model = FenluPipeline.actionsForScripts(Object.keys(current.history));
    }
}

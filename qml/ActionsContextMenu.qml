import QtQuick 2.12
import QtQuick.Controls 2.12
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

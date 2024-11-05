import QtQuick
import QtQuick.Controls
import sinkfrontend

Menu {
    property var current

    Repeater {
        id: repeater
        model: Object.keys(Actions.actions)
        MenuItem {
            required property string modelData
            text: modelData
            onTriggered: Actions.execute(current, modelData)
        }
    }
}

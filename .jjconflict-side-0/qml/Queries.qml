import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import fenlu

Item {
    Rectangle {
        anchors.fill: parent
    }

    Column {
        id: column
        anchors.fill: parent

        Repeater {
            model: FenluPipeline.queryableScripts()
            Item {
                width: parent.width
                height: 15
                required property string modelData
                Text {
                    id: label
                    text: modelData + ": "
                }

                TextInput {
                    id: input
                    anchors.left: label.right
                    width: parent.width * 0.6

                    onAccepted: FenluPipeline.setQuery(modelData, input.text)
                }
            }
        }
    }

    Button {
        anchors.right: parent.right
        enabled: !FenluPipeline.running
        id: rerun
        text: "Re-Run Pipeline"
        onClicked: FenluPipeline.runPipeline()

        Text {
            anchors.top: rerun.bottom
            width: parent.width
            horizontalAlignment: Text.AlignHCenter
            text: "Total: " + FenluPipeline.total
        }
    }
}

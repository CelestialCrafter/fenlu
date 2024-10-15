import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    function runPipeline() {
        FenluMedia.setTotal(0);
        mediaModel.clear();
        FenluMedia.handlePipeline();
    }

    Rectangle {
        anchors.fill: parent
    }

    Column {
        id: column
        anchors.fill: parent

        Repeater {
            model: FenluMedia.queryableScripts()
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

                    onAccepted: FenluMedia.setQuery(modelData, input.text)
                }
            }
        }
    }

    Button {
        anchors.right: parent.right
        id: rerun
        text: "Re-Run Pipeline"
        onClicked: runPipeline()
    }
}

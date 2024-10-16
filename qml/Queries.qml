import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    function runPipeline() {
        FenluPipeline.setTotal(0);
        mediaModel.clear();
        FenluPipeline.handlePipeline();
    }

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
        id: rerun
        text: "Re-Run Pipeline"
        onClicked: runPipeline()
    }
}

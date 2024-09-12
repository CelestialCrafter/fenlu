import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    required property FenluMedia media

    function runPipeline() {
        mediaModel.clear();
        media.runPipeline();
    }

    FenluScripts {
        id: scripts
    }

    Rectangle {
        anchors.fill: parent
    }

    Column {
        id: column
        anchors.fill: parent

        Repeater {
            model: scripts.total
            Item {
                width: parent.width
                height: 15
                property string script: scripts.item(index)
                Text {
                    id: label
                    text: script + ": "
                }

                TextInput {
                    id: input
                    anchors.left: label.right
                    width: parent.width * 0.6

                    onTextEdited: media.setQuery(script, input.text);
                    onAccepted: runPipeline()
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

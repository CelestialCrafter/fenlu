import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    required property FenluMedia media

    FenluScripts {
        id: scripts
    }

    Column {
        id: column
        anchors.fill: parent

        Repeater {
            model: scripts.filtersTotal
            Item {
                width: parent.width
                height: 10
                property string script: scripts.getFilter(index)
                Text {
                    id: label
                    text: script + ": "
                }

                TextInput {
                    id: input
                    anchors.left: label.right
                    width: parent.width * 0.6

                    onTextEdited: media.setQuery(script, input.text);
                }
            }
        }
    }

    Button {
        anchors.right: parent.right
        text: "Re-Run Pipeline"
        onClicked: {
            mediaModel.clear();
            media.runPipeline();
        }
    }
}

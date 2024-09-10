import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    required property FenluMedia media
    width: parent.width

    FenluScripts {
        id: scripts
    }

    Column {
        width: parent.width
        Repeater {
            model: scripts.filtersTotal
            Item {
                width: parent.width
                property string script: scripts.getFilter(index)

                Text {
                    anchors.left: parent.left
                    id: label
                    text: script + ": "
                }

                TextInput {
                    anchors.left: label.right
                    text: ""
                    width: parent.width * 0.2
                    id: input

                    onTextEdited: media.setQuery(script, input.text);
                }

                Button {
                    anchors.left:input.right
                    text: "Re-Run Pipeline"
                    onClicked: {
                        mediaModel.clear();
                        media.runPipeline();
                    }
                }
            }
        }
    }
}

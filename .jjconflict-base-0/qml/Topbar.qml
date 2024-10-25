import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import fenlu

Item {
    height: pane.height + separator.height

    ListModel {
        id: queryableModel
        Component.onCompleted: {
            for (const script of FenluPipeline.queryableScripts()) {
                append({ script });
            }
        }
    }

    Pane {
        id: pane
        padding: 4
        width: parent.width
        height: controls.height + implicitHeight

        background: Rectangle {
            color: "lightgray"
        }

        ListView {
            height: parent.height
            width: parent.width * 0.4
            model: queryableModel
            activeFocusOnTab: true
            interactive: false
            keyNavigationEnabled: true

            delegate: FocusScope {
                visible: ListView.isCurrentItem
                anchors.fill: parent

                Text {
                    id: label
                    text: script + ": "
                    verticalAlignment: Qt.AlignVCenter
                }

                TextInput {
                    id: textInput
                    focus: true
                    clip: true
                    anchors.left: label.right
                    width: parent.width - label.width

                    onAccepted: FenluPipeline.setQuery(script, text)
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            anchors.right: parent.right
            id: controls

            RowLayout {
                Layout.alignment: Qt.AlignRight
                spacing: 4

                RoundButton {
                    radius: 4
                    text: FenluPipeline.total
                    focusPolicy: Qt.NoFocus
                    down: false
                }

                RoundButton {
                    enabled: !FenluPipeline.running
                    text: "Re-Run Pipeline"
                    radius: 4
                    onClicked: FenluPipeline.runPipeline()
                    Keys.onReturnPressed: FenluPipeline.runPipeline()
                }
            }
        }
    }

    Rectangle {
        id: separator
        anchors.bottom: parent.bottom
        width: parent.width
        height: 1
        color: "grey"
    }
}

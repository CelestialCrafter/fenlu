import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import fenlu

Item {
    Rectangle {
        id: rect
        anchors.fill: parent
    }

    ListModel {
        id: queryableModel
        Component.onCompleted: {
            for (const script of FenluPipeline.queryableScripts()) {
                this.append({ script });
            }
        }
    }

    ListView {
        activeFocusOnTab: true
        interactive: false
        keyNavigationEnabled: true
        anchors.fill: parent
        model: queryableModel
        delegate: FocusScope {
            width: parent.width
            height: 15

            Text {
                id: label
                text: script + ": "
            }

            TextInput {
                focus: true
                clip: true
                anchors.left: label.right
                width: parent.width * 0.6

                onAccepted: FenluPipeline.setQuery(script, this.text)
            }
        }
    }

    Button {
        anchors.right: parent.right
        enabled: !FenluPipeline.running
        text: "Re-Run Pipeline"
        onClicked: FenluPipeline.runPipeline()

        Text {
            anchors.top: parent.bottom
            width: parent.width
            horizontalAlignment: Text.AlignHCenter
            text: "Total: " + FenluPipeline.total
        }
    }
}

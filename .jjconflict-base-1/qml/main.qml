import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import fenlu

ApplicationWindow {
    height: 480
    title: qsTr("Fenlu")
    visible: true
    width: 640
    id: window

    ListModel {
        id: mediaModel
    }

    ColumnLayout {
        anchors.fill: parent
        Queries {
            id: queries
            z: 5
            Layout.preferredHeight: parent.height * 0.1
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
        }

        Text {
            z: 1
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
            text: "no media loaded"
            visible: mediaModel.count === 0
        }

        MediaList {
            z: 1
            focus: true
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignBottom
            visible: mediaModel.count > 0
        }
    }

    onActiveFocusItemChanged:  console.log("main: focus now on: ", activeFocusItem)
}

import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import fenlu

ApplicationWindow {
    height: 480
    title: qsTr("Fenlu")
    visible: true
    width: 640

    ListModel {
        id: mediaModel
    }

    palette.alternateBase: FenluColors.surface
    palette.base: FenluColors.base
    palette.window: FenluColors.base
    palette.mid: FenluColors.highlightMedium
    palette.shadow: FenluColors.highlightMedium
    palette.dark: FenluColors.highlightHigh
    palette.highlight: FenluColors.accent
    palette.text: FenluColors.text


    ColumnLayout {
        anchors.fill: parent
        Topbar {
            id: queries
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
        }

        Text {
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
            text: "no media loaded"
            visible: mediaModel.count === 0
        }

        MediaList {
            focus: true
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignBottom
            visible: mediaModel.count > 0
        }
    }
}

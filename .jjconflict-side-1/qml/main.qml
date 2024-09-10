import QtQuick.Window 2.12
import QtQuick.Layouts
import fenlu 1.0

Window {
    height: 480
    title: qsTr("Fenlu")
    visible: true
    width: 640
    id: window

    FenluMedia {
        id: fenluMedia
    }

    ListModel {
        id: mediaModel
    }

    ColumnLayout {
        anchors.fill: parent
        Queries {
            id: queries
            Layout.preferredHeight: parent.height * 0.2
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
            media: fenluMedia
        }

        Media {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignBottom
            media: fenluMedia
        }
    }
}

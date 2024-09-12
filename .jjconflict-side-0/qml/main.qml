import QtQuick.Window 2.12
import QtQuick.Layouts 2.12
import QtQuick.Controls 2.12
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
            z: 5
            Layout.preferredHeight: parent.height * 0.1
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
            media: fenluMedia
        }

        Label {
            z: 1
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
            text: "no media loaded"
            visible: mediaModel.count === 0
        }

        Media {
            z: 1
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignBottom
            media: fenluMedia
            visible: mediaModel.count > 0
        }
    }
}

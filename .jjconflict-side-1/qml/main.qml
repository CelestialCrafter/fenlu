import QtQuick.Window 2.12
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

    Queries {
        id: queries
        height: parent.height * 0.3
        anchors.top: parent.top
        media: fenluMedia
    }
    Media {
        height: parent.height * 0.7
        width: parent.width
        anchors.top: queries.bottom
        media: fenluMedia
    }
}

import QtQuick 2.12
import QtQuick.Controls 2.12
import fenlu

Image {
    required property var media

    asynchronous: true
    cache: false
    fillMode: Image.PreserveAspectCrop
    source: media.uri
    sourceSize: Qt.size(media.width || undefined, media.height || undefined)

    Label {
        horizontalAlignment: Text.AlignHCenter
        anchors.top: parent.top
        width: parent.width
        topPadding: 5
        bottomPadding: 5
        fontSizeMode: Text.Fit
        elide: Text.ElideRight

        text: media.title
        color: "white"

        background: Rectangle {
            opacity: 0.7
            color: "black"
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: {
            details.current = media;
            details.open();
        }
        onDoubleClicked: FenluMedia.open(media.uri);
    }
}


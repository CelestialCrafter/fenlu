import QtQuick 2.12
import QtQuick.Controls 2.12
import fenlu

Image {
    required property var media

    asynchronous: true
    cache: false
    fillMode: Image.PreserveAspectCrop
    source: media.uri
    sourceSize: media.type === "Image" ? Qt.size(media.width, media.height) : null

    Label {
        horizontalAlignment: Text.AlignHCenter
        anchors.top: parent.top
        width: parent.width
        topPadding: 5
        bottomPadding: 5
        fontSizeMode: Text.Fit
        elide: Text.ElideRight
        wrapMode: Text.Wrap

        text: media.title
        color: "white"

        background: Rectangle {
            opacity: 0.7
            color: "black"
        }
    }

    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: event => {
            switch (event.button) {
                case Qt.LeftButton:
                    mediaDetails.current = media;
                    mediaDetails.open();
                    break;
                case Qt.RightButton:
                    contextMenu.current = media;
                    contextMenu.popup();
                    break;
            }
        }
    }
}


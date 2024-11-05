import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Effects
import sinkfrontend

Pane {
    required property var media
    property bool focusEnabled: true
    property bool focused: false

    padding: 10
    background: MediaBackground {
        focused: mouseArea.containsMouse || parent.focused
        minor: !focused || mouseArea.containsMouse
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Image {
            id: picture
            Layout.fillWidth: true
            Layout.preferredHeight: parent.height * 0.9

            asynchronous: true
            cache: false
            mipmap: true
            source: media.url
            fillMode: Image.PreserveAspectFit
            sourceSize: media.type === "image" ? Qt.size(media.width, media.height) : null

            layer.enabled: true
            layer.effect: MultiEffect {
                source: picture
                shadowEnabled: true
                shadowColor: palette.shadow
                shadowBlur: 0
                shadowVerticalOffset: 3
                shadowHorizontalOffset: 3
            }
        }

        Text {
            Layout.fillHeight: true
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter

            maximumLineCount: 1
            elide: Text.ElideRight
            text: media.title
        }
    }


    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: focusEnabled
        acceptedButtons: Qt.RightButton
        onClicked: event => (contextMenu.current = media) && contextMenu.popup();
    }
}

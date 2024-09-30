import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import fenlu 1.0

Item {
    Connections {
        target: FenluMedia
        property var previousTotal: 0

        function onTotalChanged() {
            for (let i = previousTotal; i < FenluMedia.total; i++) {
                mediaModel.append({ media: JSON.parse(FenluMedia.item(i)) });
            }

            previousTotal = FenluMedia.total;
        }
    }

    Popup {
        property var media

        id: details
        modal: true
        focus: true
        anchors.centerIn: Overlay.overlay
        width: window.width * 0.9
        height: window.height * 0.9

        RowLayout {
            anchors.fill: parent
            Image {
                id: image
                Layout.preferredWidth: details.width * 0.4
                Layout.fillHeight: true
                fillMode: Image.PreserveAspectFit
                cache: false

                Label {
                    id: title
                    horizontalAlignment: Text.AlignHCenter
                    anchors.top: parent.top
                    // position above image
                    anchors.topMargin: (image.height - image.paintedHeight) / 2 - title.height
                    width: parent.width
                }
            }

            ColumnLayout {
                Layout.fillHeight: true
                Layout.fillWidth: true
                ListView {
                    anchors.fill: parent
                    id: history
                    model: ListModel {}
                    delegate: Text {
                        required property string location
                        text: location
                    }
                }
            }
        }

        onAboutToShow: {
            title.text = media.title;
            image.source = media.uri;
            image.sourceSize = Qt.size(media.width || undefined, media.height || undefined);
            history.model.clear();
            for (let i = 0; i < media.history.length; i++) {
                history.model.append({ location: media.history[i] });
            }
        }

    }

    CustomScrollGridView {
        property var columns: 6

        anchors.fill: parent
        id: grid

        cellWidth: grid.width / columns
        cellHeight: grid.cellWidth
        model: mediaModel
        cacheBuffer: grid.cellHeight * columns

        delegate: Image {
            required property var media

            width: grid.cellWidth
            height: grid.cellHeight
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
                    details.media = media;
                    details.open();
                }
                onDoubleClicked: FenluMedia.open(media.uri);
            }
        }
    }
}

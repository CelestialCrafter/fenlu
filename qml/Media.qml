import QtQuick.Controls 2.12
import QtQuick 2.12
import fenlu 1.0

Item {
    required property FenluMedia media

    Connections {
        target: media
        property var previousTotal: 0

        function onTotalChanged() {
            for (let i = previousTotal; i < media.total; i++) {
                mediaModel.append({ url: media.item(i) });
            }

            previousTotal = media.total;
        }
    }

    CustomScrollGridView {
        property var columns: 5

        anchors.fill: parent
        id: grid

        cellWidth: grid.width / columns
        cellHeight: grid.cellWidth
        model: mediaModel
        cacheBuffer: grid.cellHeight * columns

        delegate: Image {
            required property url url
            width: grid.cellWidth
            height: grid.cellHeight
            asynchronous: true
            cache: false
            fillMode: Image.PreserveAspectCrop
            source: url
        }
    }
}

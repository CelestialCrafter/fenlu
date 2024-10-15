import QtQuick 2.12
import QtQuick.Controls 2.12
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

    MediaDetails {
        id: details
    }

    CustomScrollGridView {
        property var columns: 6

        anchors.fill: parent
        id: grid

        cellWidth: grid.width / columns
        cellHeight: grid.cellWidth
        model: mediaModel
        cacheBuffer: grid.cellHeight * columns

        delegate: Media {
            width: grid.cellWidth
            height: grid.cellHeight
        }
    }
}

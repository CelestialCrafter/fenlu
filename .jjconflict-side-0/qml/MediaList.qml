import QtQuick
import QtQuick.Controls
import fenlu

Item {
    Connections {
        target: FenluPipeline
        property var previousTotal: 0

        function onTotalChanged() {
            if (FenluPipeline.total == 0) {
                mediaModel.clear();
                previousTotal = 0;
                return;
            }

            for (const item of FenluPipeline.items(previousTotal)) {
                // @PERF json.parse is extremelyy memory intensive (~25% of total memory usage)
                mediaModel.append({ media: JSON.parse(item) });
            }

            previousTotal = FenluPipeline.total;
        }
    }

    MediaDetails {
        id: mediaDetails
    }

    ActionsContextMenu {
        id: contextMenu
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

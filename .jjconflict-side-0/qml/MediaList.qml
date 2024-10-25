import QtQuick
import QtQuick.Controls
import fenlu
import fenlu

Pane {
    focus: true
    horizontalPadding: 48

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
        property int size: 296
        property int spacing: 4

        anchors.fill: parent
        id: grid
        activeFocusOnTab: true
        focus: true
        clip: true
        cacheBuffer: size
        cellWidth: size
        cellHeight: size
        model: mediaModel

        delegate: Media {
            width: grid.size - grid.spacing
            height: grid.size - grid.spacing
            backgroundFocused: GridView.isCurrentItem
            minorBackground: !GridView.isCurrentItem
        }

        highlight: Item {}
    }
}

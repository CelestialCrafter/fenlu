import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import sinkfrontend

ApplicationWindow {
    height: 480
    width: 640
    title: qsTr("sink-frontend")
    visible: true

    ListModel {
        id: model
    }

    ActionsContextMenu {
        id: contextMenu
    }

    MediaDetails {
        id: mediaDetails
    }

    Pane {
        padding: 10
        anchors.fill: parent
        focus: true

        RowLayout {
            anchors.fill: parent
            spacing: 8
            id: row

            function renderCurrentPage() {
                MediaList.offset = offset.value;
                MediaList.rerender();
                grid.ScrollBar.vertical.position = 0;
            }

            Item {
                Layout.fillWidth: true
                Layout.maximumWidth: parent.width * 0.2
                Layout.fillHeight: true

                SpinBox {
                    id: offset
                    width: parent.width
                    wrap: true
                    stepSize: MediaList.render_amount
                    to: 0
                    onValueModified: row.renderCurrentPage()
                }    

                Button {
                    id: total
                    down: false
                    anchors.topMargin: 4
                    anchors.top: offset.bottom
                    anchors.horizontalCenter: offset.horizontalCenter
                    text: "0"
                }
            }

            CustomScrollGridView {
                property int spacing: 4

                Layout.fillHeight: true
                Layout.preferredWidth: Math.floor(parent.width / cellWidth) * cellWidth
                Layout.alignment: Qt.AlignRight

                Keys.onPressed: event => {
                    if (event.key == Qt.Key_Home) grid.currentIndex = 0;
                    if (event.key == Qt.Key_End) grid.currentIndex = grid.count - 1;
                    if (event.key == Qt.Key_Equal || event.key == Qt.Key_Plus) offset.increase() || row.renderCurrentPage();
                    if (event.key == Qt.Key_Minus) offset.decrease() || row.renderCurrentPage();
                }

                id: grid
                activeFocusOnTab: true
                focus: true
                clip: true
                cellWidth: MediaList.thumbnail_size
                cellHeight: cellWidth
                model: model

                delegate: Media {
                    width: grid.cellWidth - grid.spacing
                    height: grid.cellHeight - grid.spacing
                    focused: GridView.isCurrentItem
                }

                highlight: Item {}
            }
        }
    }

    Connections {
        target: MediaList

        function onAppend(media) {
            model.append({ media });
        }

        function onTotalChanged() {
            total.text = MediaList.total.toString();
            offset.to = Math.floor(MediaList.total / MediaList.render_amount) * MediaList.render_amount;
        }

        function onClear() {
            model.clear();
        }
    }
}

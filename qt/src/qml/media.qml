import QtQuick 2.12
import QtQuick.Window 2.12
import QtQuick.Controls 2.12

import com.github.CelestialCrafter.fenlu 1.0

Window {
    height: 480
    title: qsTr("Fenlu")
    visible: true
    width: 640
    id: window

    FenluMedia {
        id: media
    }

    ListModel {
        id: mediaModel
    }

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


    GridView {
        anchors.fill: parent
        id: view

        cellWidth: 128
        cellHeight: 128

        model: mediaModel
        delegate: Image {
            required property url url
            width: view.cellWidth
            height: view.cellHeight
            asynchronous: true
            cache: false
            fillMode: Image.PreserveAspectCrop
            source: url
        }

        property real accel: 0.7
        property real scrollMulti: 30
        property real animDuration: 20
        property real timerInterval: 15
        property real minVelocity: 0.1
        property real vy: 0

        boundsBehavior: Flickable.StopAtBounds
        flickDeceleration: 5000
        maximumFlickVelocity: 5000

        Behavior on contentY {
            NumberAnimation {
                duration: view.animDuration
                easing.type: Easing.Linear
            }
        }

        ScrollBar.vertical: ScrollBar {}

        Timer {
            id: scrollTimer
            interval: view.timerInterval
            running: false
            repeat: true
            onTriggered: {
                if (Math.abs(view.vy) > view.minVelocity) {
                    var newY = view.contentY - view.vy * (scrollTimer.interval / 1000);
                    // bounds checking
                    view.contentY = Math.max(0, Math.min(newY, view.contentHeight - view.height));
                    view.vy *= view.accel;
                } else {
                    view.vy = 0;
                    scrollTimer.stop();
                }
            }
        }

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.NoButton

            onWheel: {
                view.vy += wheel.angleDelta.y * view.scrollMulti;
                if (!scrollTimer.running) scrollTimer.start();
            }
        }
    }
}

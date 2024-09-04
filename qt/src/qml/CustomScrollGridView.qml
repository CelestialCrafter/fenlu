import QtQuick 2.12
import QtQuick.Controls 2.12

GridView {
    id: view

    property real accel: 0.7
    property real scrollMulti: 30
    property real animDuration: 20
    property real timerInterval: 15
    property real minVelocity: 0.1
    property real vy: 0

    boundsBehavior: Flickable.StopAtBounds
    flickDeceleration: 5000

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

        onWheel: function(wheel) {
            view.vy += wheel.angleDelta.y * view.scrollMulti;
            if (!scrollTimer.running) scrollTimer.start();
        }
    }
}

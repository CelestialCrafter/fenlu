import QtQuick 2.12
import QtQuick.Controls 2.12

Item {
        required property Flickable target
        property real accel: 0.7
        property real scrollMulti: 30
        property real animDuration: 20
        property real timerInterval: 15
        property real minVelocity: 0.1
        readonly property real vy: 0

        Timer {
            id: scrollTimer
            interval: timerInterval
            running: false
            repeat: true
            onTriggered: {
                if (Math.abs(vy) > minVelocity) {
                    var newY = target.contentY - vy * (scrollTimer.interval / 1000);
                    // bounds checking
                    target.contentY = Math.max(0, Math.min(newY, target.contentHeight - target.height));
                    vy *= accel;
                } else {
                    vy = 0;
                    scrollTimer.stop();
                }
            }
        }

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.NoButton
            onWheel: {
                    if (!target) return;

                    vy += wheel.angleDelta.y * scrollMulti;
                    if (!scrollTimer.running) scrollTimer.start();
            }
        }
}

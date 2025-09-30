#!/bin/bash

echo "=== PX4 MAVLink Connection Finder ==="
echo ""
echo "Checking PX4 process..."
px4_pid=$(pgrep -f "bin/px4" | head -1)
if [ -z "$px4_pid" ]; then
    echo "❌ PX4 not running!"
    exit 1
else
    echo "✅ PX4 is running (PID: $px4_pid)"
fi

echo ""
echo "PX4 is using these UDP ports:"
lsof -i UDP -a -p $px4_pid 2>/dev/null | grep -v "^COMMAND" | awk '{print $9}' | sort -u

echo ""
echo "Testing common MAVLink connection methods..."
echo ""

# Test 1: Standard broadcast listener
echo "1. Testing standard broadcast port (14540)..."
timeout 3 cargo run -- udpin:0.0.0.0:14540 2>&1 | grep -E "HEARTBEAT|Connected|Failed" | head -5

echo ""
echo "2. Testing simulator port as client (18570)..."
timeout 3 cargo run -- udpout:127.0.0.1:18570 2>&1 | grep -E "HEARTBEAT|Connected|Failed" | head -5

echo ""
echo "3. Testing alternative port (14560)..."
timeout 3 cargo run -- udpin:0.0.0.0:14560 2>&1 | grep -E "HEARTBEAT|Connected|Failed" | head -5

echo ""
echo "=== Recommendation ==="
echo "If PX4 Gazebo is running, it typically broadcasts to port 14540."
echo "You may need to add a MAVLink instance in PX4. Run this in the PX4 console:"
echo "  mavlink start -p -d /dev/ttyACM0 -m onboard -r 4000 -x -f"
echo "Or:"
echo "  mavlink stream -d /dev/ttyACM0 -r 50 -s ATTITUDE"
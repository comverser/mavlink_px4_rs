#!/bin/bash

echo "PX4 SITL Connection Test Script"
echo "================================"
echo ""

# Check if PX4 is listening on common ports
echo "Checking for PX4 on common ports..."
for port in 14540 14550 14557 18570; do
    if nc -zv 127.0.0.1 $port 2>/dev/null; then
        echo "✓ Port $port is open (PX4 might be listening here)"
    else
        echo "✗ Port $port is closed"
    fi
done

echo ""
echo "Testing different connection modes:"
echo ""

echo "1. Testing UDP listener on port 14540 (most common for PX4 SITL)..."
echo "   Command: cargo run -- udpin:0.0.0.0:14540"
echo ""
echo "   Press Ctrl+C after seeing messages to continue to next test"
echo "   Starting in 3 seconds..."
sleep 3
timeout 10 cargo run -- udpin:0.0.0.0:14540 2>&1 | head -20

echo ""
echo "2. Testing UDP client to localhost:14540..."
echo "   Command: cargo run -- udpout:127.0.0.1:14540"
echo ""
sleep 2
timeout 5 cargo run -- udpout:127.0.0.1:14540 2>&1 | head -20

echo ""
echo "================================"
echo "If you see HEARTBEAT messages, the connection is working!"
echo ""
echo "Common PX4 SITL configurations:"
echo "- Default: udpin:0.0.0.0:14540"
echo "- QGroundControl: udpin:0.0.0.0:14550"
echo "- MAVSDK: udpout:127.0.0.1:14540"
echo ""
echo "Make sure PX4 SITL is running with:"
echo "  make px4_sitl gazebo"
echo "or"
echo "  make px4_sitl jmavsim"
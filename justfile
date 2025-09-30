# MAVLink Rust Connection Helper
# Run with: just <command>

# Default command - show help
default:
    @just --list

# Build the project
build:
    cargo build

# Run with PX4 SITL (default connection)
run-px4:
    cargo run -- udpin:0.0.0.0:14540

# Run with custom connection string
run connection:
    cargo run -- {{connection}}

# Test PX4 connection
test-px4:
    @echo "Testing PX4 MAVLink connection..."
    timeout 5 cargo run -- udpin:0.0.0.0:14540 2>&1 | grep -E "HEARTBEAT|ATTITUDE|GPS" | head -10

# Find active MAVLink ports
find-ports:
    @echo "Checking for MAVLink on common ports..."
    @for port in 14540 14550 14560 14570 14580 18570; do \
        echo -n "Port $$port: "; \
        nc -zv 127.0.0.1 $$port 2>&1 | grep -q succeeded && echo "✓ Open" || echo "✗ Closed"; \
    done

# Show PX4 process info
px4-info:
    @echo "PX4 Process Information:"
    @ps aux | grep -E "px4|gazebo" | grep -v grep || echo "No PX4/Gazebo process found"
    @echo "\nPX4 UDP Ports:"
    @lsof -i UDP -a -p $(pgrep -f "bin/px4" | head -1) 2>/dev/null | grep -v "^COMMAND" || echo "PX4 not running"

# Run with ArduPilot SITL
run-ardupilot:
    cargo run -- tcpout:127.0.0.1:5760

# Run with serial connection
run-serial device="/dev/ttyUSB0" baud="57600":
    cargo run -- serial:{{device}}:{{baud}}

# Run with UDP broadcast
run-broadcast ip="192.168.1.255" port="14550":
    cargo run -- udpbcast:{{ip}}:{{port}}

# Monitor messages (with filtering)
monitor filter="HEARTBEAT":
    cargo run -- udpin:0.0.0.0:14540 2>&1 | grep {{filter}}

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Format code
fmt:
    cargo fmt

# Check code with clippy
lint:
    cargo clippy -- -D warnings

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Run with verbose output
verbose:
    RUST_LOG=debug cargo run -- udpin:0.0.0.0:14540

# Quick connect aliases
px4: run-px4
ardupilot: run-ardupilot
ports: find-ports
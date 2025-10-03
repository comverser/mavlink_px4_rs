# Default: Connect to PX4 SITL
default: sitl

# Connect to PX4 SITL
sitl msg="GLOBAL_POSITION_INT": (_run "udpin:0.0.0.0:14540" msg)

# Connect to serial hardware
serial msg="ATTITUDE": (_run "serial:/dev/ttyACM0:57600" msg)

# Run with custom connection and message
_run connection message="":
    #!/usr/bin/env bash
    if [ -z "{{message}}" ]; then
        cargo run -- "{{connection}}"
    else
        cargo run -- "{{connection}}" --messages "{{message}}"
    fi

# Default recipe - runs interactive menu
default: run-interactive

# Run with interactive menu for connection and message selection
run-interactive:
    #!/usr/bin/env bash
    echo "Select connection type:"
    echo "1) PX4 SITL (udpin:0.0.0.0:14540) [default]"
    echo "2) QGC default (udpin:0.0.0.0:14550)"
    echo "3) Custom"
    read -p "Choice [1-3] (press Enter for default): " choice
    choice=${choice:-1}

    case $choice in
        1) connection="udpin:0.0.0.0:14540" ;;
        2) connection="udpin:0.0.0.0:14550" ;;
        3) read -p "Enter connection string: " connection ;;
        *) echo "Invalid choice"; exit 1 ;;
    esac

    echo ""
    echo "Select message to display:"
    echo "1) All messages"
    echo "2) HEARTBEAT only"
    echo "3) ATTITUDE only"
    echo "4) GLOBAL_POSITION_INT only [default]"
    echo "5) GPS_RAW_INT only"
    echo "6) SYS_STATUS only"
    echo "7) PARAM_VALUE only"
    echo "8) Custom"
    read -p "Choice [1-8] (press Enter for default): " msg_choice
    msg_choice=${msg_choice:-4}

    case $msg_choice in
        1) cargo run -- "$connection" ;;
        2) cargo run -- "$connection" --messages HEARTBEAT ;;
        3) cargo run -- "$connection" --messages ATTITUDE ;;
        4) cargo run -- "$connection" --messages GLOBAL_POSITION_INT ;;
        5) cargo run -- "$connection" --messages GPS_RAW_INT ;;
        6) cargo run -- "$connection" --messages SYS_STATUS ;;
        7) cargo run -- "$connection" --messages PARAM_VALUE ;;
        8)
            read -p "Enter message type: " messages
            cargo run -- "$connection" --messages "$messages"
            ;;
        *) echo "Invalid choice"; exit 1 ;;
    esac
latest-log:
    #!/usr/bin/env bash
    STATE_LOG_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/chrona/logs"
    DATA_LOG_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/chrona/logs"

    LOG_DIR=""
    if [ -d "$STATE_LOG_DIR" ]; then
      LOG_DIR="$STATE_LOG_DIR"
    elif [ -d "$DATA_LOG_DIR" ]; then
      LOG_DIR="$DATA_LOG_DIR"
    else
      echo "Error: Chrona log directory not found in XDG state or data paths." >&2
      exit 1
    fi

    latest_log_filename=$(cd "$LOG_DIR" && ls -t | head -n 1)
    if [ -n "$latest_log_filename" ]; then
      echo "Displaying latest log: $LOG_DIR/$latest_log_filename"
      echo "---"
      bat "$LOG_DIR/$latest_log_filename"
    else
      echo "No log files found in '$LOG_DIR'."
    fi

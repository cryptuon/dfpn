#!/bin/bash
# DFPN Worker Startup Script
#
# This script starts all necessary services for a DFPN worker node.
#
# Usage:
#   ./scripts/start-worker.sh [--config CONFIG_FILE] [--no-video]
#
# Options:
#   --config FILE   Configuration file (default: config.yaml)
#   --no-video      Don't start video detection service

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MODELS_DIR="$PROJECT_DIR/models"

# Default values
CONFIG_FILE="$PROJECT_DIR/config.yaml"
START_VIDEO=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        --no-video)
            START_VIDEO=false
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "=============================================="
echo "DFPN Worker Startup"
echo "=============================================="
echo "Configuration: $CONFIG_FILE"
echo ""

# Check configuration file exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Error: Configuration file not found: $CONFIG_FILE"
    echo ""
    echo "Available configurations:"
    echo "  $PROJECT_DIR/config.yaml      (GPU)"
    echo "  $PROJECT_DIR/config-cpu.yaml  (CPU only)"
    exit 1
fi

# Activate virtual environment if it exists
if [ -d "$PROJECT_DIR/venv" ]; then
    echo "Activating Python virtual environment..."
    source "$PROJECT_DIR/venv/bin/activate"
fi

# Start video detection service (Docker)
if [ "$START_VIDEO" = true ]; then
    if command -v docker &> /dev/null; then
        VIDEO_DIR="$MODELS_DIR/video-ftcn"
        if [ -f "$VIDEO_DIR/docker-compose.yml" ]; then
            echo "Starting video detection service..."
            cd "$VIDEO_DIR"

            # Check if already running
            if docker-compose ps | grep -q "Up"; then
                echo "Video service already running"
            else
                docker-compose up -d
                echo "Waiting for video service to start..."
                sleep 10

                # Check health
                if curl -s http://localhost:8001/health | grep -q "healthy"; then
                    echo "✓ Video detection service started"
                else
                    echo "⚠ Video service may not be ready yet"
                fi
            fi
        else
            echo "Video service not configured (docker-compose.yml not found)"
        fi
    else
        echo "Docker not available, skipping video service"
    fi
fi

echo ""

# Check worker binary exists
WORKER_BIN="$PROJECT_DIR/target/release/dfpn-worker"
if [ ! -f "$WORKER_BIN" ]; then
    echo "Worker binary not found, attempting to build..."
    cd "$PROJECT_DIR"
    cargo build --release -p dfpn-worker

    if [ ! -f "$WORKER_BIN" ]; then
        echo "Error: Failed to build worker binary"
        exit 1
    fi
fi

# Start the DFPN worker
echo "Starting DFPN Worker..."
echo "Press Ctrl+C to stop"
echo ""
echo "=============================================="

cd "$PROJECT_DIR"
exec "$WORKER_BIN" --config "$CONFIG_FILE"

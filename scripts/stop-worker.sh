#!/bin/bash
# DFPN Worker Shutdown Script
#
# This script stops all DFPN worker services.
#
# Usage:
#   ./scripts/stop-worker.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MODELS_DIR="$PROJECT_DIR/models"

echo "=============================================="
echo "DFPN Worker Shutdown"
echo "=============================================="

# Stop video service
VIDEO_DIR="$MODELS_DIR/video-ftcn"
if [ -f "$VIDEO_DIR/docker-compose.yml" ]; then
    echo "Stopping video detection service..."
    cd "$VIDEO_DIR"
    docker-compose down || true
    echo "✓ Video service stopped"
fi

# Kill any running dfpn-worker processes
echo "Stopping DFPN worker processes..."
pkill -f "dfpn-worker" 2>/dev/null || true
echo "✓ Worker processes stopped"

echo ""
echo "All services stopped"

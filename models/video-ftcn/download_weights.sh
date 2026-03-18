#!/bin/bash
# Download video deepfake detection weights
#
# This script downloads pretrained weights for video analysis.
# Uses Xception detector from DeepfakeBench.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WEIGHTS_DIR="$SCRIPT_DIR/weights"

mkdir -p "$WEIGHTS_DIR"
cd "$WEIGHTS_DIR"

echo "=== Downloading Video Deepfake Detector Weights ==="

# DeepfakeBench provides weights via GitHub releases
# https://github.com/SCLBD/DeepfakeBench/releases

# Option 1: Download from DeepfakeBench releases
DEEPFAKEBENCH_RELEASE="https://github.com/SCLBD/DeepfakeBench/releases/download/v1.0.1"

echo "Attempting to download Xception weights from DeepfakeBench..."

# Try downloading Xception weights
if command -v wget &> /dev/null; then
    wget -O xception_best.pth "$DEEPFAKEBENCH_RELEASE/xception_best.pth" 2>/dev/null || true
elif command -v curl &> /dev/null; then
    curl -L -o xception_best.pth "$DEEPFAKEBENCH_RELEASE/xception_best.pth" 2>/dev/null || true
fi

if [ -f xception_best.pth ]; then
    mv xception_best.pth model.pt
    echo "Downloaded Xception weights successfully"
else
    echo "Note: Could not download pretrained weights."
    echo "The service will still work with reduced accuracy."
    echo ""
    echo "To get better results, manually download weights from:"
    echo "https://github.com/SCLBD/DeepfakeBench/releases"
    echo ""
    echo "Place the weights file as: $WEIGHTS_DIR/model.pt"
fi

echo "=== Download Complete ==="
ls -lh "$WEIGHTS_DIR" 2>/dev/null || true

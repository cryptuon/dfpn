#!/bin/bash
# Download UniversalFakeDetect pretrained weights
#
# These weights are trained for detecting AI-generated images
# using CLIP-ViT-L/14 features with a linear classification head.
#
# Source: https://github.com/WisconsinAIVision/UniversalFakeDetect
# Paper: "Detecting Generated Images by Real-World Images" (CVPR 2023)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Downloading Universal Fake Detect Weights ==="

# Official weights from the repository
WEIGHTS_URL="https://github.com/WisconsinAIVision/UniversalFakeDetect/raw/main/pretrained_weights/fc_weights.pth"

echo "Downloading from: $WEIGHTS_URL"

if command -v wget &> /dev/null; then
    wget -O fc_weights.pth "$WEIGHTS_URL"
elif command -v curl &> /dev/null; then
    curl -L -o fc_weights.pth "$WEIGHTS_URL"
else
    echo "Error: Neither wget nor curl found. Please install one of them."
    exit 1
fi

# Verify download
if [ -f fc_weights.pth ]; then
    echo "=== Download Complete ==="
    echo "Weights saved to: $SCRIPT_DIR/fc_weights.pth"
    ls -lh fc_weights.pth
else
    echo "Error: Download failed"
    exit 1
fi

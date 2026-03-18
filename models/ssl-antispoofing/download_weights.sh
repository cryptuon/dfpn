#!/bin/bash
# Download SSL Anti-spoofing pretrained weights
#
# This script downloads:
# 1. XLSR-53 base model (wav2vec 2.0 cross-lingual)
# 2. Trained classifier weights for spoofing detection
#
# Source: https://github.com/TakHemlata/SSL_Anti-spoofing

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Downloading SSL Anti-spoofing Weights ==="

# Download XLSR-53 (300M parameters)
# This is the self-supervised model used as feature extractor
XLSR_URL="https://dl.fbaipublicfiles.com/fairseq/wav2vec/xlsr2_300m.pt"

echo "Downloading XLSR-53 base model (1.2GB)..."
if [ ! -f xlsr2_300m.pt ]; then
    if command -v wget &> /dev/null; then
        wget -c "$XLSR_URL" -O xlsr2_300m.pt
    elif command -v curl &> /dev/null; then
        curl -L -C - -o xlsr2_300m.pt "$XLSR_URL"
    else
        echo "Error: Neither wget nor curl found"
        exit 1
    fi
else
    echo "XLSR model already downloaded"
fi

# Download classifier weights
# These are trained on ASVspoof 2021 dataset
echo ""
echo "Downloading classifier weights..."

# SSL Anti-spoofing classifier weights (from the paper's repository)
# Note: The exact URL depends on the repository release
# You may need to manually download from:
# https://github.com/TakHemlata/SSL_Anti-spoofing

# Try to download from common locations
CLASSIFIER_URLS=(
    "https://github.com/TakHemlata/SSL_Anti-spoofing/raw/main/best_SSL_model_LA.pth"
    "https://github.com/TakHemlata/SSL_Anti-spoofing/releases/download/v1.0/best_SSL_model_LA.pth"
)

DOWNLOADED=false
for url in "${CLASSIFIER_URLS[@]}"; do
    echo "Trying: $url"
    if wget -q --spider "$url" 2>/dev/null; then
        wget -O classifier_weights.pth "$url"
        DOWNLOADED=true
        break
    fi
done

if [ "$DOWNLOADED" = false ]; then
    echo ""
    echo "Note: Could not auto-download classifier weights."
    echo "The detector will still work with reduced accuracy using the simple model."
    echo ""
    echo "For best results, manually download from:"
    echo "https://github.com/TakHemlata/SSL_Anti-spoofing"
    echo ""
    echo "Look for 'best_SSL_model_LA.pth' (for Logical Access attacks)"
    echo "or 'best_SSL_model_DF.pth' (for DeepFake attacks)"
    echo ""
    echo "Place the file as: $SCRIPT_DIR/classifier_weights.pth"
fi

echo ""
echo "=== Download Complete ==="
ls -lh "$SCRIPT_DIR"/*.pt "$SCRIPT_DIR"/*.pth 2>/dev/null || true

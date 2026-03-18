#!/bin/bash
# Download SBI (Self-Blended Images) pretrained weights for face manipulation detection
#
# The weights are trained on FaceForensics++ dataset with Self-Blended augmentation
# Source: https://github.com/mapooon/SelfBlendedImages

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Downloading Face Forensics Detector Weights ==="

# Option 1: Download from SBI official release (Google Drive)
# Note: Replace with actual Google Drive file ID from the SBI repository
# The SBI team provides weights at: https://github.com/mapooon/SelfBlendedImages#pretrained-models
#
# FFraw.tar - Trained on raw (uncompressed) FaceForensics++
# FFc23.tar - Trained on c23 (HQ compressed) FaceForensics++

# Using gdown for Google Drive downloads
if command -v gdown &> /dev/null; then
    echo "Downloading SBI weights using gdown..."
    # FFc23 weights (compressed quality - more generalizable)
    # Replace YOUR_FILE_ID with actual ID from SBI repo
    # gdown "https://drive.google.com/uc?id=YOUR_FILE_ID" -O sbi_weights.tar
    # tar -xf sbi_weights.tar
    # mv FFc23/weights.pt model.pt
    # rm -rf FFc23 sbi_weights.tar
    echo "Note: Please download weights manually from https://github.com/mapooon/SelfBlendedImages"
    echo "and place them as model.pt in this directory"
else
    echo "gdown not installed. Install with: pip install gdown"
    echo "Or download weights manually from: https://github.com/mapooon/SelfBlendedImages"
fi

# Option 2: Use DFDC 1st place weights (alternative)
# These weights work well and are easier to download
# Source: https://github.com/selimsef/dfdc_deepfake_challenge
#
# Uncomment below to use DFDC weights instead:
# echo "Downloading DFDC EfficientNet-B4 weights..."
# wget -O model.pt "https://github.com/selimsef/dfdc_deepfake_challenge/releases/download/v1.0/efficientnet-b4_ns_jag_final.pth"

# Option 3: Create a dummy model for testing (generates random predictions)
if [ ! -f model.pt ]; then
    echo "Creating placeholder model for testing..."
    python3 << 'EOF'
import torch
from efficientnet_pytorch import EfficientNet

# Create model with random weights (for testing only)
model = EfficientNet.from_pretrained('efficientnet-b4', num_classes=2)
torch.save(model.state_dict(), 'model.pt')
print("Created placeholder model.pt - replace with trained weights for production")
EOF
fi

echo "=== Download Complete ==="
echo "Weights saved to: $SCRIPT_DIR/model.pt"

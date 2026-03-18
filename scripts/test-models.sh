#!/bin/bash
# DFPN Detection Models Test Script
#
# This script tests all detection models with sample inputs.
# Run this after setup-models.sh to verify everything works.
#
# Usage:
#   ./scripts/test-models.sh [--model MODEL_NAME]
#
# The script will:
# 1. Generate test samples if they don't exist
# 2. Run each model's inference on appropriate test files
# 3. Report success/failure for each model

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MODELS_DIR="$PROJECT_DIR/models"
SAMPLES_DIR="$PROJECT_DIR/test_samples"

echo "=============================================="
echo "DFPN Detection Models Test"
echo "=============================================="

# Create test samples directory
mkdir -p "$SAMPLES_DIR"

# Generate test image if it doesn't exist
create_test_image() {
    local path=$1
    if [ ! -f "$path" ]; then
        echo "Creating test image: $path"
        python3 << EOF
from PIL import Image
import numpy as np
# Create a simple test image (random noise)
img = Image.fromarray(np.random.randint(0, 255, (224, 224, 3), dtype=np.uint8))
img.save("$path")
print(f"Created test image: $path")
EOF
    fi
}

# Generate test audio if it doesn't exist
create_test_audio() {
    local path=$1
    if [ ! -f "$path" ]; then
        echo "Creating test audio: $path"
        python3 << EOF
import numpy as np
try:
    import scipy.io.wavfile as wav
    # Generate 2 seconds of white noise at 16kHz
    sample_rate = 16000
    duration = 2
    samples = np.random.randn(sample_rate * duration).astype(np.float32) * 0.1
    samples = (samples * 32767).astype(np.int16)
    wav.write("$path", sample_rate, samples)
    print(f"Created test audio: $path")
except ImportError:
    print("scipy not installed, creating placeholder audio")
    open("$path", "w").close()
EOF
    fi
}

# Create test samples
echo ""
echo "Preparing test samples..."
create_test_image "$SAMPLES_DIR/face.jpg"
create_test_image "$SAMPLES_DIR/image.png"
create_test_audio "$SAMPLES_DIR/audio.wav"

# Test results tracking
declare -A TEST_RESULTS

# Function to test a model
test_model() {
    local model_name=$1
    local test_file=$2
    local model_dir="$MODELS_DIR/$model_name"

    echo ""
    echo "----------------------------------------"
    echo "Testing: $model_name"
    echo "Input: $test_file"
    echo "----------------------------------------"

    if [ ! -d "$model_dir" ]; then
        echo "❌ Model directory not found"
        TEST_RESULTS[$model_name]="SKIP"
        return
    fi

    if [ ! -f "$model_dir/inference.py" ]; then
        echo "❌ inference.py not found"
        TEST_RESULTS[$model_name]="SKIP"
        return
    fi

    # Run inference
    cd "$model_dir"
    set +e
    OUTPUT=$(python3 inference.py "$test_file" 2>&1)
    EXIT_CODE=$?
    set -e

    if [ $EXIT_CODE -eq 0 ]; then
        # Check if output is valid JSON
        if echo "$OUTPUT" | python3 -c "import sys, json; json.load(sys.stdin)" 2>/dev/null; then
            echo "Output:"
            echo "$OUTPUT" | python3 -m json.tool
            echo ""
            echo "✓ Test passed"
            TEST_RESULTS[$model_name]="PASS"
        else
            echo "Output (not JSON):"
            echo "$OUTPUT"
            echo ""
            echo "⚠ Test completed but output is not valid JSON"
            TEST_RESULTS[$model_name]="WARN"
        fi
    else
        echo "Error output:"
        echo "$OUTPUT"
        echo ""
        echo "❌ Test failed (exit code: $EXIT_CODE)"
        TEST_RESULTS[$model_name]="FAIL"
    fi
}

# Test video-ftcn HTTP service
test_video_service() {
    echo ""
    echo "----------------------------------------"
    echo "Testing: video-ftcn (HTTP service)"
    echo "----------------------------------------"

    # Check if service is running
    if ! curl -s http://localhost:8001/health > /dev/null 2>&1; then
        echo "Video service not running at localhost:8001"
        echo "Start it with: cd models/video-ftcn && docker-compose up -d"
        TEST_RESULTS["video-ftcn"]="SKIP"
        return
    fi

    # Create a minimal test video (just a few frames)
    TEST_VIDEO="$SAMPLES_DIR/video.mp4"
    if [ ! -f "$TEST_VIDEO" ]; then
        echo "Creating test video..."
        python3 << EOF
import cv2
import numpy as np

# Create a 2-second video with random frames
fourcc = cv2.VideoWriter_fourcc(*'mp4v')
out = cv2.VideoWriter("$TEST_VIDEO", fourcc, 10, (224, 224))
for _ in range(20):
    frame = np.random.randint(0, 255, (224, 224, 3), dtype=np.uint8)
    out.write(frame)
out.release()
print("Created test video")
EOF
    fi

    # Test the service
    set +e
    OUTPUT=$(curl -s -X POST -F "file=@$TEST_VIDEO" http://localhost:8001/analyze)
    EXIT_CODE=$?
    set -e

    if [ $EXIT_CODE -eq 0 ]; then
        echo "Output:"
        echo "$OUTPUT" | python3 -m json.tool 2>/dev/null || echo "$OUTPUT"
        echo ""
        echo "✓ Test passed"
        TEST_RESULTS["video-ftcn"]="PASS"
    else
        echo "❌ Test failed"
        TEST_RESULTS["video-ftcn"]="FAIL"
    fi
}

# Run tests
echo ""
echo "Running model tests..."

# Face forensics
test_model "face-forensics" "$SAMPLES_DIR/face.jpg"

# Universal fake detect
test_model "universal-fake-detect" "$SAMPLES_DIR/image.png"

# SSL anti-spoofing
test_model "ssl-antispoofing" "$SAMPLES_DIR/audio.wav"

# Video service (if running)
test_video_service

# Summary
echo ""
echo "=============================================="
echo "Test Summary"
echo "=============================================="

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0
SKIP_COUNT=0

for model in "${!TEST_RESULTS[@]}"; do
    result="${TEST_RESULTS[$model]}"
    case $result in
        PASS)
            echo "✓ $model: PASS"
            ((PASS_COUNT++))
            ;;
        FAIL)
            echo "❌ $model: FAIL"
            ((FAIL_COUNT++))
            ;;
        WARN)
            echo "⚠ $model: WARNING"
            ((WARN_COUNT++))
            ;;
        SKIP)
            echo "○ $model: SKIPPED"
            ((SKIP_COUNT++))
            ;;
    esac
done

echo ""
echo "Results: $PASS_COUNT passed, $FAIL_COUNT failed, $WARN_COUNT warnings, $SKIP_COUNT skipped"

if [ $FAIL_COUNT -gt 0 ]; then
    exit 1
fi

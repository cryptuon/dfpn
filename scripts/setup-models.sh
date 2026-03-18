#!/bin/bash
# DFPN Detection Models Setup Script
#
# This script sets up all detection models for a DFPN worker node.
# It creates virtual environments, installs dependencies, and downloads weights.
#
# Usage:
#   ./scripts/setup-models.sh [--cpu-only] [--models MODEL1,MODEL2,...]
#
# Options:
#   --cpu-only      Skip GPU-specific setup
#   --models        Comma-separated list of models to setup (default: all)
#
# Examples:
#   ./scripts/setup-models.sh                           # Setup all models
#   ./scripts/setup-models.sh --cpu-only                # CPU-only setup
#   ./scripts/setup-models.sh --models face-forensics   # Setup only face-forensics

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MODELS_DIR="$PROJECT_DIR/models"

# Parse arguments
CPU_ONLY=false
MODELS="all"

while [[ $# -gt 0 ]]; do
    case $1 in
        --cpu-only)
            CPU_ONLY=true
            shift
            ;;
        --models)
            MODELS="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "=============================================="
echo "DFPN Detection Models Setup"
echo "=============================================="
echo "Project directory: $PROJECT_DIR"
echo "Models directory: $MODELS_DIR"
echo "CPU only: $CPU_ONLY"
echo "Models to setup: $MODELS"
echo ""

# Check Python version
PYTHON_VERSION=$(python3 --version 2>&1 | cut -d' ' -f2 | cut -d'.' -f1-2)
echo "Python version: $PYTHON_VERSION"

# Check for CUDA
if [ "$CPU_ONLY" = false ]; then
    if command -v nvidia-smi &> /dev/null; then
        echo "GPU detected:"
        nvidia-smi --query-gpu=name,memory.total --format=csv,noheader
    else
        echo "Warning: No GPU detected, continuing with CPU setup"
        CPU_ONLY=true
    fi
fi

echo ""

# Function to setup a model
setup_model() {
    local model_name=$1
    local model_dir="$MODELS_DIR/$model_name"

    echo "----------------------------------------"
    echo "Setting up: $model_name"
    echo "----------------------------------------"

    if [ ! -d "$model_dir" ]; then
        echo "Error: Model directory not found: $model_dir"
        return 1
    fi

    cd "$model_dir"

    # Install requirements if they exist
    if [ -f "requirements.txt" ]; then
        echo "Installing Python dependencies..."
        pip install -q -r requirements.txt
    fi

    # Download weights if script exists
    if [ -f "download_weights.sh" ]; then
        echo "Downloading model weights..."
        chmod +x download_weights.sh
        ./download_weights.sh
    fi

    # Make inference.py executable
    if [ -f "inference.py" ]; then
        chmod +x inference.py
    fi

    echo "✓ $model_name setup complete"
    echo ""
}

# Create virtual environment (optional but recommended)
create_venv() {
    VENV_DIR="$PROJECT_DIR/venv"

    if [ ! -d "$VENV_DIR" ]; then
        echo "Creating Python virtual environment..."
        python3 -m venv "$VENV_DIR"
    fi

    echo "Activating virtual environment..."
    source "$VENV_DIR/bin/activate"

    # Upgrade pip
    pip install --upgrade pip

    # Install PyTorch with appropriate CUDA version
    if [ "$CPU_ONLY" = true ]; then
        echo "Installing PyTorch (CPU)..."
        pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu
    else
        echo "Installing PyTorch (CUDA 11.8)..."
        pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
    fi

    # Install common dependencies
    pip install numpy pillow opencv-python
}

# Main setup
main() {
    # Create models directory if it doesn't exist
    mkdir -p "$MODELS_DIR"

    # Optionally create venv
    read -p "Create/use Python virtual environment? (recommended) [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_venv
    fi

    # Determine which models to setup
    if [ "$MODELS" = "all" ]; then
        MODELS_LIST=(
            "face-forensics"
            "universal-fake-detect"
            "ssl-antispoofing"
        )

        # Only setup video-ftcn if Docker is available and GPU exists
        if command -v docker &> /dev/null && [ "$CPU_ONLY" = false ]; then
            MODELS_LIST+=("video-ftcn")
        fi
    else
        IFS=',' read -ra MODELS_LIST <<< "$MODELS"
    fi

    # Setup each model
    for model in "${MODELS_LIST[@]}"; do
        setup_model "$model" || echo "Warning: Failed to setup $model"
    done

    # Setup video-ftcn Docker container
    if [[ " ${MODELS_LIST[*]} " =~ " video-ftcn " ]] && command -v docker &> /dev/null; then
        echo "----------------------------------------"
        echo "Building video-ftcn Docker container"
        echo "----------------------------------------"
        cd "$MODELS_DIR/video-ftcn"

        # Download weights first
        if [ -f "download_weights.sh" ]; then
            chmod +x download_weights.sh
            ./download_weights.sh
        fi

        # Build Docker image
        docker-compose build
        echo "✓ video-ftcn Docker container built"
    fi

    echo ""
    echo "=============================================="
    echo "Setup Complete!"
    echo "=============================================="
    echo ""
    echo "Next steps:"
    echo "1. Test models: ./scripts/test-models.sh"
    echo "2. Start worker: ./scripts/start-worker.sh"
    echo ""
    echo "Configuration files:"
    echo "  GPU:  $PROJECT_DIR/config.yaml"
    echo "  CPU:  $PROJECT_DIR/config-cpu.yaml"
}

main

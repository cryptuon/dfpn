# DFPN Video Authenticity Detector
# Uses FTCN (Fully Temporal Convolutional Network) from DeepfakeBench
#
# This service runs as a separate container due to heavy dependencies
# and provides HTTP API for video deepfake detection.

FROM pytorch/pytorch:2.0.1-cuda11.7-cudnn8-runtime

LABEL maintainer="DFPN Team"
LABEL description="Video deepfake detection service using FTCN"

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libsm6 \
    libxext6 \
    libgl1-mesa-glx \
    git \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip install --no-cache-dir \
    flask>=3.0.0 \
    gunicorn>=21.0.0 \
    opencv-python-headless>=4.8.0 \
    numpy>=1.24.0 \
    pillow>=10.0.0 \
    dlib>=19.24.0 \
    scikit-learn>=1.3.0 \
    tqdm>=4.65.0

# Clone DeepfakeBench for model architectures
RUN git clone --depth 1 https://github.com/SCLBD/DeepfakeBench.git /app/deepfakebench

# Install DeepfakeBench dependencies
WORKDIR /app/deepfakebench
RUN pip install --no-cache-dir -r requirements.txt || true

# Create weights directory
RUN mkdir -p /app/weights

# Copy server code
COPY server.py /app/server.py
COPY config.py /app/config.py

WORKDIR /app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Expose port
EXPOSE 8000

# Run with gunicorn for production
CMD ["gunicorn", "-w", "1", "-b", "0.0.0.0:8000", "--timeout", "300", "server:app"]

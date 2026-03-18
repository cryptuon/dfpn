"""Configuration for video detection service."""

# Model settings
MODEL_WEIGHTS_PATH = "/app/weights/model.pt"

# Video processing
MAX_FRAMES = 32  # Maximum frames to sample from video
FRAME_SIZE = 299  # Face crop size for model input

# Server settings
HOST = "0.0.0.0"
PORT = 8000
WORKERS = 1  # Keep at 1 for GPU memory efficiency
TIMEOUT = 300  # 5 minutes for long videos

# Detection thresholds
FAKE_THRESHOLD = 0.7  # Above this -> "manipulated"
REAL_THRESHOLD = 0.3  # Below this -> "authentic"

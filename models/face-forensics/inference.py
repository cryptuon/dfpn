#!/usr/bin/env python3
"""
DFPN Face Manipulation Detector using SBI/EfficientNet-B4

This detector identifies face manipulation (deepfakes, face swaps, reenactment)
using the Self-Blended Images approach with EfficientNet-B4 backbone.

Usage:
    python inference.py /path/to/image.jpg

Output:
    JSON object with verdict, confidence, and detections

Source: https://github.com/mapooon/SelfBlendedImages
"""

import sys
import json
import os
from pathlib import Path

import torch
import numpy as np
from PIL import Image

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent.absolute()


class FaceForensicsDetector:
    """Face manipulation detector using EfficientNet-B4 trained on SBI dataset."""

    def __init__(self, weights_path=None):
        """
        Initialize the detector.

        Args:
            weights_path: Path to model weights. Defaults to model.pt in script directory.
        """
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

        if weights_path is None:
            weights_path = SCRIPT_DIR / "model.pt"

        # Lazy imports to speed up module loading when just checking interface
        from facenet_pytorch import MTCNN
        from efficientnet_pytorch import EfficientNet

        # Initialize face detector
        self.mtcnn = MTCNN(
            device=self.device,
            select_largest=True,
            post_process=False,
        )

        # Initialize classifier
        self.model = EfficientNet.from_pretrained("efficientnet-b4", num_classes=2)

        # Load trained weights if available
        if os.path.exists(weights_path):
            state_dict = torch.load(weights_path, map_location=self.device)
            self.model.load_state_dict(state_dict)
            print(f"Loaded weights from {weights_path}", file=sys.stderr)
        else:
            print(
                f"Warning: No weights found at {weights_path}, using pretrained backbone",
                file=sys.stderr,
            )

        self.model.to(self.device)
        self.model.eval()

        # Normalization parameters (ImageNet)
        self.mean = torch.tensor([0.485, 0.456, 0.406]).view(1, 3, 1, 1).to(self.device)
        self.std = torch.tensor([0.229, 0.224, 0.225]).view(1, 3, 1, 1).to(self.device)

    def preprocess(self, image_path):
        """
        Extract and preprocess face from image.

        Args:
            image_path: Path to input image

        Returns:
            Tuple of (tensor, region) or (None, None) if no face found
        """
        img = Image.open(image_path).convert("RGB")
        img_array = np.array(img)

        # Detect face
        boxes, probs = self.mtcnn.detect(img)

        if boxes is None or len(boxes) == 0:
            return None, None

        # Get first (largest) face
        x1, y1, x2, y2 = map(int, boxes[0])

        # Add margin (30% of face size)
        width = x2 - x1
        height = y2 - y1
        margin = int(0.3 * max(width, height))

        # Expand bounding box with margin
        x1 = max(0, x1 - margin)
        y1 = max(0, y1 - margin)
        x2 = min(img_array.shape[1], x2 + margin)
        y2 = min(img_array.shape[0], y2 + margin)

        # Crop and resize face
        face = img.crop((x1, y1, x2, y2))
        face = face.resize((380, 380), Image.BILINEAR)

        # Convert to tensor and normalize
        tensor = torch.tensor(np.array(face)).permute(2, 0, 1).float() / 255.0
        tensor = tensor.unsqueeze(0).to(self.device)
        tensor = (tensor - self.mean) / self.std

        region = {
            "x": int(boxes[0][0]),
            "y": int(boxes[0][1]),
            "width": int(boxes[0][2] - boxes[0][0]),
            "height": int(boxes[0][3] - boxes[0][1]),
        }

        return tensor, region

    def analyze(self, media_path):
        """
        Analyze image for face manipulation.

        Args:
            media_path: Path to input image

        Returns:
            Dict with verdict, confidence, and detections
        """
        tensor, region = self.preprocess(media_path)

        if tensor is None:
            return {
                "verdict": "inconclusive",
                "confidence": 0,
                "detections": [
                    {
                        "detection_type": "no_face_detected",
                        "confidence": 0,
                        "region": None,
                    }
                ],
            }

        with torch.no_grad():
            output = self.model(tensor)
            prob = torch.softmax(output, dim=1)
            # Index 1 is "fake/manipulated" class
            fake_prob = prob[0][1].item()

        # Determine verdict
        if fake_prob > 0.7:
            verdict = "manipulated"
            confidence = int(fake_prob * 100)
        elif fake_prob < 0.3:
            verdict = "authentic"
            confidence = int((1 - fake_prob) * 100)
        else:
            verdict = "inconclusive"
            confidence = int(max(fake_prob, 1 - fake_prob) * 100)

        return {
            "verdict": verdict,
            "confidence": confidence,
            "detections": [
                {
                    "detection_type": "face_manipulation",
                    "confidence": confidence,
                    "region": region,
                }
            ],
        }


def main():
    """Main entry point for DFPN worker integration."""
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: python inference.py <image_path>"}))
        sys.exit(1)

    media_path = sys.argv[1]

    if not os.path.exists(media_path):
        print(json.dumps({"error": f"File not found: {media_path}"}))
        sys.exit(1)

    try:
        detector = FaceForensicsDetector()
        result = detector.analyze(media_path)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"error": str(e), "verdict": "inconclusive", "confidence": 0, "detections": []}))
        sys.exit(1)


if __name__ == "__main__":
    main()

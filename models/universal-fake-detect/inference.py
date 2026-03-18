#!/usr/bin/env python3
"""
DFPN AI-Generated Image Detector using UniversalFakeDetect (CLIP-based)

This detector identifies AI-generated images (Stable Diffusion, DALL-E, Midjourney,
GANs, etc.) using CLIP features with a trained classification head.

Usage:
    python inference.py /path/to/image.png

Output:
    JSON object with verdict, confidence, and detections

Source: https://github.com/WisconsinAIVision/UniversalFakeDetect
Paper: "Detecting Generated Images by Real-World Images" (CVPR 2023)
"""

import sys
import json
import os
from pathlib import Path

import torch
import torch.nn as nn
from PIL import Image

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent.absolute()


class UniversalFakeDetector:
    """
    AI-generated image detector using CLIP features.

    Uses frozen CLIP-ViT-L/14 as feature extractor with a trained linear
    classification head. Achieves excellent generalization across unseen
    generators including diffusion models.
    """

    def __init__(self, weights_path=None):
        """
        Initialize the detector.

        Args:
            weights_path: Path to classification head weights.
                         Defaults to fc_weights.pth in script directory.
        """
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

        if weights_path is None:
            weights_path = SCRIPT_DIR / "fc_weights.pth"

        # Lazy import CLIP
        try:
            import clip
        except ImportError:
            raise ImportError(
                "CLIP not installed. Install with: pip install git+https://github.com/openai/CLIP.git"
            )

        # Load CLIP model (ViT-L/14 for best performance)
        self.clip_model, self.preprocess = clip.load("ViT-L/14", device=self.device)
        self.clip_model.eval()

        # Freeze CLIP - we only use it as feature extractor
        for param in self.clip_model.parameters():
            param.requires_grad = False

        # Classification head: Linear layer from CLIP features to binary output
        # CLIP ViT-L/14 produces 768-dimensional features
        self.fc = nn.Linear(768, 1)

        # Load trained weights if available
        if os.path.exists(weights_path):
            state_dict = torch.load(weights_path, map_location=self.device)
            self.fc.load_state_dict(state_dict)
            print(f"Loaded weights from {weights_path}", file=sys.stderr)
        else:
            print(
                f"Warning: No weights found at {weights_path}, using random initialization",
                file=sys.stderr,
            )

        self.fc.to(self.device)
        self.fc.eval()

    def analyze(self, image_path):
        """
        Analyze image for AI generation artifacts.

        Args:
            image_path: Path to input image

        Returns:
            Dict with verdict, confidence, and detections
        """
        try:
            # Load and preprocess image
            image = Image.open(image_path).convert("RGB")
            image_tensor = self.preprocess(image).unsqueeze(0).to(self.device)
        except Exception as e:
            return {
                "verdict": "inconclusive",
                "confidence": 0,
                "detections": [
                    {
                        "detection_type": "image_load_error",
                        "confidence": 0,
                        "region": None,
                    }
                ],
            }

        with torch.no_grad():
            # Extract CLIP features
            features = self.clip_model.encode_image(image_tensor).float()

            # Classify
            logit = self.fc(features)
            prob = torch.sigmoid(logit).item()

        # prob > 0.5 means generated/fake
        if prob > 0.7:
            verdict = "manipulated"  # AI-generated
            confidence = int(prob * 100)
        elif prob < 0.3:
            verdict = "authentic"  # Real image
            confidence = int((1 - prob) * 100)
        else:
            verdict = "inconclusive"
            confidence = int(max(prob, 1 - prob) * 100)

        return {
            "verdict": verdict,
            "confidence": confidence,
            "detections": [
                {
                    "detection_type": "ai_generated_image",
                    "confidence": confidence,
                    "region": None,  # Full image analysis
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
        detector = UniversalFakeDetector()
        result = detector.analyze(media_path)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({
            "error": str(e),
            "verdict": "inconclusive",
            "confidence": 0,
            "detections": []
        }))
        sys.exit(1)


if __name__ == "__main__":
    main()

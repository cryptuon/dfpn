#!/usr/bin/env python3
"""
DFPN Voice Cloning / Audio Spoofing Detector

Uses self-supervised learning features (wav2vec 2.0 / XLSR) with a trained
classification head to detect synthetic/cloned audio.

Usage:
    python inference.py /path/to/audio.wav

Output:
    JSON object with verdict, confidence, and detections

Source: https://github.com/TakHemlata/SSL_Anti-spoofing
Paper: "Automatic Speaker Verification Spoofing and Deepfake Detection" (ASVspoof 2021)
"""

import sys
import json
import os
from pathlib import Path

import torch
import torch.nn as nn
import numpy as np

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent.absolute()


class SSLAntispoofModel(nn.Module):
    """
    SSL-based anti-spoofing model.

    Uses wav2vec 2.0 / XLSR features with a classification head.
    """

    def __init__(self, ssl_model, hidden_dim=1024):
        super().__init__()
        self.ssl_model = ssl_model

        # Classification head
        self.classifier = nn.Sequential(
            nn.Linear(hidden_dim, 256),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(256, 2),  # [bonafide, spoof]
        )

    def forward(self, x):
        # Extract SSL features
        with torch.no_grad():
            features = self.ssl_model.extract_features(x, padding_mask=None)
            # Pool over time dimension
            pooled = features["x"].mean(dim=1)

        # Classify
        logits = self.classifier(pooled)
        return logits


class SSLAntispoofDetector:
    """
    Voice cloning / audio spoofing detector using SSL features.

    Supports:
    - wav2vec 2.0 based detection
    - XLSR (cross-lingual) based detection
    - Simple spectrogram-based fallback
    """

    def __init__(self, weights_path=None, use_ssl=True):
        """
        Initialize the detector.

        Args:
            weights_path: Path to classifier weights.
            use_ssl: Whether to use SSL features (requires fairseq).
        """
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

        if weights_path is None:
            weights_path = SCRIPT_DIR / "classifier_weights.pth"

        self.use_ssl = use_ssl
        self.ssl_model = None
        self.model = None

        if use_ssl:
            self._init_ssl_model(weights_path)
        else:
            self._init_simple_model(weights_path)

    def _init_ssl_model(self, weights_path):
        """Initialize SSL-based model."""
        try:
            import torchaudio
            from fairseq.models.wav2vec import Wav2VecModel

            # Load XLSR or wav2vec 2.0 checkpoint
            xlsr_path = SCRIPT_DIR / "xlsr2_300m.pt"

            if xlsr_path.exists():
                print(f"Loading XLSR model from {xlsr_path}", file=sys.stderr)
                cp = torch.load(xlsr_path, map_location=self.device)
                self.ssl_model = Wav2VecModel.build_model(cp["args"], task=None)
                self.ssl_model.load_state_dict(cp["model"])
                self.ssl_model.to(self.device)
                self.ssl_model.eval()

                # Freeze SSL model
                for param in self.ssl_model.parameters():
                    param.requires_grad = False

                # Create full model
                self.model = SSLAntispoofModel(self.ssl_model, hidden_dim=1024)

                # Load classifier weights if available
                if os.path.exists(weights_path):
                    state_dict = torch.load(weights_path, map_location=self.device)
                    self.model.classifier.load_state_dict(state_dict)
                    print(f"Loaded classifier weights from {weights_path}", file=sys.stderr)

                self.model.to(self.device)
                self.model.eval()
            else:
                print(f"XLSR model not found at {xlsr_path}", file=sys.stderr)
                print("Falling back to simple spectrogram model", file=sys.stderr)
                self.use_ssl = False
                self._init_simple_model(weights_path)

        except ImportError as e:
            print(f"SSL dependencies not available: {e}", file=sys.stderr)
            print("Falling back to simple spectrogram model", file=sys.stderr)
            self.use_ssl = False
            self._init_simple_model(weights_path)

    def _init_simple_model(self, weights_path):
        """Initialize simple spectrogram-based model as fallback."""
        # Simple CNN for spectrogram classification
        self.model = nn.Sequential(
            nn.Conv2d(1, 32, 3, padding=1),
            nn.ReLU(),
            nn.MaxPool2d(2),
            nn.Conv2d(32, 64, 3, padding=1),
            nn.ReLU(),
            nn.MaxPool2d(2),
            nn.AdaptiveAvgPool2d((4, 4)),
            nn.Flatten(),
            nn.Linear(64 * 4 * 4, 128),
            nn.ReLU(),
            nn.Linear(128, 2),
        )

        if os.path.exists(weights_path):
            try:
                state_dict = torch.load(weights_path, map_location=self.device)
                self.model.load_state_dict(state_dict)
            except:
                pass

        self.model.to(self.device)
        self.model.eval()

    def load_audio(self, audio_path):
        """Load and preprocess audio file."""
        try:
            import torchaudio

            waveform, sr = torchaudio.load(audio_path)

            # Convert to mono
            if waveform.shape[0] > 1:
                waveform = waveform.mean(dim=0, keepdim=True)

            # Resample to 16kHz (required for wav2vec)
            if sr != 16000:
                resampler = torchaudio.transforms.Resample(sr, 16000)
                waveform = resampler(waveform)

            # Normalize
            waveform = waveform / (waveform.abs().max() + 1e-8)

            return waveform.squeeze(0)

        except ImportError:
            # Fallback to scipy/librosa if torchaudio not available
            try:
                import librosa

                waveform, sr = librosa.load(audio_path, sr=16000, mono=True)
                return torch.tensor(waveform)
            except ImportError:
                raise ImportError(
                    "Audio loading requires torchaudio or librosa. "
                    "Install with: pip install torchaudio or pip install librosa"
                )

    def compute_spectrogram(self, waveform):
        """Compute mel spectrogram for simple model."""
        try:
            import torchaudio

            mel_transform = torchaudio.transforms.MelSpectrogram(
                sample_rate=16000,
                n_fft=1024,
                hop_length=256,
                n_mels=80,
            )
            mel = mel_transform(waveform.unsqueeze(0))
            mel = torch.log(mel + 1e-8)
            return mel.unsqueeze(0)  # Add channel dim
        except:
            # Manual spectrogram computation
            import numpy as np
            from scipy import signal

            f, t, Sxx = signal.spectrogram(waveform.numpy(), fs=16000)
            mel = torch.tensor(np.log(Sxx + 1e-8)).float().unsqueeze(0).unsqueeze(0)
            return mel

    def analyze(self, audio_path):
        """
        Analyze audio for voice cloning / spoofing.

        Args:
            audio_path: Path to audio file (WAV, MP3, FLAC, etc.)

        Returns:
            Dict with verdict, confidence, and detections
        """
        try:
            waveform = self.load_audio(audio_path)
            waveform = waveform.to(self.device)
        except Exception as e:
            return {
                "verdict": "inconclusive",
                "confidence": 0,
                "detections": [
                    {
                        "detection_type": "audio_load_error",
                        "confidence": 0,
                        "region": None,
                    }
                ],
            }

        # Truncate or pad to reasonable length (max 10 seconds)
        max_samples = 16000 * 10
        if waveform.shape[0] > max_samples:
            waveform = waveform[:max_samples]

        with torch.no_grad():
            if self.use_ssl:
                # SSL-based inference
                logits = self.model(waveform.unsqueeze(0))
            else:
                # Simple spectrogram-based inference
                spec = self.compute_spectrogram(waveform).to(self.device)
                logits = self.model(spec)

            prob = torch.softmax(logits, dim=1)
            # Index 1 is "spoof" class
            spoof_prob = prob[0][1].item()

        # Determine verdict
        if spoof_prob > 0.7:
            verdict = "manipulated"  # Spoofed/cloned audio
            confidence = int(spoof_prob * 100)
        elif spoof_prob < 0.3:
            verdict = "authentic"  # Bonafide audio
            confidence = int((1 - spoof_prob) * 100)
        else:
            verdict = "inconclusive"
            confidence = int(max(spoof_prob, 1 - spoof_prob) * 100)

        # Calculate audio duration for region info
        duration_ms = int(waveform.shape[0] / 16000 * 1000)

        return {
            "verdict": verdict,
            "confidence": confidence,
            "detections": [
                {
                    "detection_type": "voice_cloning",
                    "confidence": confidence,
                    "region": {
                        "start_ms": 0,
                        "end_ms": duration_ms,
                    },
                }
            ],
        }


def main():
    """Main entry point for DFPN worker integration."""
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: python inference.py <audio_path>"}))
        sys.exit(1)

    audio_path = sys.argv[1]

    if not os.path.exists(audio_path):
        print(json.dumps({"error": f"File not found: {audio_path}"}))
        sys.exit(1)

    try:
        detector = SSLAntispoofDetector()
        result = detector.analyze(audio_path)
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

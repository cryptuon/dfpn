<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import ComparisonTable from '../../components/learn/ComparisonTable.vue'

const models = [
  {
    name: 'face-forensics',
    modality: 'Face Manipulation',
    modalityColor: 'bg-pink-500/20 text-pink-400',
    accentColor: 'border-pink-500',
    architecture: 'SBI / EfficientNet-B4',
    stat: '97.2% accuracy on FF++',
    description: 'Detects face swaps, reenactments, and facial manipulation in images.',
  },
  {
    name: 'universal-fake-detect',
    modality: 'AI-Generated Images',
    modalityColor: 'bg-purple-500/20 text-purple-400',
    accentColor: 'border-purple-500',
    architecture: 'CLIP-ViT-L/14',
    stat: '99.8% on ProGAN',
    description: 'Identifies AI-generated images from various generator architectures.',
  },
  {
    name: 'video-ftcn',
    modality: 'Video Authenticity',
    modalityColor: 'bg-blue-500/20 text-blue-400',
    accentColor: 'border-blue-500',
    architecture: 'Xception + Temporal CNN',
    stat: '96.4% on FaceForensics++',
    description: 'Analyzes temporal consistency to detect manipulated video content.',
  },
  {
    name: 'ssl-antispoofing',
    modality: 'Voice Cloning',
    modalityColor: 'bg-emerald-500/20 text-emerald-400',
    accentColor: 'border-emerald-500',
    architecture: 'wav2vec 2.0 / XLSR-53',
    stat: '99.2% on ASVspoof',
    description: 'Detects synthetic and cloned voices in audio recordings.',
  },
]

const performanceColumns = [
  { key: 'model', label: 'Model' },
  { key: 'dataset', label: 'Primary Dataset' },
  { key: 'accuracy', label: 'Accuracy' },
  { key: 'auc', label: 'AUC' },
]

const performanceRows = [
  { model: 'face-forensics', dataset: 'FF++ (c23)', accuracy: '97.2%', auc: '0.994' },
  { model: 'universal-fake-detect', dataset: 'ProGAN', accuracy: '99.8%', auc: '0.998' },
  { model: 'video-ftcn', dataset: 'FaceForensics++', accuracy: '96.4%', auc: '0.987' },
  { model: 'ssl-antispoofing', dataset: 'ASVspoof 2021', accuracy: '99.2%', auc: '0.995' },
]

const speedColumns = [
  { key: 'model', label: 'Model' },
  { key: 'gpu', label: 'GPU Time' },
  { key: 'cpu', label: 'CPU Time' },
]

const speedRows = [
  { model: 'face-forensics', gpu: '50 ms', cpu: '500 ms' },
  { model: 'universal-fake-detect', gpu: '100 ms', cpu: '800 ms' },
  { model: 'video-ftcn', gpu: '2 seconds', cpu: '30 seconds' },
  { model: 'ssl-antispoofing', gpu: '200 ms', cpu: '2 seconds' },
]
</script>

<template>
  <div class="space-y-8">
    <!-- Title -->
    <div>
      <h2 class="text-2xl font-bold text-white">Detection Models</h2>
      <p class="text-gray-400 mt-2">
        The algorithms that power deepfake detection across the DFPN network.
      </p>
    </div>

    <!-- Model Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div
        v-for="m in models"
        :key="m.name"
        class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden"
      >
        <div class="h-1" :class="m.accentColor.replace('border-', 'bg-')"></div>
        <div class="p-6">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-lg font-semibold text-white font-mono">{{ m.name }}</h3>
            <span class="text-xs px-2 py-1 rounded-full font-medium" :class="m.modalityColor">
              {{ m.modality }}
            </span>
          </div>
          <p class="text-sm text-gray-500 mb-1">{{ m.architecture }}</p>
          <p class="text-sm font-medium text-dfpn-primary-light mb-3">{{ m.stat }}</p>
          <p class="text-sm text-gray-400">{{ m.description }}</p>
        </div>
      </div>
    </div>

    <!-- Performance Comparison -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Performance Comparison</h3>
      <ComparisonTable :columns="performanceColumns" :rows="performanceRows" />
    </div>

    <!-- Processing Speed -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Processing Speed</h3>
      <ComparisonTable :columns="speedColumns" :rows="speedRows" />
    </div>

    <!-- Supported Modalities -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Supported Modalities</h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <InfoCard
          icon="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
          title="Image Authenticity"
          description="Detect manipulated or tampered photographs and images."
        />
        <InfoCard
          icon="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
          title="Video Authenticity"
          description="Identify manipulated video with temporal and spatial analysis."
        />
        <InfoCard
          icon="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
          title="Audio Authenticity"
          description="Detect spliced, edited, or synthesized audio recordings."
        />
        <InfoCard
          icon="M15 12a3 3 0 11-6 0 3 3 0 016 0zM2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
          title="Face Manipulation"
          description="Detect face swaps, reenactments, and facial attribute edits."
        />
        <InfoCard
          icon="M5.636 18.364a9 9 0 010-12.728m12.728 0a9 9 0 010 12.728m-9.9-2.829a5 5 0 010-7.07m7.072 0a5 5 0 010 7.07M13 12a1 1 0 11-2 0 1 1 0 012 0z"
          title="Voice Cloning"
          description="Identify cloned or synthetically generated speech audio."
        />
        <InfoCard
          icon="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
          title="Generated Content"
          description="Detect fully AI-generated images, video, and audio."
        />
      </div>
    </div>
  </div>
</template>

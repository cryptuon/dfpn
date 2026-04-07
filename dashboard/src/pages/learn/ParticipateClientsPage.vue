<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import StepIndicator from '../../components/learn/StepIndicator.vue'
import ComparisonTable from '../../components/learn/ComparisonTable.vue'
import ParticipateTabNav from '../../components/learn/ParticipateTabNav.vue'

const howItWorksSteps = [
  { number: 1, title: 'Upload', description: 'Upload media to IPFS, Arweave, or S3.' },
  { number: 2, title: 'Request', description: 'Submit analysis request with hash, modalities, and fee.' },
  { number: 3, title: 'Wait', description: 'Multiple workers analyze independently.' },
  { number: 4, title: 'Result', description: 'Receive consensus verdict with confidence score.' },
]

const feeColumns = [
  { key: 'modality', label: 'Modality' },
  { key: 'baseFee', label: 'Base Fee' },
  { key: 'typicalTime', label: 'Typical Time' },
]

const feeRows = [
  { modality: 'Image Authenticity', baseFee: '~0.002 SOL', typicalTime: '1-2 seconds' },
  { modality: 'Face Manipulation', baseFee: '~0.003 SOL', typicalTime: '1-2 seconds' },
  { modality: 'AI-Generated Image', baseFee: '~0.003 SOL', typicalTime: '1-2 seconds' },
  { modality: 'Video Authenticity', baseFee: '~0.008 SOL', typicalTime: '5-30 seconds' },
  { modality: 'Voice Cloning', baseFee: '~0.004 SOL', typicalTime: '2-5 seconds' },
]
</script>

<template>
  <div class="space-y-8">
    <!-- Title + Tab Nav -->
    <div>
      <h2 class="text-2xl font-bold text-white">Participate in DFPN</h2>
      <p class="text-gray-400 mt-2">Choose how you want to contribute to the network.</p>
    </div>
    <ParticipateTabNav />

    <!-- Hero -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <h3 class="text-xl font-semibold text-white">Submit Media for Analysis</h3>
      <p class="text-gray-400 mt-2">
        Clients submit images, videos, and audio to the DFPN network for deepfake detection.
        Multiple independent workers analyze your content and return a consensus verdict with a confidence score.
      </p>
    </div>

    <!-- How It Works -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <h3 class="text-lg font-semibold text-white mb-6">How It Works</h3>
      <StepIndicator :steps="howItWorksSteps" direction="horizontal" />
    </div>

    <!-- Integration Options -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Integration Options</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
          title="TypeScript SDK"
          description="Easiest for JS/TS apps. Full type safety and helper methods."
        />
        <InfoCard
          icon="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2z"
          title="Python SDK"
          description="For Python applications and data pipelines."
        />
        <InfoCard
          icon="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
          title="REST API"
          description="Quick integration via the indexer HTTP endpoints."
        />
        <InfoCard
          icon="M13 10V3L4 14h7v7l9-11h-7z"
          title="Direct RPC"
          description="Advanced: interact directly with Solana programs."
        />
      </div>
    </div>

    <!-- Fee Tiers -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Fee Tiers</h3>
      <ComparisonTable :columns="feeColumns" :rows="feeRows" />
    </div>

    <!-- Fee Distribution -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Fee Distribution</h3>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <div class="flex gap-1 rounded-lg overflow-hidden h-10">
          <div class="bg-dfpn-primary flex items-center justify-center text-xs font-medium text-white" style="width: 65%">65%</div>
          <div class="bg-emerald-500 flex items-center justify-center text-xs font-medium text-white" style="width: 20%">20%</div>
          <div class="bg-blue-500 flex items-center justify-center text-xs font-medium text-white" style="width: 10%">10%</div>
          <div class="bg-yellow-500 flex items-center justify-center text-xs font-medium text-gray-900" style="width: 5%"></div>
        </div>
        <div class="flex gap-4 mt-4 flex-wrap">
          <div class="flex items-center gap-2">
            <div class="w-3 h-3 rounded bg-dfpn-primary"></div>
            <span class="text-sm text-gray-400">65% Workers</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-3 h-3 rounded bg-emerald-500"></div>
            <span class="text-sm text-gray-400">20% Model Devs</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-3 h-3 rounded bg-blue-500"></div>
            <span class="text-sm text-gray-400">10% Treasury</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-3 h-3 rounded bg-yellow-500"></div>
            <span class="text-sm text-gray-400">5% Insurance</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Use Cases -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Use Cases</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          title="Content Moderation"
          description="Automatically flag deepfakes in user-generated content."
        />
        <InfoCard
          icon="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2"
          title="Journalism"
          description="Verify authenticity of images and videos before publication."
        />
        <InfoCard
          icon="M10 6H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V8a2 2 0 00-2-2h-5m-4 0V5a2 2 0 114 0v1m-4 0a2 2 0 104 0"
          title="Identity Verification"
          description="Detect face swaps in KYC processes."
        />
        <InfoCard
          icon="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"
          title="Social Media"
          description="Protect platforms from AI-generated misinformation."
        />
      </div>
    </div>
  </div>
</template>

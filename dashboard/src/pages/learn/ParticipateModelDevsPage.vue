<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import StepIndicator from '../../components/learn/StepIndicator.vue'
import ParticipateTabNav from '../../components/learn/ParticipateTabNav.vue'

const lifecycleSteps = [
  { number: 1, title: 'Develop Model', description: 'Build a detection model for one or more modalities.' },
  { number: 2, title: 'Submit Metadata', description: 'Register model name, version, modalities, and download URI on-chain.' },
  { number: 3, title: 'Stake DFPN', description: 'Stake 20,000 DFPN per model version as a quality commitment.' },
  { number: 4, title: 'Benchmark', description: 'The protocol tests your model on held-out datasets.' },
  { number: 5, title: 'Activation', description: 'Model goes live once it meets accuracy thresholds.' },
  { number: 6, title: 'Adoption', description: 'Workers deploy and use your model for detection tasks.' },
  { number: 7, title: 'Earn Revenue', description: 'Receive 20% of fees from requests processed with your model.' },
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
      <h3 class="text-xl font-semibold text-white">Register Detection Models</h3>
      <p class="text-gray-400 mt-2">
        Model developers contribute detection algorithms to the network. Your models are deployed
        by workers across the globe, and you earn a share of every fee generated from requests
        that use them.
      </p>
    </div>

    <!-- Model Lifecycle -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <h3 class="text-lg font-semibold text-white mb-6">Model Lifecycle</h3>
      <StepIndicator :steps="lifecycleSteps" direction="vertical" />
    </div>

    <!-- Requirements -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Requirements</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          title="Minimum Stake"
          description="20,000 DFPN per model version."
        />
        <InfoCard
          icon="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
          title="Standard Output"
          description="JSON format with verdict, confidence, and detections."
        />
        <InfoCard
          icon="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          title="Benchmark Passing"
          description="Must meet accuracy thresholds on evaluation datasets."
        />
        <InfoCard
          icon="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
          title="Modality Support"
          description="Declare supported detection types: image, video, audio, face, voice."
        />
      </div>
    </div>

    <!-- Revenue Model -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Revenue Model</h3>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-4">
        <p class="text-gray-400">
          Model developers receive <span class="text-emerald-400 font-semibold">20% of the fees</span>
          from every detection request that uses their model. This fee is automatically distributed
          on-chain when a request is finalized.
        </p>
        <p class="text-gray-400">
          In addition to per-request fees, model developers earn epoch rewards proportional to the
          total usage and accuracy of their models. Higher-performing models attract more workers,
          which drives more usage and greater revenue.
        </p>
        <div class="flex gap-1 rounded-lg overflow-hidden h-8 max-w-md">
          <div class="bg-dfpn-primary flex items-center justify-center text-xs font-medium text-white" style="width: 65%">65% Workers</div>
          <div class="bg-emerald-500 flex items-center justify-center text-xs font-medium text-white" style="width: 20%">20% You</div>
          <div class="bg-blue-500 flex items-center justify-center text-xs font-medium text-white" style="width: 15%">15%</div>
        </div>
      </div>
    </div>

    <!-- Output Format -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Output Format</h3>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <p class="text-gray-400 mb-4">
          All models must return results in the following standardized JSON format:
        </p>
        <div class="bg-gray-900/50 border border-dfpn-border rounded-lg p-4 font-mono text-sm space-y-1">
          <p class="text-gray-400">verdict: <span class="text-emerald-400">"authentic"</span> | <span class="text-yellow-400">"manipulated"</span> | <span class="text-gray-500">"inconclusive"</span></p>
          <p class="text-gray-400">confidence: <span class="text-dfpn-primary-light">0-100</span></p>
          <p class="text-gray-400">detections: [</p>
          <p class="text-gray-400 pl-4">{{ '{' }} detection_type, confidence, region? {{ '}' }}</p>
          <p class="text-gray-400">]</p>
        </div>
      </div>
    </div>
  </div>
</template>

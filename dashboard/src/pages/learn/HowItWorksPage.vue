<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import StepIndicator from '../../components/learn/StepIndicator.vue'
</script>

<template>
  <div class="space-y-8">
    <!-- Title -->
    <div>
      <h1 class="text-2xl font-bold text-white">How DFPN Works</h1>
      <p class="text-gray-400 mt-2">
        A step-by-step look at how media analysis requests flow through the decentralized detection network.
      </p>
    </div>

    <!-- Request Lifecycle -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-6">
      <h2 class="text-2xl font-bold text-white">Request Lifecycle</h2>
      <StepIndicator
        direction="vertical"
        :steps="[
          { number: 1, title: 'Submit', description: 'Client uploads media and posts analysis request with fee and modalities.' },
          { number: 2, title: 'Route', description: 'Workers poll the indexer for requests matching their supported modalities.' },
          { number: 3, title: 'Download', description: 'Matched workers fetch media from the storage URI (IPFS, Arweave, S3).' },
          { number: 4, title: 'Analyze', description: 'Workers run detection models locally on their own hardware.' },
          { number: 5, title: 'Commit', description: 'Workers submit a hash of their result to Solana (prevents copying).' },
          { number: 6, title: 'Reveal', description: 'After commit window closes, workers reveal their actual results.' },
          { number: 7, title: 'Aggregate', description: 'Results are combined into a consensus verdict using reputation weights.' },
          { number: 8, title: 'Finalize', description: 'Request is marked complete with immutable results on-chain.' },
          { number: 9, title: 'Reward', description: 'Workers and model developers earn based on accuracy and availability.' },
        ]"
      />
    </div>

    <!-- Commit-Reveal -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-6">
      <h2 class="text-2xl font-bold text-white">Commit-Reveal Protocol</h2>
      <p class="text-gray-400">
        To prevent workers from copying each other's answers, DFPN uses a two-phase commit-reveal scheme. Workers first submit a cryptographic hash of their result. Only after all commits are recorded do workers reveal their actual answers. The on-chain hash proves the result was determined before other answers were visible.
      </p>
      <StepIndicator
        direction="horizontal"
        :steps="[
          { number: 1, title: 'Commit', description: 'Lock answer hash on-chain.' },
          { number: 2, title: 'Wait', description: 'All answers remain hidden.' },
          { number: 3, title: 'Reveal', description: 'Verify answers match hashes.' },
        ]"
      />
    </div>

    <!-- Scoring Factors -->
    <div class="space-y-6">
      <h2 class="text-2xl font-bold text-white">Scoring Factors</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
          title="Accuracy - 50%"
          description="Agreement with consensus and ground truth. The most heavily weighted factor in determining worker reputation."
        />
        <InfoCard
          icon="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"
          title="Availability - 25%"
          description="Percentage of assigned tasks completed. Workers must remain online and responsive to maintain high scores."
        />
        <InfoCard
          icon="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
          title="Latency - 15%"
          description="Speed relative to deadline. Faster responses earn better scores, but accuracy always takes priority."
        />
        <InfoCard
          icon="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          title="Consistency - 10%"
          description="Variance in result quality over time. Steady, reliable performance is rewarded over erratic bursts."
        />
      </div>
    </div>

    <!-- Security -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-4">
      <h2 class="text-2xl font-bold text-white">Security Guarantees</h2>
      <ul class="space-y-3">
        <li class="flex items-start gap-3">
          <svg class="w-5 h-5 text-dfpn-primary-light mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span class="text-gray-400 text-sm"><span class="text-white font-medium">No on-chain inference</span> - DFPN never sees your models. All detection runs locally on worker hardware.</span>
        </li>
        <li class="flex items-start gap-3">
          <svg class="w-5 h-5 text-dfpn-primary-light mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span class="text-gray-400 text-sm"><span class="text-white font-medium">Operator independence</span> - Choose your own hardware, models, and configuration.</span>
        </li>
        <li class="flex items-start gap-3">
          <svg class="w-5 h-5 text-dfpn-primary-light mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span class="text-gray-400 text-sm"><span class="text-white font-medium">Commit-reveal prevents result copying</span> - Cryptographic hashes ensure independent analysis.</span>
        </li>
        <li class="flex items-start gap-3">
          <svg class="w-5 h-5 text-dfpn-primary-light mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span class="text-gray-400 text-sm"><span class="text-white font-medium">Staking + slashing enforces honest behavior</span> - Economic penalties deter bad actors.</span>
        </li>
        <li class="flex items-start gap-3">
          <svg class="w-5 h-5 text-dfpn-primary-light mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span class="text-gray-400 text-sm"><span class="text-white font-medium">Multi-worker consensus prevents manipulation</span> - No single worker can control the outcome.</span>
        </li>
      </ul>
    </div>
  </div>
</template>

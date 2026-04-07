<script setup lang="ts">
import RoadmapTimeline from '../../components/learn/RoadmapTimeline.vue'
</script>

<template>
  <div class="space-y-8">
    <!-- Title -->
    <div>
      <h1 class="text-2xl font-bold text-white">Roadmap</h1>
      <p class="text-gray-400 mt-2">
        The development timeline for the Decentralized Deepfake Detection Network, from initial research through mainnet launch and beyond.
      </p>
    </div>

    <!-- Timeline -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <RoadmapTimeline
        :phases="[
          {
            label: 'Phase 0',
            title: 'Research & Foundations',
            duration: '4-6 weeks',
            items: [
              'Threat model definition',
              'Dataset selection and curation',
              'Evaluation harness building',
              'Benchmark suite creation',
            ],
            status: 'completed',
          },
          {
            label: 'Phase 1',
            title: 'Devnet MVP',
            duration: '6-8 weeks',
            items: [
              'Core Solana programs deployed',
              'Content registry and marketplace',
              'Single reference worker implementation',
              'Commit-reveal protocol',
              'Basic CLI and dashboard',
            ],
            status: 'completed',
          },
          {
            label: 'Phase 2',
            title: 'Testnet Pilot',
            duration: '8-10 weeks',
            items: [
              'Worker registry and staking',
              'Epoch-based scoring and rewards',
              'Model versioning system',
              'Indexer and REST API',
              'Load testing and optimization',
            ],
            status: 'current',
          },
          {
            label: 'Phase 3',
            title: 'Mainnet Beta',
            duration: '8-12 weeks',
            items: [
              'SPL token treasury launch',
              'Governance via Realms',
              'Security audits',
              'Public beta with capped volume',
              'Model marketplace',
            ],
            status: 'upcoming',
          },
          {
            label: 'Phase 4',
            title: 'Scale & Ecosystem',
            duration: 'Ongoing',
            items: [
              'Advanced storage integrations',
              'Specialized detection modes',
              'Platform partnerships',
              'Cross-chain bridges',
              'Community governance expansion',
            ],
            status: 'upcoming',
          },
        ]"
      />
    </div>
  </div>
</template>

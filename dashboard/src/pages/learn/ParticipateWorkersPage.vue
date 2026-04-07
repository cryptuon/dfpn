<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import StepIndicator from '../../components/learn/StepIndicator.vue'
import ComparisonTable from '../../components/learn/ComparisonTable.vue'
import ParticipateTabNav from '../../components/learn/ParticipateTabNav.vue'

const hardwareColumns = [
  { key: 'tier', label: 'Tier' },
  { key: 'cpu', label: 'CPU' },
  { key: 'ram', label: 'RAM' },
  { key: 'gpu', label: 'GPU' },
  { key: 'storage', label: 'Storage' },
  { key: 'network', label: 'Network' },
]

const hardwareRows = [
  { tier: 'Entry', cpu: '8 cores', ram: '32 GB', gpu: 'RTX 3080 10 GB', storage: '500 GB SSD', network: '100 Mbps' },
  { tier: 'Mid', cpu: '16 cores', ram: '64 GB', gpu: 'RTX 4090 24 GB', storage: '1 TB NVMe', network: '500 Mbps' },
  { tier: 'Pro', cpu: '32+ cores', ram: '128 GB', gpu: 'A100 80 GB', storage: '2 TB NVMe', network: '1 Gbps' },
]

const gettingStartedSteps = [
  { number: 1, title: 'Install Worker Client', description: 'Clone the repo and build the dfpn-worker binary.' },
  { number: 2, title: 'Set Up Wallet', description: 'Generate or import a Solana keypair.' },
  { number: 3, title: 'Fund Wallet', description: 'Get SOL for transactions and DFPN for staking.' },
  { number: 4, title: 'Configure Models', description: 'Run the setup script to download detection models.' },
  { number: 5, title: 'Configure Worker', description: 'Edit config.yaml with modalities, fees, and concurrency.' },
  { number: 6, title: 'Register On-Chain', description: 'Stake DFPN and register as a worker.' },
  { number: 7, title: 'Start Worker', description: 'Launch the daemon and begin processing tasks.' },
]

const slashingColumns = [
  { key: 'violation', label: 'Violation' },
  { key: 'penalty', label: 'Penalty' },
  { key: 'details', label: 'Details' },
]

const slashingRows = [
  { violation: 'Invalid Results', penalty: '10% stake', details: 'Results that disagree with consensus' },
  { violation: 'Missed Deadlines', penalty: '1-3% stake', details: 'Failing to submit within the commit window' },
  { violation: 'Repeated Failures', penalty: 'Progressive', details: 'Escalating penalties for >20% failure rate' },
  { violation: 'Fraud or Collusion', penalty: '25-50% stake', details: 'Coordinated manipulation of results' },
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
      <h3 class="text-xl font-semibold text-white">Run a Detection Node</h3>
      <p class="text-gray-400 mt-2">
        Workers provide GPU/CPU infrastructure to analyze media. You run detection models locally
        and earn DFPN tokens for accurate, timely results.
      </p>
    </div>

    <!-- What You Provide vs. What You Receive -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">What You Provide vs. What You Receive</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M5 12h14M12 5l7 7-7 7"
          title="You Provide"
          description="GPU/CPU hardware, detection models, storage, stable internet, and a 5,000 DFPN stake."
        />
        <InfoCard
          icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          title="You Receive"
          description="Task assignments, DFPN token rewards, a reputation score, and 65% of request fees."
        />
      </div>
    </div>

    <!-- Hardware Requirements -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Hardware Requirements</h3>
      <ComparisonTable :columns="hardwareColumns" :rows="hardwareRows" />
    </div>

    <!-- Getting Started -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <h3 class="text-lg font-semibold text-white mb-6">Getting Started</h3>
      <StepIndicator :steps="gettingStartedSteps" direction="vertical" />
    </div>

    <!-- Slashing Penalties -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Slashing Penalties</h3>
      <ComparisonTable :columns="slashingColumns" :rows="slashingRows" />
    </div>
  </div>
</template>

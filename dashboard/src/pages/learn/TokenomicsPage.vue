<script setup lang="ts">
import StatHighlight from '../../components/learn/StatHighlight.vue'
import ComparisonTable from '../../components/learn/ComparisonTable.vue'

const emissionsColumns = [
  { key: 'year', label: 'Year' },
  { key: 'rate', label: 'Emission Rate' },
  { key: 'tokens', label: 'Tokens' },
]

const emissionsRows = [
  { year: 'Year 1', rate: '12%', tokens: '120,000,000' },
  { year: 'Year 2', rate: '9%', tokens: '90,000,000' },
  { year: 'Year 3', rate: '6%', tokens: '60,000,000' },
  { year: 'Year 4', rate: '4%', tokens: '40,000,000' },
  { year: 'Year 5', rate: '3%', tokens: '30,000,000' },
  { year: 'Year 6', rate: '2%', tokens: '20,000,000' },
  { year: 'Year 7', rate: '1%', tokens: '10,000,000' },
  { year: 'Year 8', rate: '1%', tokens: '10,000,000' },
]

const stakingColumns = [
  { key: 'role', label: 'Role' },
  { key: 'minimumStake', label: 'Minimum Stake' },
  { key: 'purpose', label: 'Purpose' },
]

const stakingRows = [
  { role: 'Worker', minimumStake: '5,000 DFPN', purpose: 'Skin in the game for honest detection' },
  { role: 'Model Developer', minimumStake: '20,000 DFPN', purpose: 'Quality commitment per model version' },
  { role: 'Challenger', minimumStake: '5% of disputed reward', purpose: 'Cost to initiate a dispute' },
]

const slashingColumns = [
  { key: 'violation', label: 'Violation' },
  { key: 'penalty', label: 'Penalty' },
]

const slashingRows = [
  { violation: 'Invalid Results', penalty: '10% of stake' },
  { violation: 'Missed Deadlines', penalty: '1-3% of stake' },
  { violation: 'Repeated Failures', penalty: 'Progressive escalation' },
  { violation: 'Fraud or Collusion', penalty: '25-50% of stake' },
]

const allocationSegments = [
  { label: 'Network Rewards', pct: 38, color: 'bg-dfpn-primary' },
  { label: 'Treasury', pct: 20, color: 'bg-blue-500' },
  { label: 'Team & Advisors', pct: 18, color: 'bg-emerald-500' },
  { label: 'Ecosystem', pct: 12, color: 'bg-orange-500' },
  { label: 'Strategic', pct: 7, color: 'bg-pink-500' },
  { label: 'Liquidity', pct: 5, color: 'bg-cyan-500' },
]
</script>

<template>
  <div class="space-y-8">
    <!-- Title -->
    <div>
      <h2 class="text-2xl font-bold text-white">DFPN Token Economics</h2>
      <p class="text-gray-400 mt-2">Supply, allocation, emissions, and staking mechanics.</p>
    </div>

    <!-- Key Stats -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatHighlight value="1,000,000,000" label="Total Supply" />
        <StatHighlight value="DFPN" label="SPL Token on Solana" />
        <StatHighlight value="38%" label="Allocated to Rewards" />
      </div>
    </div>

    <!-- Token Allocation -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Token Allocation</h3>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <div class="flex gap-0.5 rounded-lg overflow-hidden h-12">
          <div
            v-for="seg in allocationSegments"
            :key="seg.label"
            :class="seg.color"
            class="flex items-center justify-center text-xs font-medium text-white transition-all"
            :style="{ width: seg.pct + '%' }"
          >
            <span v-if="seg.pct >= 10">{{ seg.pct }}%</span>
          </div>
        </div>
        <div class="flex flex-wrap gap-4 mt-4">
          <div
            v-for="seg in allocationSegments"
            :key="seg.label"
            class="flex items-center gap-2"
          >
            <div class="w-3 h-3 rounded" :class="seg.color"></div>
            <span class="text-sm text-gray-400">{{ seg.pct }}% {{ seg.label }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Emissions Schedule -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Emissions Schedule</h3>
      <ComparisonTable :columns="emissionsColumns" :rows="emissionsRows" />
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

    <!-- Staking Requirements -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Staking Requirements</h3>
      <ComparisonTable :columns="stakingColumns" :rows="stakingRows" />
    </div>

    <!-- Slashing Penalties -->
    <div>
      <h3 class="text-lg font-semibold text-white mb-4">Slashing Penalties</h3>
      <ComparisonTable :columns="slashingColumns" :rows="slashingRows" />
    </div>
  </div>
</template>

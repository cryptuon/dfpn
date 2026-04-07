<script setup lang="ts">
import FaqAccordion from '../../components/learn/FaqAccordion.vue'

const generalFaqs = [
  {
    question: 'What is DFPN?',
    answer: 'DFPN is a decentralized coordination layer for deepfake detection built on Solana. It connects clients who need media verified with independent workers who run detection algorithms, using economic incentives to ensure honest behavior.',
  },
  {
    question: 'How is DFPN different from centralized detection?',
    answer: 'Unlike centralized services, DFPN uses multiple independent workers with different models. Results are aggregated into consensus verdicts with full on-chain transparency. No single entity controls detection.',
  },
  {
    question: 'What blockchain does DFPN use?',
    answer: 'DFPN runs on Solana for fast, low-cost transactions. All requests, results, and reputation data are stored on-chain.',
  },
  {
    question: 'What types of deepfakes can DFPN detect?',
    answer: 'DFPN supports face manipulation, AI-generated images, video authenticity, and voice cloning detection through four specialized models.',
  },
  {
    question: 'Is DFPN live?',
    answer: 'DFPN is currently in the testnet pilot phase. See the Roadmap for timeline details.',
  },
]

const workerFaqs = [
  {
    question: 'How do I become a worker?',
    answer: 'Install the worker client, set up a Solana wallet, acquire at least 5,000 DFPN tokens for staking, configure your models, and register on-chain. See the Participate page for detailed steps.',
  },
  {
    question: 'What hardware do I need?',
    answer: 'Minimum: 8-core CPU, 32GB RAM, RTX 3080 GPU (10GB VRAM), 500GB SSD. Recommended: 16+ cores, 64GB RAM, RTX 4090 or A100.',
  },
  {
    question: 'How much can I earn?',
    answer: 'Earnings depend on task volume, accuracy, and stake weight. Workers receive 65% of request fees plus epoch-based rewards from the token emission schedule.',
  },
  {
    question: 'What happens if I submit incorrect results?',
    answer: 'Invalid results trigger a 10% stake slash. Repeated failures escalate penalties. Fraud or collusion can result in 25-50% slashing.',
  },
  {
    question: 'Can I run a CPU-only node?',
    answer: 'Yes, but GPU nodes process tasks much faster. CPU-only nodes handle image analysis in ~500-800ms vs 50-100ms on GPU. Video analysis is significantly slower on CPU.',
  },
]

const tokenFaqs = [
  {
    question: 'What is the DFPN token used for?',
    answer: 'DFPN is used for staking (workers and model developers), paying analysis fees, earning rewards, and participating in governance.',
  },
  {
    question: 'How does staking work?',
    answer: 'Workers must stake at least 5,000 DFPN. Model developers stake 20,000 DFPN per model version. Stakes can be slashed for bad behavior and earn rewards for good performance.',
  },
  {
    question: 'What is slashing?',
    answer: 'Slashing removes a portion of a participant\'s staked tokens as penalty. Rates: 10% for invalid results, 1-3% for missed deadlines, 25-50% for fraud or collusion.',
  },
]

const securityFaqs = [
  {
    question: 'How does commit-reveal prevent cheating?',
    answer: 'Workers submit a hash of their result before seeing other answers. After all commits, they reveal the actual results. This prevents copying and ensures independent analysis.',
  },
  {
    question: 'What if workers disagree?',
    answer: 'When workers disagree, the system uses reputation-weighted consensus. Workers who consistently agree with ground truth earn higher reputation and their votes carry more weight.',
  },
]
</script>

<template>
  <div class="space-y-8">
    <!-- Title -->
    <div>
      <h1 class="text-2xl font-bold text-white">Frequently Asked Questions</h1>
    </div>

    <!-- General -->
    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-white">General</h3>
      <FaqAccordion :items="generalFaqs" />
    </div>

    <!-- Workers -->
    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-white">Workers</h3>
      <FaqAccordion :items="workerFaqs" />
    </div>

    <!-- Tokens -->
    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-white">Tokens</h3>
      <FaqAccordion :items="tokenFaqs" />
    </div>

    <!-- Security -->
    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-white">Security</h3>
      <FaqAccordion :items="securityFaqs" />
    </div>
  </div>
</template>

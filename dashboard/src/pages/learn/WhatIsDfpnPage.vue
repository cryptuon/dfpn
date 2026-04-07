<script setup lang="ts">
import InfoCard from '../../components/learn/InfoCard.vue'
import StatHighlight from '../../components/learn/StatHighlight.vue'
</script>

<template>
  <div class="space-y-8">
    <!-- Hero -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 text-center space-y-6">
      <h1 class="text-2xl font-bold text-white">The Decentralized Deepfake Detection Network</h1>
      <p class="text-gray-400 max-w-2xl mx-auto">
        DFPN coordinates independent node operators to detect deepfakes using economic incentives on Solana.
      </p>
      <div class="flex justify-center gap-12">
        <StatHighlight value="1B" label="Total Supply" />
        <StatHighlight value="4" label="Detection Models" />
        <StatHighlight value="Solana" label="Blockchain" />
      </div>
    </div>

    <!-- The Problem -->
    <div class="space-y-6">
      <h2 class="text-2xl font-bold text-white">The Deepfake Crisis</h2>
      <p class="text-gray-400">
        AI-generated fake media is growing exponentially. From face swaps to fully synthetic voices, deepfakes threaten elections, financial markets, personal reputations, and the very concept of trust in digital content. Centralized detection services create single points of failure and cannot scale to meet the challenge.
      </p>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <InfoCard
          icon="M15 12a3 3 0 11-6 0 3 3 0 016 0zM2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
          title="Face Swaps"
          description="Replacing one person's face with another in images and video, enabling impersonation and fraud at scale."
        />
        <InfoCard
          icon="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
          title="AI-Generated Images"
          description="Entirely synthetic photos created by diffusion models that are indistinguishable from real photographs."
        />
        <InfoCard
          icon="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
          title="Video Manipulation"
          description="Altered video footage with changed facial expressions, lip movements, or entire body actions."
        />
        <InfoCard
          icon="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
          title="Voice Cloning"
          description="Synthetic audio that replicates a person's voice with just seconds of sample material."
        />
      </div>
    </div>

    <!-- How DFPN Solves It -->
    <div class="space-y-6">
      <h2 class="text-2xl font-bold text-white">How DFPN Solves It</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <InfoCard
          icon="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
          title="Decentralized Detection"
          description="Multiple independent workers analyze each request - no single point of failure."
        />
        <InfoCard
          icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          title="Economic Incentives"
          description="Staking and rewards ensure honest behavior - bad actors get slashed."
        />
        <InfoCard
          icon="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
          title="Multi-Model Consensus"
          description="Results from different algorithms are aggregated into reliable verdicts."
        />
      </div>
    </div>

    <!-- Three Roles -->
    <div class="space-y-6">
      <h2 class="text-2xl font-bold text-white">Three Roles in the Network</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <InfoCard
          icon="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2"
          title="Workers"
          description="Run detection hardware and earn DFPN tokens."
        >
          <router-link to="/learn/participate" class="inline-block mt-3 text-sm text-dfpn-primary-light hover:underline">
            Learn more &rarr;
          </router-link>
        </InfoCard>
        <InfoCard
          icon="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
          title="Clients"
          description="Submit media for deepfake analysis."
        >
          <router-link to="/learn/participate" class="inline-block mt-3 text-sm text-dfpn-primary-light hover:underline">
            Learn more &rarr;
          </router-link>
        </InfoCard>
        <InfoCard
          icon="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
          title="Model Developers"
          description="Register detection models and earn from usage."
        >
          <router-link to="/learn/participate" class="inline-block mt-3 text-sm text-dfpn-primary-light hover:underline">
            Learn more &rarr;
          </router-link>
        </InfoCard>
      </div>
    </div>

    <!-- CTA -->
    <div class="flex gap-4 justify-center">
      <router-link
        to="/"
        class="px-6 py-3 bg-dfpn-primary text-white font-medium rounded-lg hover:bg-dfpn-primary/80 transition-colors"
      >
        Explore the Network
      </router-link>
      <router-link
        to="/learn/faq"
        class="px-6 py-3 bg-dfpn-surface border border-dfpn-border text-gray-400 font-medium rounded-lg hover:border-dfpn-primary/30 transition-colors"
      >
        Read the FAQ
      </router-link>
    </div>
  </div>
</template>

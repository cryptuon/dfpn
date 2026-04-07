import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  scrollBehavior() {
    return { top: 0 }
  },
  routes: [
    // Landing
    { path: '/', name: 'overview', component: () => import('../pages/OverviewPage.vue') },

    // Learn
    { path: '/learn', name: 'learn', component: () => import('../pages/learn/WhatIsDfpnPage.vue') },
    { path: '/learn/how-it-works', name: 'how-it-works', component: () => import('../pages/learn/HowItWorksPage.vue') },
    { path: '/learn/participate/workers', name: 'participate-workers', component: () => import('../pages/learn/ParticipateWorkersPage.vue') },
    { path: '/learn/participate/clients', name: 'participate-clients', component: () => import('../pages/learn/ParticipateClientsPage.vue') },
    { path: '/learn/participate/model-developers', name: 'participate-model-devs', component: () => import('../pages/learn/ParticipateModelDevsPage.vue') },
    { path: '/learn/tokenomics', name: 'tokenomics', component: () => import('../pages/learn/TokenomicsPage.vue') },
    { path: '/learn/detection-models', name: 'detection-models', component: () => import('../pages/learn/DetectionModelsPage.vue') },
    { path: '/learn/roadmap', name: 'roadmap', component: () => import('../pages/learn/RoadmapPage.vue') },
    { path: '/learn/faq', name: 'faq', component: () => import('../pages/learn/FaqPage.vue') },

    // Network
    { path: '/workers', name: 'workers', component: () => import('../pages/WorkersPage.vue') },
    { path: '/workers/:operator', name: 'worker-detail', component: () => import('../pages/WorkerDetailPage.vue'), props: true },
    { path: '/models', name: 'models', component: () => import('../pages/ModelsPage.vue') },
    { path: '/models/:id', name: 'model-detail', component: () => import('../pages/ModelDetailPage.vue'), props: true },
    { path: '/requests', name: 'requests', component: () => import('../pages/RequestsPage.vue') },
    { path: '/requests/:id', name: 'request-detail', component: () => import('../pages/RequestDetailPage.vue'), props: true },
    { path: '/my-dashboard', name: 'my-dashboard', component: () => import('../pages/MyDashboardPage.vue') },

    // 404
    { path: '/:pathMatch(.*)*', name: 'not-found', component: () => import('../pages/NotFoundPage.vue') },
  ],
})

export default router

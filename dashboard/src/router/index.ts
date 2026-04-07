import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'overview', component: () => import('../pages/OverviewPage.vue') },
    { path: '/workers', name: 'workers', component: () => import('../pages/WorkersPage.vue') },
    { path: '/workers/:operator', name: 'worker-detail', component: () => import('../pages/WorkerDetailPage.vue'), props: true },
    { path: '/models', name: 'models', component: () => import('../pages/ModelsPage.vue') },
    { path: '/models/:id', name: 'model-detail', component: () => import('../pages/ModelDetailPage.vue'), props: true },
    { path: '/requests', name: 'requests', component: () => import('../pages/RequestsPage.vue') },
    { path: '/requests/:id', name: 'request-detail', component: () => import('../pages/RequestDetailPage.vue'), props: true },
    { path: '/my-dashboard', name: 'my-dashboard', component: () => import('../pages/MyDashboardPage.vue') },
    { path: '/:pathMatch(.*)*', name: 'not-found', component: () => import('../pages/NotFoundPage.vue') },
  ],
})

export default router

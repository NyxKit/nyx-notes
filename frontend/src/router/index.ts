import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/composables/useAuth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/views/HomeView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/vaults/:vault_id/notes/:id',
      component: () => import('@/views/NoteView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/vaults/:vault_id/settings',
      component: () => import('@/views/VaultSettingsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/teams/:team_id/settings',
      component: () => import('@/views/TeamSettingsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/login',
      component: () => import('@/views/LoginView.vue'),
    },
  ],
})

router.beforeEach(async (to) => {
  const { authMode, isAuthenticated, discoverMode } = useAuth()

  // Discover auth mode once on first navigation
  if (authMode.value === null) {
    await discoverMode()
  }

  // Local mode: no auth needed, skip login entirely
  if (authMode.value === 'local') return true

  if (to.meta.requiresAuth && !isAuthenticated.value) {
    return { path: '/login', query: { redirect: to.fullPath } }
  }

  if (to.path === '/login' && isAuthenticated.value) {
    return { path: '/' }
  }
})

export default router

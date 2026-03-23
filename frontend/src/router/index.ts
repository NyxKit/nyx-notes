import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/composables/useAuth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/components/AppLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          component: () => import('@/views/HomeView.vue'),
        },
        {
          path: 'vaults/:vault_id',
          component: () => import('@/views/VaultView.vue'),
        },
        {
          path: 'vaults/:vault_id/notes/:id?',
          component: () => import('@/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/favorites',
          component: () => import('@/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/drafts',
          component: () => import('@/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/settings',
          component: () => import('@/views/VaultSettingsView.vue'),
        },
        {
          path: 'teams/:team_id/settings',
          component: () => import('@/views/TeamSettingsView.vue'),
        },
      ],
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

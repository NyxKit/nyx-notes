import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/shared/composables/useAuth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/shared/components/AppLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          component: () => import('@/vaults/views/HomeView.vue'),
        },
        {
          path: 'vaults/:vault_id',
          component: () => import('@/vaults/views/VaultView.vue'),
        },
        {
          path: 'vaults/:vault_id/notes/:id?',
          component: () => import('@/notes/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/favorites',
          component: () => import('@/notes/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/drafts',
          component: () => import('@/notes/views/NoteView.vue'),
        },
        {
          path: 'vaults/:vault_id/settings',
          component: () => import('@/vaults/views/VaultSettingsView.vue'),
        },
        {
          path: 'teams/:team_id/settings',
          component: () => import('@/teams/views/TeamSettingsView.vue'),
        },
      ],
    },
    {
      path: '/login',
      component: () => import('@/auth/views/LoginView.vue'),
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

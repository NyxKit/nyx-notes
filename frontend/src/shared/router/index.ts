import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/auth/composables'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/shared/components').then(({ AppLayout }) => AppLayout),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          component: () => import('@/vaults/views').then(({ HomeView }) => HomeView),
        },
        {
          path: 'vaults/:vault_id',
          component: () => import('@/vaults/views').then(({ VaultView }) => VaultView),
        },
        {
          path: 'vaults/:vault_id/notes/:id?',
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: 'vaults/:vault_id/favorites',
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: 'vaults/:vault_id/drafts',
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: 'vaults/:vault_id/settings',
          component: () => import('@/vaults/views').then(({ VaultSettingsView }) => VaultSettingsView),
        },
        {
          path: 'teams/:team_id/settings',
          component: () => import('@/teams/views').then(({ TeamSettingsView }) => TeamSettingsView),
        },
      ],
    },
    {
      path: '/login',
      component: () => import('@/auth/views').then(({ LoginView }) => LoginView),
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

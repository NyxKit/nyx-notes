import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'

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
          path: 'notes/search',
          component: () => import('@/notes/views').then(({ GlobalSearchView }) => GlobalSearchView),
        },
        {
          path: 'notes/favorites',
          component: () => import('@/notes/views').then(({ FavoritesView }) => FavoritesView),
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
          redirect: '/notes/favorites',
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
          path: 'servers',
          component: () => import('@/servers/views').then(({ ServersView }) => ServersView),
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
  const { activeProfile, setActiveProfile } = useWorkspaceProfiles()
  const { authMode, isAuthenticated, bootstrapActiveProfile } = useAuth()
  let switchedProfile = false

  if (typeof to.query.profile === 'string' && to.query.profile !== activeProfile.value?.id) {
    const nextProfile = setActiveProfile(to.query.profile)
    if (!nextProfile) {
      return { path: '/' }
    }
    switchedProfile = true
  }

  if (!activeProfile.value) {
    if (to.path === '/login') return true
    return { path: '/login', query: { redirect: to.fullPath } }
  }

  if (switchedProfile || authMode.value === null) {
    await bootstrapActiveProfile()
  }

  if (authMode.value === 'local') return true

  if (to.meta.requiresAuth && !isAuthenticated.value) {
    return { path: '/login', query: { redirect: to.fullPath } }
  }

  if (to.path === '/login' && isAuthenticated.value && !('add' in to.query) && !('manage' in to.query)) {
    return { path: '/' }
  }
})

router.afterEach((to) => {
  const { activeProfile, updateProfileRoute } = useWorkspaceProfiles()
  if (!activeProfile.value) return
  if (to.path === '/login') return
  updateProfileRoute(activeProfile.value.id, to.fullPath)
})

export default router

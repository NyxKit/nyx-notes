import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { RouteName } from '@/shared/types/router'

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
          name: RouteName.Home,
          component: () => import('@/vaults/views').then(({ HomeView }) => HomeView),
        },
        {
          path: 'search',
          name: RouteName.Search,
          component: () => import('@/notes/views').then(({ GlobalSearchView }) => GlobalSearchView),
        },
        {
          path: 'favorites',
          name: RouteName.Favorites,
          component: () => import('@/notes/views').then(({ FavoritesView }) => FavoritesView),
        },
        {
          path: 'vaults/:vault_id',
          name: RouteName.Vault,
          component: () => import('@/vaults/views').then(({ VaultView }) => VaultView),
        },
        {
          path: 'vaults/:vault_id/notes/:id?',
          name: RouteName.Note,
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: 'vaults/:vault_id/settings',
          name: RouteName.VaultSettings,
          component: () => import('@/vaults/views').then(({ VaultSettingsView }) => VaultSettingsView),
        },
        {
          path: 'servers',
          name: RouteName.Servers,
          component: () => import('@/servers/views').then(({ ServersView }) => ServersView),
        },
        {
          path: 'teams/:team_id/settings',
          name: RouteName.TeamSettings,
          component: () => import('@/teams/views').then(({ TeamSettingsView }) => TeamSettingsView),
        },
      ],
    },
    {
      path: '/login',
      name: RouteName.Login,
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

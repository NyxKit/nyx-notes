import { createRouter, createWebHistory } from 'vue-router'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { RouteName, RoutePath, RouteQueryKey } from '@/shared/types/router'
import { AuthMode, ServerRole } from '@/shared/types'

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
          path: RoutePath.Search,
          name: RouteName.Search,
          component: () => import('@/notes/views').then(({ GlobalSearchView }) => GlobalSearchView),
        },
        {
          path: RoutePath.Favorites,
          name: RouteName.Favorites,
          component: () => import('@/notes/views').then(({ FavoritesView }) => FavoritesView),
        },
        {
          path: ':server_slug/feedback',
          name: RouteName.Feedback,
          component: () => import('@/feedback/views').then(({ FeedbackVaultView }) => FeedbackVaultView),
        },
        {
          path: ':server_slug/feedback/:note_id',
          name: RouteName.FeedbackNote,
          component: () => import('@/feedback/views').then(({ FeedbackNoteView }) => FeedbackNoteView),
        },
        {
          path: ':server_slug/homes/:home_slug',
          name: RouteName.UserRoot,
          component: () => import('@/vaults/views').then(({ RootView }) => RootView),
          props: { scope: 'personal' },
        },
        {
          path: ':server_slug/vaults',
          name: RouteName.ServerRoot,
          component: () => import('@/vaults/views').then(({ RootView }) => RootView),
          props: { scope: 'server' },
        },
        {
          path: ':server_slug/homes/:home_slug/:vault_id',
          name: RouteName.UserVault,
          component: () => import('@/vaults/views').then(({ VaultView }) => VaultView),
        },
        {
          path: ':server_slug/vaults/:vault_id',
          name: RouteName.ServerVault,
          component: () => import('@/vaults/views').then(({ VaultView }) => VaultView),
        },
        {
          path: ':server_slug/homes/:home_slug/:vault_id/:note_id',
          name: RouteName.UserNote,
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: ':server_slug/vaults/:vault_id/:note_id',
          name: RouteName.ServerNote,
          component: () => import('@/notes/views').then(({ NoteView }) => NoteView),
        },
        {
          path: ':server_slug/homes/:home_slug/:vault_id/settings',
          name: RouteName.UserVaultSettings,
          component: () => import('@/vaults/views').then(({ VaultSettingsView }) => VaultSettingsView),
        },
        {
          path: ':server_slug/vaults/:vault_id/settings',
          name: RouteName.ServerVaultSettings,
          component: () => import('@/vaults/views').then(({ VaultSettingsView }) => VaultSettingsView),
        },
        {
          path: RoutePath.Settings,
          name: RouteName.Settings,
          component: () => import('@/settings/views').then(({ SettingsView }) => SettingsView),
        },
        {
          path: RoutePath.Users,
          name: RouteName.Users,
          component: () => import('@/users/views').then(({ UsersView }) => UsersView),
        },
      ],
    },
    {
      path: RoutePath.Login,
      name: RouteName.Login,
      component: () => import('@/auth/views').then(({ LoginView }) => LoginView),
    },
    {
      path: RoutePath.Setup,
      name: RouteName.Setup,
      component: () => import('@/auth/views').then(({ SetupView }) => SetupView),
    },
  ],
})

router.beforeEach(async (to) => {
  const { activeProfile, setActiveProfile } = useWorkspaceProfiles()
  const { authMode, isAuthenticated, bootstrapActiveProfile, checkInitialized, serverMetadata } = useAuth()
  let switchedProfile = false

  if (typeof to.query[RouteQueryKey.Profile] === 'string' && to.query[RouteQueryKey.Profile] !== activeProfile.value?.id) {
    const nextProfile = setActiveProfile(to.query[RouteQueryKey.Profile])
    if (!nextProfile) {
      return { path: '/' }
    }
    switchedProfile = true
  }

  if (!activeProfile.value) {
    if (to.path === RoutePath.Login) return true
    return { path: RoutePath.Login, query: { [RouteQueryKey.Redirect]: to.fullPath } }
  }

  if (switchedProfile || authMode.value === null) {
    await bootstrapActiveProfile()
  }

  if ((to.name === RouteName.Feedback || to.name === RouteName.FeedbackNote) && serverMetadata.value && serverMetadata.value.role !== ServerRole.Admin) {
    return { path: '/' }
  }

  if (authMode.value === AuthMode.Local) return true

  if (to.path === RoutePath.Login) return true

  if (to.path === RoutePath.Setup) {
    const initialized = await checkInitialized()
    if (initialized) {
      return { path: RoutePath.Login }
    }
    return true
  }

  const initialized = await checkInitialized()
  if (!initialized) {
    return { path: RoutePath.Setup }
  }

  if (to.meta.requiresAuth && !isAuthenticated.value) {
    return { path: RoutePath.Login, query: { [RouteQueryKey.Redirect]: to.fullPath } }
  }

  if (to.path === RoutePath.Login && isAuthenticated.value && !(RouteQueryKey.Add in to.query) && !(RouteQueryKey.Manage in to.query)) {
    return { path: RoutePath.Home }
  }
})

router.afterEach((to) => {
  const { activeProfile, updateProfileRoute } = useWorkspaceProfiles()
  if (!activeProfile.value) return
  if (to.path === RoutePath.Login) return
  updateProfileRoute(activeProfile.value.id, to.fullPath)
})

export default router

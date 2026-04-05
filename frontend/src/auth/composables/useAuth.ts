import { computed, ref } from 'vue'
import { fetchAuthMode, fetchInitialized, login as apiLogin } from '@/auth/api'
import { api } from '@/shared/api'
import {
  getApiRequestEpoch,
  resetApiClientContext,
  setApiProfileContext,
} from '@/shared/api'
import { useWorkspaceProfiles } from '@/shared/composables'
import { readProfilePassword } from '@/shared/utils'
import { RouteName } from '@/shared/types'
import type { AuthMode, ProfileSession, RemoteWorkspaceProfile, ServerMetadata, User } from '@/shared/types'

const SESSION_STORAGE_KEY = 'nyx_profile_sessions'

type StoredSessions = Record<string, ProfileSession>

const authMode = ref<AuthMode | null>(null)
const oidcIssuer = ref<string | null>(null)
const token = ref<string | null>(null)
const currentUser = ref<User | null>(null)
const serverMetadata = ref<ServerMetadata | null>(null)

function slugifyClient(value: string) {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
}
const bootstrapping = ref(false)
const serverInitialized = ref<boolean | null>(null)
const sessions = ref<StoredSessions>(readStoredSessions())
const apiEpoch = ref(getApiRequestEpoch())

function readStoredSessions(): StoredSessions {
  const raw = localStorage.getItem(SESSION_STORAGE_KEY)
  if (!raw) return {}

  try {
    return JSON.parse(raw) as StoredSessions
  } catch {
    return {}
  }
}

function persistSessions() {
  localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(sessions.value))
}

function updateSession(profileId: string, session: Partial<ProfileSession>) {
  const existingSession = sessions.value[profileId]
  sessions.value = {
    ...sessions.value,
    [profileId]: {
      ...(existingSession ?? { profile_id: profileId, state: 'signed_out' }),
      ...session,
    },
  }
  persistSessions()
}

function clearSession(profileId: string) {
  const next = { ...sessions.value }
  delete next[profileId]
  sessions.value = next
  persistSessions()
}

export function useAuth() {
  const workspaceProfiles = useWorkspaceProfiles()
  const { activeProfile, updateRemoteProfileStatus, updateProfileDisplayName } = workspaceProfiles

  const isAuthenticated = computed(() =>
    authMode.value === 'local' || token.value !== null
  )

  const hasProfiles = computed(() => workspaceProfiles.profiles.value.length > 0)
  const activeSession = computed(() => {
    const profileId = activeProfile.value?.id
    return profileId ? sessions.value[profileId] ?? null : null
  })

  function applyApiContext(baseUrl: string | null, nextToken: string | null) {
    setApiProfileContext({ baseUrl, token: nextToken })
    apiEpoch.value = getApiRequestEpoch()
  }

  function classifyConnectionError(error: unknown) {
    if (error instanceof Error && /404|Unexpected token|JSON/i.test(error.message)) {
      return 'invalid_server'
    }

    return 'unreachable'
  }

  async function discoverMode(profile: RemoteWorkspaceProfile) {
    const response = await fetchAuthMode(profile.server_url)

    updateRemoteProfileStatus(profile.id, {
      auth_mode: response.mode,
      server_id: response.server_id,
      server_label: response.server_name,
      api_version: response.api_version,
      connection_status: response.mode === 'secret_key' ? 'reachable' : 'unsupported_auth',
      last_error: response.mode === 'secret_key' ? undefined : 'This remote auth mode is not supported in the multi-profile flow yet.',
    })

    authMode.value = response.mode
    oidcIssuer.value = response.issuer ?? null
    return response
  }

  async function bootstrapActiveProfile() {
    bootstrapping.value = true
    resetApiClientContext()
    token.value = null
    currentUser.value = null
    oidcIssuer.value = null
    authMode.value = null

    const profile = activeProfile.value

    if (!profile) {
      applyApiContext(null, null)
      bootstrapping.value = false
      return null
    }

    if (profile.type === 'local') {
      let response = null
      try {
        response = await fetchAuthMode()
      } catch {
        response = null
      }
      authMode.value = response?.mode ?? 'local'
      if (response?.server_name) {
        updateProfileDisplayName(profile.id, response.server_name)
      }

      if (authMode.value === 'local') {
        applyApiContext(null, null)
        await refreshServerMetadata()
        bootstrapping.value = false
        return profile
      }

      const storedSession = sessions.value[profile.id]
      if (storedSession?.token) {
        token.value = storedSession.token
        applyApiContext(null, storedSession.token)
        await refreshServerMetadata()
        updateSession(profile.id, {
          state: 'signed_in',
          auth_mode: response?.mode ?? 'secret_key',
        })
        bootstrapping.value = false
        return profile
      }

      const password = readProfilePassword(profile.id)
      const storedUsername = storedSession?.username
      if (password && storedUsername) {
        await login(storedUsername, password)
      } else {
        applyApiContext(null, null)
        updateSession(profile.id, {
          state: 'signed_out',
          auth_mode: response?.mode ?? 'secret_key',
          last_error: undefined,
        })
      }
      await refreshServerMetadata()
      bootstrapping.value = false
      return profile
    }

    applyApiContext(profile.server_url, null)
    updateSession(profile.id, { state: 'probing', auth_mode: profile.auth_mode })

    try {
      const response = await discoverMode(profile)

      if (response.mode !== 'secret_key') {
        updateSession(profile.id, {
          state: 'error',
          auth_mode: response.mode,
          token: undefined,
          last_error: 'Unsupported auth mode for remote profiles',
        })
        bootstrapping.value = false
        return profile
      }

      const storedSession = sessions.value[profile.id]
      if (storedSession?.token) {
        token.value = storedSession.token
        applyApiContext(profile.server_url, storedSession.token)
        await refreshServerMetadata()
        updateSession(profile.id, {
          state: 'signed_in',
          auth_mode: response.mode,
        })
        bootstrapping.value = false
        return profile
      }

      const password = readProfilePassword(profile.id)
      if (password) {
        await login(profile.username, password)
      } else {
        updateSession(profile.id, {
          state: 'signed_out',
          auth_mode: response.mode,
          last_error: undefined,
        })
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unable to reach server'
      updateRemoteProfileStatus(profile.id, {
        connection_status: classifyConnectionError(error),
        last_error: message,
      })
      updateSession(profile.id, {
        state: 'error',
        token: undefined,
        last_error: message,
      })
    } finally {
      bootstrapping.value = false
    }

    return profile
  }

  async function login(username: string, password: string, serverUrl?: string) {
    const profile = activeProfile.value
    if (!profile) throw new Error('No active profile')
    const isRemote = profile.type === 'remote'
    
    const targetServerUrl = serverUrl ?? (isRemote ? profile.server_url : null)
    
    if (isRemote) {
      updateSession(profile.id, {
        state: 'signing_in',
        auth_mode: profile.auth_mode ?? authMode.value ?? undefined,
        last_error: undefined,
      })
    }

    try {
      const res = await apiLogin(username, password, targetServerUrl ?? undefined)
      const expiresAt = new Date(Date.now() + res.expires_in * 1000).toISOString()

      token.value = res.token
      applyApiContext(targetServerUrl, res.token)
      await refreshServerMetadata()
      
      if (isRemote) {
        updateRemoteProfileStatus(profile.id, {
          connection_status: 'reachable',
          last_error: undefined,
        })
      }
      updateSession(profile.id, {
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: res.token,
        username,
        expires_at: expiresAt,
        last_error: undefined,
      })
      return res
    } catch (error) {
      token.value = null
      applyApiContext(targetServerUrl, null)
      if (isRemote) {
        updateRemoteProfileStatus(profile.id, {
          connection_status: 'auth_failed',
          last_error: 'Invalid credentials',
        })
      }
      updateSession(profile.id, {
        state: 'signed_out',
        last_error: 'Invalid credentials',
      })
      throw error
    }
  }

  function logout() {
    const profile = activeProfile.value
    token.value = null
    currentUser.value = null
    serverMetadata.value = null
    resetApiClientContext()

    if (!profile) {
      applyApiContext(null, null)
      return
    }

    if (profile.type === 'remote') {
      updateSession(profile.id, {
        state: 'signed_out',
        token: undefined,
        expires_at: undefined,
      })
      updateRemoteProfileStatus(profile.id, {
        last_error: undefined,
      })
      applyApiContext(profile.server_url, null)
      return
    }

    updateSession(profile.id, {
      state: 'signed_out',
      token: undefined,
      expires_at: undefined,
    })
    applyApiContext(null, null)
  }

  function clearProfileSession(profileId: string) {
    clearSession(profileId)
  }

  async function refreshServerMetadata() {
    try {
      serverMetadata.value = await api<ServerMetadata>('/api/server')
    } catch {
      serverMetadata.value = null
    }
  }

  async function checkInitialized() {
    try {
      const res = await fetchInitialized()
      serverInitialized.value = res.initialized
      return res.initialized
    } catch {
      serverInitialized.value = null
      return null
    }
  }

  const personalOverviewRoute = computed(() => {
    const serverSlug = serverMetadata.value?.slug || slugifyClient(activeProfile.value?.display_name ?? 'Main Server')
    const homeSlug = serverMetadata.value?.current_user_username || serverMetadata.value?.current_user_id || currentUser.value?.id

    if (!serverSlug || !homeSlug) {
      return { name: RouteName.Home }
    }
    return {
      name: RouteName.UserRoot,
      params: {
        server_slug: serverSlug,
        home_slug: homeSlug,
      },
    }
  })

  const serverVaultsRoute = computed(() => {
    const serverSlug = serverMetadata.value?.slug || slugifyClient(activeProfile.value?.display_name ?? 'Main Server')

    if (!serverSlug) {
      return { name: RouteName.Home }
    }
    return {
      name: RouteName.ServerRoot,
      params: {
        server_slug: serverSlug,
      },
    }
  })

  return {
    authMode,
    oidcIssuer,
    token,
    currentUser,
    serverMetadata,
    bootstrapping,
    serverInitialized,
    sessions,
    activeSession,
    hasProfiles,
    isAuthenticated,
    apiEpoch,
    bootstrapActiveProfile,
    refreshServerMetadata,
    checkInitialized,
    personalOverviewRoute,
    serverVaultsRoute,
    discoverMode,
    login,
    logout,
    clearProfileSession,
  }
}

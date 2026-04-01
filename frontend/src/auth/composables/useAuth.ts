import { computed, ref } from 'vue'
import { fetchAuthMode, login as apiLogin } from '@/auth/api'
import {
  getApiRequestEpoch,
  resetApiClientContext,
  setApiProfileContext,
} from '@/shared/api'
import { useWorkspaceProfiles } from '@/shared/composables'
import { readProfilePassword } from '@/shared/utils'
import type { AuthMode, ProfileSession, RemoteWorkspaceProfile, User } from '@/shared/types'

const SESSION_STORAGE_KEY = 'nyx_profile_sessions'

type StoredSessions = Record<string, ProfileSession>

const authMode = ref<AuthMode | null>(null)
const oidcIssuer = ref<string | null>(null)
const token = ref<string | null>(null)
const currentUser = ref<User | null>(null)
const bootstrapping = ref(false)
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
  const { activeProfile, updateRemoteProfileStatus } = workspaceProfiles

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
      authMode.value = 'local'
      applyApiContext(null, null)
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

  async function login(username: string, password: string) {
    const profile = activeProfile.value
    if (!profile || profile.type !== 'remote') {
      throw new Error('No active remote profile selected')
    }

    updateSession(profile.id, {
      state: 'signing_in',
      auth_mode: profile.auth_mode ?? authMode.value ?? undefined,
      last_error: undefined,
    })

    try {
      const res = await apiLogin(username, password, profile.server_url)
      const expiresAt = new Date(Date.now() + res.expires_in * 1000).toISOString()

      token.value = res.token
      applyApiContext(profile.server_url, res.token)
      updateRemoteProfileStatus(profile.id, {
        connection_status: 'reachable',
        last_error: undefined,
      })
      updateSession(profile.id, {
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: res.token,
        expires_at: expiresAt,
        last_error: undefined,
      })
      return res
    } catch (error) {
      token.value = null
      applyApiContext(profile.server_url, null)
      updateRemoteProfileStatus(profile.id, {
        connection_status: 'auth_failed',
        last_error: 'Invalid credentials',
      })
      updateSession(profile.id, {
        state: 'error',
        token: undefined,
        last_error: 'Invalid credentials',
      })
      throw error
    }
  }

  function logout() {
    const profile = activeProfile.value
    token.value = null
    currentUser.value = null
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

    clearSession(profile.id)
    applyApiContext(null, null)
  }

  function clearProfileSession(profileId: string) {
    clearSession(profileId)
  }

  return {
    authMode,
    oidcIssuer,
    token,
    currentUser,
    bootstrapping,
    sessions,
    activeSession,
    hasProfiles,
    isAuthenticated,
    apiEpoch,
    bootstrapActiveProfile,
    discoverMode,
    login,
    logout,
    clearProfileSession,
  }
}

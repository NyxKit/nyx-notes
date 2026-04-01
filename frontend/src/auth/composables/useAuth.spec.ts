import { beforeEach, describe, expect, it, vi } from 'vitest'

vi.mock('@/auth/api', () => ({
  fetchAuthMode: vi.fn(),
  login: vi.fn(),
}))

describe('useAuth', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.resetModules()
    vi.clearAllMocks()
  })

  it('boots a local profile without requiring login', async () => {
    const { useWorkspaceProfiles } = await import('@/shared/composables')
    const { useAuth } = await import('./useAuth')

    const profiles = useWorkspaceProfiles()
    const auth = useAuth()

    profiles.createLocalProfile()
    await auth.bootstrapActiveProfile()

    expect(auth.authMode.value).toBe('local')
    expect(auth.isAuthenticated.value).toBe(true)
  })

  it('auto-signs into a saved remote profile with stored credentials', async () => {
    const authApi = await import('@/auth/api')
    vi.mocked(authApi.fetchAuthMode).mockResolvedValue({ mode: 'secret_key' })
    vi.mocked(authApi.login).mockResolvedValue({ token: 'remote-token', expires_in: 60 })

    const { useWorkspaceProfiles } = await import('@/shared/composables')
    const { useAuth } = await import('./useAuth')

    const profiles = useWorkspaceProfiles()
    const auth = useAuth()

    profiles.addRemoteProfile({
      display_name: 'Work NAS',
      server_url: 'https://notes.example.com',
      username: 'alice',
      password: 'top-secret',
    })

    await auth.bootstrapActiveProfile()

    expect(auth.authMode.value).toBe('secret_key')
    expect(auth.token.value).toBe('remote-token')
    expect(auth.isAuthenticated.value).toBe(true)
  })

  it('signs out only the active remote profile session', async () => {
    const authApi = await import('@/auth/api')
    vi.mocked(authApi.fetchAuthMode).mockResolvedValue({ mode: 'secret_key' })
    vi.mocked(authApi.login).mockResolvedValue({ token: 'token-a', expires_in: 60 })

    const { useWorkspaceProfiles } = await import('@/shared/composables')
    const { useAuth } = await import('./useAuth')

    const profiles = useWorkspaceProfiles()
    const auth = useAuth()

    const alpha = profiles.addRemoteProfile({
      display_name: 'Alpha',
      server_url: 'https://alpha.example.com',
      username: 'alice',
      password: 'one',
    })
    await auth.bootstrapActiveProfile()

    const bravo = profiles.addRemoteProfile({
      display_name: 'Bravo',
      server_url: 'https://bravo.example.com',
      username: 'bob',
      password: 'two',
    })

    auth.sessions.value[bravo.id] = {
      profile_id: bravo.id,
      state: 'signed_in',
      auth_mode: 'secret_key',
      token: 'token-b',
    }

    profiles.setActiveProfile(alpha.id)
    auth.logout()

    expect(auth.sessions.value[alpha.id]?.token).toBeUndefined()
    expect(auth.sessions.value[bravo.id]?.token).toBe('token-b')
  })
})

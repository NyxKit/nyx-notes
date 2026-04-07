import { beforeEach, describe, expect, it, vi } from 'vitest'

describe('useWorkspaceProfiles', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.resetModules()
  })

  it('allows the same server URL when usernames differ', async () => {
    const { useWorkspaceProfiles } = await import('./useWorkspaceProfiles')
    const profiles = useWorkspaceProfiles()

    profiles.addRemoteProfile({
      display_name: 'Work',
      server_url: 'notes.example.com',
      username: 'alice',
      password: 'one',
    })

    profiles.addRemoteProfile({
      display_name: 'Review',
      server_url: 'https://notes.example.com/',
      username: 'bob',
      password: 'two',
    })

    expect(profiles.remoteProfiles.value).toHaveLength(2)
  })

  it('rejects duplicates for the same normalized server URL and username', async () => {
    const { useWorkspaceProfiles } = await import('./useWorkspaceProfiles')
    const profiles = useWorkspaceProfiles()

    profiles.addRemoteProfile({
      display_name: 'Work',
      server_url: 'https://notes.example.com',
      username: 'alice',
      password: 'one',
    })

    expect(() => profiles.addRemoteProfile({
      display_name: 'Duplicate',
      server_url: 'notes.example.com/',
      username: 'ALICE',
      password: 'two',
    })).toThrow(/already exists/i)
  })
})

import { beforeEach, describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'
import { RemoteConnectionStatus, WorkspaceProfileType, NotePermission, VaultOwnerType } from '@/shared/types'

const mockProfiles = ref([
  { id: 'local', type: WorkspaceProfileType.Local, display_name: 'Local' },
  {
    id: 'remote-offline',
    type: WorkspaceProfileType.Remote,
    display_name: 'Remote',
    server_url: 'https://remote.example.com',
    username: 'alice',
    connection_status: RemoteConnectionStatus.Unreachable,
  },
])

const mockSessions = ref<Record<string, { state: string; token?: string }>>({})

vi.mock('@/shared/composables', () => ({
  useWorkspaceProfiles: () => ({ profiles: mockProfiles }),
}))

vi.mock('@/auth/composables', () => ({
  useAuth: () => ({ sessions: mockSessions }),
}))

vi.mock('ofetch', () => ({
  ofetch: {
    create: vi.fn(({ baseURL }: { baseURL?: string }) => {
      return async (path: string) => {
        if ((baseURL ?? '/').startsWith('https://remote')) {
          throw new Error('offline')
        }

        if (path === '/api/vaults') {
          return [
            {
              id: 'vault-1',
              slug: 'writing',
              name: 'Writing',
              owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
              permission: NotePermission.Edit,
            },
          ]
        }

        if (path === '/api/vaults/writing/notes') {
          return [
            {
              id: 'note-1',
              vault_id: 'writing',
              title: 'Project plan',
              description: 'Roadmap',
              author_id: 'user-1',
              tags: ['planning'],
              category: null,
              created_at: '2026-04-01T09:00:00Z',
              updated_at: '2026-04-01T10:00:00Z',
              is_encrypted: false,
               permission: NotePermission.Edit,
            },
          ]
        }

        if (path === '/api/vaults/writing/notes/note-1') {
          return {
            meta: {
              id: 'note-1',
              vault_id: 'writing',
              title: 'Project plan',
              description: 'Roadmap',
              author_id: 'user-1',
              tags: ['planning'],
              category: null,
              created_at: '2026-04-01T09:00:00Z',
              updated_at: '2026-04-01T10:00:00Z',
              is_encrypted: false,
               permission: NotePermission.Edit,
            },
            content: 'Body keyword lives here',
          }
        }

        throw new Error(`Unhandled ${path}`)
      }
    }),
  },
}))

describe('useGlobalNoteBrowsing', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    vi.setSystemTime(new Date('2026-04-01T12:00:00Z'))
    mockSessions.value = {}
  })

  it('searches title, tags, and body while counting excluded profiles', async () => {
    const { useGlobalNoteBrowsing } = await import('./useGlobalNoteBrowsing')
    const browsing = useGlobalNoteBrowsing()

    const result = await browsing.loadSearchResults('keyword', [])

    expect(result.excluded_profiles_count).toBe(1)
    expect(result.results).toHaveLength(1)
    expect(result.results[0].server_label).toBe('Local')
    expect(result.results[0].vault_name).toBe('Writing')
  })
})

import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { createNote, fetchNotes } from '@/notes/api'
import { NotePermission, VaultOwnerType } from '@/shared/types'
import { fetchVaults } from '@/vaults/api'
import VaultView from './VaultView.vue'

const push = vi.fn()

vi.mock('vue-router', async () => {
  const actual = await vi.importActual<typeof import('vue-router')>('vue-router')
  return {
    ...actual,
    useRouter: () => ({ push }),
    useRoute: () => ({ params: { vault_id: 'writing' } }),
  }
})

vi.mock('@/vaults/api', () => ({
  fetchVaults: vi.fn(),
  createVault: vi.fn(),
  deleteVault: vi.fn(),
  patchVaultPermission: vi.fn(),
  updateVault: vi.fn(),
}))

vi.mock('@/notes/api', () => ({
  fetchNotes: vi.fn(),
  fetchNote: vi.fn(),
  createNote: vi.fn(),
  updateNote: vi.fn(),
  deleteNote: vi.fn(),
  patchNotePermission: vi.fn(),
}))

describe('VaultView note cards', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    push.mockReset()
    vi.useFakeTimers()
    vi.setSystemTime(new Date('2026-03-26T12:00:00Z'))

    vi.mocked(fetchVaults).mockResolvedValue([
      {
        id: 'vault-1',
        slug: 'writing',
        name: 'Writing',
        description: 'Long-form drafts and essays.',
        owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
        permission: NotePermission.Edit,
        icon: 'folder',
      },
    ])

    vi.mocked(fetchNotes).mockResolvedValue([
      {
        id: 'note-1',
        vault_id: 'writing',
        title: '',
        description: 'The first actual paragraph becomes the card summary.',
        author_id: 'user-1',
        images: [],
        tags: ['focus', 'draft'],
        category: null,
        created_at: '2026-03-26T10:30:00Z',
        updated_at: '2026-03-26T11:00:00Z',
        is_encrypted: false,
        permission: NotePermission.Edit,
      },
    ])

    vi.mocked(createNote).mockResolvedValue({
      id: 'note-2',
      vault_id: 'writing',
      title: '',
      description: undefined,
      author_id: 'user-1',
      images: [],
      tags: [],
      category: null,
      created_at: '2026-03-26T12:00:00Z',
      updated_at: '2026-03-26T12:00:00Z',
      is_encrypted: false,
      permission: NotePermission.Edit,
    })
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('renders note metadata and distilled description in the dedicated note card', async () => {
    const wrapper = mount(VaultView, {
      global: {
        stubs: {
          VaultIcon: true,
          RouterLink: {
            props: ['to'],
            template: '<a :href="`/${to.params.server_slug}/homes/${to.params.home_slug}/vaults/${to.params.vault_id}/${to.params.id}?profile=local`" :aria-label="$attrs[\'aria-label\']"><slot /></a>',
          },
        },
      },
    })

    await flushPromises()

    expect(wrapper.text()).toContain('Untitled')
    expect(wrapper.text()).toContain('Main Server')
    expect(wrapper.text()).toContain('Writing')
    expect(wrapper.text()).toContain('The first actual paragraph becomes the card summary.')
    expect(wrapper.text()).toContain('focus')
    expect(wrapper.text()).toContain('draft')
    expect(wrapper.text()).toContain('1h ago')
  })

  it('renders note destinations as anchors so standard link behavior is preserved', async () => {
    const wrapper = mount(VaultView, {
      global: {
        stubs: {
          VaultIcon: true,
          RouterLink: {
            props: ['to'],
            template: '<a :href="`/${to.params.server_slug}/homes/${to.params.home_slug}/vaults/${to.params.vault_id}/${to.params.id}?profile=local`" :aria-label="$attrs[\'aria-label\']"><slot /></a>',
          },
        },
      },
    })

    await flushPromises()

    const link = wrapper.get('a[aria-label="Open Untitled note"]')
    expect(link.attributes('href')).toBe('/main-server/homes/user-1/vaults/writing/note-1?profile=local')
  })
})

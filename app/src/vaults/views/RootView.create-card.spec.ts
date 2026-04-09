import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createVault, fetchPersonalVaults, fetchVaults } from '@/vaults/api'
import { NotePermission, VaultOwnerType } from '@/shared/types'
import RootView from './RootView.vue'

const push = vi.fn()
const replace = vi.fn()

vi.mock('vue-router', async () => {
  const actual = await vi.importActual<typeof import('vue-router')>('vue-router')
  return {
    ...actual,
    useRouter: () => ({ push, replace }),
  }
})

vi.mock('@/vaults/api', () => ({
  fetchVaults: vi.fn(),
  fetchPersonalVaults: vi.fn(),
  fetchServerVaults: vi.fn(),
  createVault: vi.fn(),
  deleteVault: vi.fn(),
  patchVaultPermission: vi.fn(),
  updateVault: vi.fn(),
}))

describe('RootView create card', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    push.mockReset()
    replace.mockReset()
    vi.mocked(fetchVaults).mockResolvedValue([
      {
        id: 'vault-1',
        slug: 'writing',
        name: 'Writing',
        description: 'Fresh drafts and long-form work.',
        owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
        permission: NotePermission.Edit,
        icon: 'folder',
      },
      {
        id: 'vault-2',
        slug: 'archive',
        name: 'Archive',
        description: 'Finished material.',
        owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
        permission: NotePermission.Edit,
        icon: 'folder',
      },
    ])
    vi.mocked(fetchPersonalVaults).mockResolvedValue([
      {
        id: 'vault-1',
        slug: 'writing',
        name: 'Writing',
        description: 'Fresh drafts and long-form work.',
        owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
        permission: NotePermission.Edit,
        icon: 'folder',
      },
      {
        id: 'vault-2',
        slug: 'archive',
        name: 'Archive',
        description: 'Finished material.',
        owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
        permission: NotePermission.Edit,
        icon: 'folder',
      },
    ])
    vi.mocked(createVault).mockResolvedValue({
      id: 'vault-3',
      slug: 'ideas',
      name: 'Ideas',
      description: 'Quick capture for future projects.',
      owner: { type: VaultOwnerType.Home, server_slug: 'main-server', home_slug: 'user-1' },
      permission: NotePermission.Edit,
      icon: 'folder',
    })
  })

  it('shows and cancels the inline create card with description controls', async () => {
    const wrapper = mount(RootView, {
      global: {
        stubs: {
          VaultCard: true,
          VaultIconPicker: true,
        },
      },
    })

    await flushPromises()
    const headerButton = document.querySelector('#layout-header-actions button') as HTMLButtonElement
    headerButton.click()
    await flushPromises()

    expect(wrapper.text()).toContain('Choose a name, slug, description, and icon.')
    expect(wrapper.text()).toContain('Description')

    const buttons = wrapper.findAll('button')
    await buttons[1].trigger('click')

    expect(wrapper.text()).not.toContain('Choose a name, slug, description, and icon.')
  })

  it('submits create-vault values including description and routes to the created vault', async () => {
    const wrapper = mount(RootView, {
      global: {
        stubs: {
          VaultCard: true,
          VaultIconPicker: true,
        },
      },
    })

    await flushPromises()
    const headerButton = document.querySelector('#layout-header-actions button') as HTMLButtonElement
    headerButton.click()
    await flushPromises()

    const inputs = wrapper.findAll('input')
    await inputs[0].setValue('  Ideas  ')
    await inputs[1].setValue('  ideas  ')
    await wrapper.get('textarea').setValue('  Quick capture for future projects.  ')

    await wrapper.get('form').trigger('submit')
    await flushPromises()

    expect(createVault).toHaveBeenCalledWith({
      slug: 'ideas',
      name: 'Ideas',
      description: 'Quick capture for future projects.',
      icon: undefined,
    })
    expect(push).toHaveBeenCalledWith({
      name: 'user-vault',
      params: {
        server_slug: 'main-server',
        home_slug: 'user-1',
        vault_id: 'ideas',
      },
    })
  })
})

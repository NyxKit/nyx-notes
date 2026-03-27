import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createVault, fetchVaults } from '@/vaults/api'
import HomeView from './HomeView.vue'

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
  createVault: vi.fn(),
  deleteVault: vi.fn(),
  patchVaultPermission: vi.fn(),
  updateVault: vi.fn(),
}))

describe('HomeView create card', () => {
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
        owner: { type: 'user', id: 'user-1' },
        permission: 'edit',
        icon: 'folder',
      },
      {
        id: 'vault-2',
        slug: 'archive',
        name: 'Archive',
        description: 'Finished material.',
        owner: { type: 'user', id: 'user-1' },
        permission: 'edit',
        icon: 'folder',
      },
    ])
    vi.mocked(createVault).mockResolvedValue({
      id: 'vault-3',
      slug: 'ideas',
      name: 'Ideas',
      description: 'Quick capture for future projects.',
      owner: { type: 'user', id: 'user-1' },
      permission: 'edit',
      icon: 'folder',
    })
  })

  it('shows and cancels the inline create card with description controls', async () => {
    const wrapper = mount(HomeView, {
      global: {
        stubs: {
          VaultCard: true,
          VaultIconPicker: true,
        },
      },
    })

    await flushPromises()
    await wrapper.get('button').trigger('click')

    expect(wrapper.text()).toContain('Choose a name, slug, description, and icon.')
    expect(wrapper.text()).toContain('Description')

    const buttons = wrapper.findAll('button')
    await buttons[2].trigger('click')

    expect(wrapper.text()).not.toContain('Choose a name, slug, description, and icon.')
  })

  it('submits create-vault values including description and routes to the created vault', async () => {
    const wrapper = mount(HomeView, {
      global: {
        stubs: {
          VaultCard: true,
          VaultIconPicker: true,
        },
      },
    })

    await flushPromises()
    await wrapper.get('button').trigger('click')

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
    expect(push).toHaveBeenCalledWith('/vaults/vault-3')
  })
})

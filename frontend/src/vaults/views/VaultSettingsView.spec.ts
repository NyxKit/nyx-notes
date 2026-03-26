import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { fetchVaults, updateVault } from '@/vaults/api'
import VaultSettingsView from './VaultSettingsView.vue'

const push = vi.fn()
const back = vi.fn()

vi.mock('vue-router', async () => {
  const actual = await vi.importActual<typeof import('vue-router')>('vue-router')
  return {
    ...actual,
    useRouter: () => ({ push, back }),
    useRoute: () => ({ params: { vault_id: 'vault-1' } }),
  }
})

vi.mock('@/vaults/api', () => ({
  fetchVaults: vi.fn(),
  createVault: vi.fn(),
  deleteVault: vi.fn(),
  patchVaultPermission: vi.fn(),
  updateVault: vi.fn(),
}))

describe('VaultSettingsView', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    push.mockReset()
    back.mockReset()

    vi.mocked(fetchVaults).mockResolvedValue([
      {
        id: 'vault-1',
        slug: 'writing',
        name: 'Writing',
        description: 'Original description.',
        owner: { type: 'user', id: 'user-1' },
        permission: 'edit',
        icon: 'folder',
      },
    ])

    vi.mocked(updateVault).mockResolvedValue({
      id: 'vault-1',
      slug: 'writing',
      name: 'Writing',
      description: 'Updated description.',
      owner: { type: 'user', id: 'user-1' },
      permission: 'edit',
      icon: 'folder',
    })
  })

  it('saves updated vault description from settings', async () => {
    const wrapper = mount(VaultSettingsView, {
      global: {
        stubs: {
          VaultIconPicker: true,
        },
      },
    })

    await flushPromises()

    const fields = wrapper.findAll('textarea, input')
    await fields[1].setValue('Updated description.')
    await wrapper.get('button.gradient-primary').trigger('click')
    await flushPromises()

    expect(updateVault).toHaveBeenCalledWith('vault-1', {
      name: 'Writing',
      description: 'Updated description.',
    })
    expect(wrapper.text()).toContain('Saved')
  })
})

import { flushPromises, mount } from '@vue/test-utils'
import { ref } from 'vue'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import UsersView from './UsersView.vue'

const load = vi.fn()
const create = vi.fn()
const update = vi.fn()
const remove = vi.fn()

vi.mock('@/auth/composables', () => ({
  useAuth: () => ({
    authMode: ref('secret_key'),
    serverMetadata: ref({ current_user_id: 'admin-1', role: 'admin' }),
  }),
}))

vi.mock('@/users/composables', () => ({
  useUsers: () => ({
    users: ref([
      {
        id: 'admin-1',
        username: 'admin',
        email: 'admin@localhost',
        display_name: 'Admin',
        role: 'admin',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        can_edit: true,
        can_delete: false,
      },
    ]),
    loading: ref(false),
    saving: ref(false),
    deleting: ref(null),
    error: ref(null),
    load,
    create,
    update,
    remove,
  }),
}))

describe('UsersView', () => {
  beforeEach(() => {
    load.mockReset()
  })

  it('loads users and renders the table for secret_key admins', async () => {
    const wrapper = mount(UsersView, {
      global: {
        stubs: {
          Teleport: true,
          CreateEditUser: true,
        },
      },
    })

    await flushPromises()

    expect(load).toHaveBeenCalled()
    expect(wrapper.text()).toContain('Users')
    expect(wrapper.text()).toContain('Add user')
  })
})

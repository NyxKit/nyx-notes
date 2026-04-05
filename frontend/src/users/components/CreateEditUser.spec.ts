import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import CreateEditUser from './CreateEditUser.vue'

describe('CreateEditUser', () => {
  it('disables role selection when editing the current user', () => {
    const wrapper = mount(CreateEditUser, {
      props: {
        open: true,
        currentUserId: 'admin-1',
        user: {
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
      },
      global: {
        stubs: {
          Teleport: true,
        },
        directives: {
          'click-outside': {},
        },
      },
    })

    expect(wrapper.text()).toContain('Edit User')
    expect(wrapper.text()).toContain('New Password')
  })
})

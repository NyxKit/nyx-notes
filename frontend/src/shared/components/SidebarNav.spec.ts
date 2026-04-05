import { mount } from '@vue/test-utils'
import { computed, ref } from 'vue'
import { describe, expect, it, vi } from 'vitest'
import SidebarNav from './SidebarNav.vue'

vi.mock('vue-router', async () => {
  const actual = await vi.importActual<typeof import('vue-router')>('vue-router')
  return {
    ...actual,
    useRoute: () => ({ name: 'favorites' }),
  }
})

vi.mock('@/auth/composables', () => ({
  useAuth: () => ({
    personalOverviewRoute: computed(() => ({ name: 'user-root' })),
    serverVaultsRoute: computed(() => ({ name: 'server-root' })),
  }),
}))

vi.mock('@/shared/composables', () => ({
  useWorkspaceProfiles: () => ({
    activeProfile: ref({ display_name: 'Main Server' }),
  }),
}))

describe('SidebarNav', () => {
  it('renders the workspace navigation items', () => {
    const wrapper = mount(SidebarNav, {
      global: {
        stubs: {
          RouterLink: {
            props: ['to'],
            template: '<a><slot /></a>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('Personal')
    expect(wrapper.text()).toContain('Main Server')
    expect(wrapper.text()).toContain('Favorites')
  })
})

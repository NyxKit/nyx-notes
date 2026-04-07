import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import GlobalNoteBrowseView from './GlobalNoteBrowseView.vue'

describe('GlobalNoteBrowseView', () => {
  it('renders the shared empty state and excluded-profile notice', () => {
    const wrapper = mount(GlobalNoteBrowseView, {
      props: {
        title: 'Search',
        notes: [],
        loading: false,
        emptyTitle: 'No matching notes',
        emptyMessage: 'Try another phrase.',
        excludedProfilesCount: 2,
        sortModelValue: 'recent',
        sortOptions: [
          { label: 'Recent', value: 'recent' },
        ],
      },
      global: {
        stubs: {
          NyxGrid: { template: '<div><slot /></div>' },
          NyxIcon: true,
          NyxSelect: { template: '<div class="sort-select" />' },
          NoteCard: true,
          RouterLink: true,
        },
      },
    })

    expect(wrapper.text()).toContain('2 profiles excluded.')
    expect(wrapper.text()).toContain('No matching notes')
    expect(wrapper.text()).toContain('Try another phrase.')
  })
})

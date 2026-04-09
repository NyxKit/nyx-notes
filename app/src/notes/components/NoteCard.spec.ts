import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import NoteCard from './NoteCard.vue'

describe('NoteCard', () => {
  it('renders origin labels and profile-aware destination links', () => {
    const wrapper = mount(NoteCard, {
      props: {
        note: {
          note_id: 'note-1',
          vault_id: 'vault-1',
          profile_id: 'profile-1',
          title: 'Origin test',
          description: 'Card body',
          tags: ['alpha'],
          images: [],
          updated_at: '2026-04-01T10:00:00Z',
          updated_label: '2h ago',
          href: '/vaults/vault-1/notes/note-1?profile=profile-1',
          server_label: 'Local',
          vault_name: 'Writing',
          vault_slug: 'writing',
          is_favorite: false,
        },
      },
      global: {
        stubs: {
          RouterLink: {
            props: ['to'],
            template: '<a :href="to"><slot /></a>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('Local')
    expect(wrapper.text()).toContain('Writing')
    expect(wrapper.get('a').attributes('href')).toBe('/vaults/vault-1/notes/note-1?profile=profile-1')
  })
})

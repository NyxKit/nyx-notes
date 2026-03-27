import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import type { Vault } from '@/shared/types'
import VaultCard from './VaultCard.vue'

describe('VaultCard', () => {
  const vault: Vault = {
    id: 'vault-1',
    slug: 'research-notes',
    name: 'Research Notes',
    description: 'Drafts, references, and interview transcripts.',
    owner: { type: 'user', id: 'user-1' },
    permission: 'edit',
    icon: 'folder',
  }

  it('renders vault identity details inside a link-based card', () => {
    const wrapper = mount(VaultCard, {
      props: { modelValue: vault },
      global: {
        stubs: {
          RouterLink: {
            props: ['to'],
            template: '<a :href="to"><slot /></a>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('Research Notes')
    expect(wrapper.text()).toContain('research-notes')
    expect(wrapper.text()).toContain('Drafts, references, and interview transcripts.')
  })

  it('uses a router-link anchor so standard link behavior is preserved', () => {
    const wrapper = mount(VaultCard, {
      props: { modelValue: vault },
      global: {
        stubs: {
          RouterLink: {
            props: ['to'],
            template: '<a :href="to" :aria-label="$attrs[\'aria-label\']"><slot /></a>',
          },
        },
      },
    })

    const link = wrapper.get('a[aria-label="Open Research Notes"]')
    expect(link.attributes('href')).toBe('/vaults/vault-1')
  })
})

import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { defineComponent, nextTick, ref } from 'vue'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { AuthMode, NotePermission, ServerRole } from '@/shared/types'

vi.mock('@/auth/composables', () => ({
  useAuth: () => ({
    authMode: ref(AuthMode.SecretKey),
    currentUser: ref(null),
    serverMetadata: ref({
      id: 'server-1',
      slug: 'main-server',
      name: 'Main Server',
      current_user_id: 'user-1',
      current_user_username: 'alice',
      role: ServerRole.User,
    }),
  }),
}))

vi.mock('nyx-kit/components', () => ({
  NyxEditor: defineComponent({
    name: 'NyxEditor',
    props: ['disabled', 'modelValue'],
    template: '<div class="nyx-editor" :data-disabled="disabled" />',
  }),
}))

describe('NoteEditor', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('keeps a home-vault note editable for the current server user', async () => {
    const { useNotesStore, useEditorStore } = await import('@/notes/stores')
    const { default: NoteEditor } = await import('./NoteEditor.vue')

    useNotesStore()
    useEditorStore()

    const wrapper = mount(NoteEditor, {
      props: {
        note: {
          meta: {
            id: 'note-1',
            vault_id: 'writing',
            title: 'Draft',
            author_id: 'user-1',
            images: [],
            tags: [],
            category: null,
            created_at: '2026-04-01T09:00:00Z',
            updated_at: '2026-04-01T09:00:00Z',
            is_encrypted: false,
            permission: NotePermission.Edit,
          },
          content: 'Hello world',
        },
      },
      global: {
        stubs: {
          NoteToolbar: true,
        },
      },
    })

    await nextTick()

    expect(wrapper.find('.nyx-editor').attributes('data-disabled')).toBe('false')
  })

})

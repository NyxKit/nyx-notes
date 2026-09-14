import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { Note, NoteMeta } from '@/shared/types'
import type { CreateNoteRequest, UpdateNoteRequest, NotePermission } from '@/shared/types'

export const useNotesStore = defineStore('notes', () => {
  const notesByVault = ref<Record<string, NoteMeta[]>>({})
  const activeNote = ref<Note | null>(null)
  const saving = ref(false)
  const error = ref<string | null>(null)

  function notesFor(vaultId: string): NoteMeta[] {
    return notesByVault.value[vaultId] ?? []
  }

  async function create(vaultId: string, body: CreateNoteRequest) {
    void vaultId
    void body
    return null as unknown as NoteMeta
  }

  async function save(vaultId: string, id: string, body: UpdateNoteRequest) {
    void vaultId
    void id
    void body
    return null as unknown as NoteMeta
  }

  async function remove(vaultId: string, id: string) {
    void vaultId
    void id
  }

  async function updatePermission(vaultId: string, id: string, permission: NotePermission) {
    void vaultId
    void id
    void permission
    return null as unknown as NoteMeta
  }

  function clearActive() {
    activeNote.value = null
  }

  function $reset() {
    notesByVault.value = {}
    activeNote.value = null
    saving.value = false
    error.value = null
  }

  return {
    notesByVault,
    activeNote,
    saving,
    error,
    notesFor,
    create,
    save,
    remove,
    updatePermission,
    clearActive,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useNotesStore, import.meta.hot)

import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import {
  fetchNotes,
  fetchNote,
  createNote,
  updateNote,
  deleteNote,
  patchNotePermission,
} from '@/api/notes'
import type { Note, NoteMeta, CreateNoteRequest, UpdateNoteRequest, NotePermission } from '@/types'

export const useNotesStore = defineStore('notes', () => {
  const notesByVault = ref<Record<string, NoteMeta[]>>({})
  const activeNote = ref<Note | null>(null)
  const listLoading = ref(false)
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)

  function notesFor(vaultId: string): NoteMeta[] {
    return notesByVault.value[vaultId] ?? []
  }

  async function loadAll(vaultIds: string[]) {
    const results = await Promise.allSettled(
      vaultIds.map(id => fetchNotes(id).then(notes => ({ id, notes })))
    )
    for (const r of results) {
      if (r.status === 'fulfilled') notesByVault.value[r.value.id] = r.value.notes
    }
  }

  async function loadList(vaultId: string) {
    listLoading.value = true
    error.value = null
    try {
      notesByVault.value[vaultId] = await fetchNotes(vaultId)
    } catch (e) {
      error.value = String(e)
    } finally {
      listLoading.value = false
    }
  }

  async function loadNote(vaultId: string, id: string) {
    loading.value = true
    error.value = null
    try {
      activeNote.value = await fetchNote(vaultId, id)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function create(vaultId: string, body: CreateNoteRequest) {
    const meta = await createNote(vaultId, body)
    if (!notesByVault.value[vaultId]) notesByVault.value[vaultId] = []
    notesByVault.value[vaultId] = [meta, ...notesByVault.value[vaultId]]
    return meta
  }

  async function save(vaultId: string, id: string, body: UpdateNoteRequest) {
    saving.value = true
    try {
      const meta = await updateNote(vaultId, id, body)
      const list = notesByVault.value[vaultId]
      if (list) {
        const idx = list.findIndex(n => n.id === id)
        if (idx !== -1) list[idx] = meta
      }
      if (activeNote.value?.meta.id === id) {
        activeNote.value = { meta, content: body.content }
      }
      return meta
    } finally {
      saving.value = false
    }
  }

  async function remove(vaultId: string, id: string) {
    await deleteNote(vaultId, id)
    const list = notesByVault.value[vaultId]
    if (list) notesByVault.value[vaultId] = list.filter(n => n.id !== id)
    if (activeNote.value?.meta.id === id) activeNote.value = null
  }

  async function updatePermission(vaultId: string, id: string, permission: NotePermission) {
    const meta = await patchNotePermission(vaultId, id, { permission })
    const list = notesByVault.value[vaultId]
    if (list) {
      const idx = list.findIndex(n => n.id === id)
      if (idx !== -1) list[idx] = meta
    }
    if (activeNote.value?.meta.id === id) {
      activeNote.value = { ...activeNote.value, meta }
    }
    return meta
  }

  function $reset() {
    notesByVault.value = {}
    activeNote.value = null
    listLoading.value = false
    loading.value = false
    saving.value = false
    error.value = null
  }

  return {
    notesByVault,
    activeNote,
    listLoading,
    loading,
    saving,
    error,
    notesFor,
    loadAll,
    loadList,
    loadNote,
    create,
    save,
    remove,
    updatePermission,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useNotesStore, import.meta.hot)

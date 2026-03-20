import { ref } from 'vue'
import {
  fetchNotes,
  fetchNote,
  createNote,
  updateNote,
  deleteNote,
  patchNotePermission,
} from '@/api/notes'
import type { Note, NoteMeta, CreateNoteRequest, UpdateNoteRequest, NotePermission } from '@/types'

const notes = ref<NoteMeta[]>([])
const activeNote = ref<Note | null>(null)
const listLoading = ref(false)
const loading = ref(false)
const saving = ref(false)
const error = ref<string | null>(null)

export function useNotes() {
  async function loadList(vaultId: string) {
    listLoading.value = true
    error.value = null
    try {
      notes.value = await fetchNotes(vaultId)
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
    notes.value.unshift(meta)
    return meta
  }

  async function save(vaultId: string, id: string, body: UpdateNoteRequest) {
    saving.value = true
    try {
      const meta = await updateNote(vaultId, id, body)
      const idx = notes.value.findIndex(n => n.id === id)
      if (idx !== -1) notes.value[idx] = meta
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
    notes.value = notes.value.filter(n => n.id !== id)
    if (activeNote.value?.meta.id === id) activeNote.value = null
  }

  async function updatePermission(vaultId: string, id: string, permission: NotePermission) {
    const meta = await patchNotePermission(vaultId, id, { permission })
    const idx = notes.value.findIndex(n => n.id === id)
    if (idx !== -1) notes.value[idx] = meta
    if (activeNote.value?.meta.id === id) {
      activeNote.value = { ...activeNote.value, meta }
    }
    return meta
  }

  return {
    notes,
    activeNote,
    listLoading,
    loading,
    saving,
    error,
    loadList,
    loadNote,
    create,
    save,
    remove,
    updatePermission,
  }
}

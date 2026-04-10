import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { getApiRequestEpoch, NyxBase } from '@/shared/api'
import {
  fetchNotes,
  fetchNote,
  createNote,
  updateNote,
  deleteNote,
  patchNotePermission,
} from '@/notes/api'
import { Note, NoteMeta } from '@/shared/types'
import type { CreateNoteRequest, UpdateNoteRequest, NotePermission } from '@/shared/types'

export const useNotesStore = defineStore('notes', () => {
  const notesByVault = ref<Record<string, NoteMeta[]>>({})
  const activeNote = ref<Note | null>(null)
  const listLoading = ref(false)
  const loading = ref(false)
  const listSubscriptionKeys = ref<Record<string, string>>({})
  const noteSubscriptionKey = ref<string | null>(null)
  const saving = ref(false)
  const error = ref<string | null>(null)

  function notesFor(vaultId: string): NoteMeta[] {
    return notesByVault.value[vaultId] ?? []
  }

  async function loadAll(vaultIds: string[]) {
    const requestEpoch = getApiRequestEpoch()
    const results = await Promise.allSettled(
      vaultIds.map(id => fetchNotes(id).then(notes => ({ id, notes })))
    )
    if (requestEpoch !== getApiRequestEpoch()) return
    for (const r of results) {
      if (r.status === 'fulfilled') notesByVault.value[r.value.id] = r.value.notes
    }
  }

  async function loadList(vaultId: string) {
    const requestEpoch = getApiRequestEpoch()
    const hasCachedList = Object.prototype.hasOwnProperty.call(notesByVault.value, vaultId)
    listLoading.value = !hasCachedList
    error.value = null
    try {
      const notes = await fetchNotes(vaultId)
      if (requestEpoch !== getApiRequestEpoch()) return
      notesByVault.value[vaultId] = notes
    } catch (e) {
      error.value = String(e)
    } finally {
      listLoading.value = false
    }
  }

  function subscribeList(serverSlug: string, vaultId: string) {
    const handle = NyxBase.subscribe<NoteMeta[]>(
      NyxBase.createNoteListQuery(serverSlug, vaultId),
      snapshot => {
        notesByVault.value[vaultId] = snapshot
      },
    )
    listSubscriptionKeys.value[vaultId] = handle.key
    return handle
  }

  async function loadNote(vaultId: string, id: string) {
    const requestEpoch = getApiRequestEpoch()
    loading.value = true
    error.value = null
    try {
      const note = await fetchNote(vaultId, id)
      if (requestEpoch !== getApiRequestEpoch()) return
      activeNote.value = note
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  function subscribeNote(serverSlug: string, vaultId: string, id: string) {
    const handle = NyxBase.subscribe<Note>(
      NyxBase.createNoteQuery(serverSlug, vaultId, id),
      snapshot => {
        activeNote.value = snapshot
      },
    )
    noteSubscriptionKey.value = handle.key
    return handle
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
        activeNote.value = new Note({ meta, content: body.content })
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
      activeNote.value = new Note({ meta, content: activeNote.value.content })
    }
    return meta
  }

  function clearActive() {
    activeNote.value = null
  }

  function $reset() {
    notesByVault.value = {}
    activeNote.value = null
    listLoading.value = false
    loading.value = false
    listSubscriptionKeys.value = {}
    noteSubscriptionKey.value = null
    saving.value = false
    error.value = null
  }

  return {
    notesByVault,
    activeNote,
    listLoading,
    loading,
    listSubscriptionKeys,
    noteSubscriptionKey,
    saving,
    error,
    notesFor,
    loadAll,
    loadList,
    subscribeList,
    loadNote,
    subscribeNote,
    create,
    save,
    remove,
    updatePermission,
    clearActive,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useNotesStore, import.meta.hot)

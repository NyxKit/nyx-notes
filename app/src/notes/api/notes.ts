import { api } from '@/shared/api'
import type {
  CreateNoteRequest,
  UpdateNoteRequest,
  PatchPermissionRequest,
} from '@/shared/types'
import { Note, NoteMeta } from '@/shared/types'

export async function fetchNotes(vaultId: string) {
  const notes = await api<unknown[]>(`/api/vaults/${vaultId}/notes`)
  return notes.map(note => new NoteMeta(note))
}

export async function fetchNote(vaultId: string, id: string) {
  return new Note(await api<unknown>(`/api/vaults/${vaultId}/notes/${id}`))
}

export async function createNote(vaultId: string, body: CreateNoteRequest) {
  return new NoteMeta(await api<unknown>(`/api/vaults/${vaultId}/notes`, {
    method: 'POST',
    body,
  }))
}

export async function updateNote(vaultId: string, id: string, body: UpdateNoteRequest) {
  return new NoteMeta(await api<unknown>(`/api/vaults/${vaultId}/notes/${id}`, {
    method: 'PUT',
    body,
  }))
}

export function deleteNote(vaultId: string, id: string) {
  return api(`/api/vaults/${vaultId}/notes/${id}`, { method: 'DELETE' })
}

export async function patchNotePermission(vaultId: string, id: string, body: PatchPermissionRequest) {
  return new NoteMeta(await api<unknown>(`/api/vaults/${vaultId}/notes/${id}/permission`, {
    method: 'PATCH',
    body,
  }))
}

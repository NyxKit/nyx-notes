import { api } from '@/shared/api'
import type {
  Note,
  NoteMeta,
  CreateNoteRequest,
  UpdateNoteRequest,
  PatchPermissionRequest,
} from '@/shared/types'

export function fetchNotes(vaultId: string) {
  return api<NoteMeta[]>(`/api/vaults/${vaultId}/notes`)
}

export function fetchNote(vaultId: string, id: string) {
  return api<Note>(`/api/vaults/${vaultId}/notes/${id}`)
}

export function createNote(vaultId: string, body: CreateNoteRequest) {
  return api<NoteMeta>(`/api/vaults/${vaultId}/notes`, {
    method: 'POST',
    body,
  })
}

export function updateNote(vaultId: string, id: string, body: UpdateNoteRequest) {
  return api<NoteMeta>(`/api/vaults/${vaultId}/notes/${id}`, {
    method: 'PUT',
    body,
  })
}

export function deleteNote(vaultId: string, id: string) {
  return api(`/api/vaults/${vaultId}/notes/${id}`, { method: 'DELETE' })
}

export function patchNotePermission(vaultId: string, id: string, body: PatchPermissionRequest) {
  return api<NoteMeta>(`/api/vaults/${vaultId}/notes/${id}/permission`, {
    method: 'PATCH',
    body,
  })
}

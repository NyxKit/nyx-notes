import { api } from '@/api/client'
import type { Comment, CommentReply } from '@/types'

export function fetchComments(vaultId: string, noteId: string) {
  return api<Comment[]>(`/api/vaults/${vaultId}/notes/${noteId}/comments`)
}

export function createComment(vaultId: string, noteId: string, body: { quoted_text: string; body: string }) {
  return api<Comment>(`/api/vaults/${vaultId}/notes/${noteId}/comments`, {
    method: 'POST',
    body,
  })
}

export function deleteComment(vaultId: string, noteId: string, commentId: string) {
  return api(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}`, {
    method: 'DELETE',
  })
}

export function patchComment(vaultId: string, noteId: string, commentId: string, resolved: boolean) {
  return api<Comment>(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}`, {
    method: 'PATCH',
    body: { resolved },
  })
}

export function createReply(vaultId: string, noteId: string, commentId: string, body: string) {
  return api<CommentReply>(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}/replies`, {
    method: 'POST',
    body: { body },
  })
}

export function deleteReply(vaultId: string, noteId: string, commentId: string, replyId: string) {
  return api(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}/replies/${replyId}`, {
    method: 'DELETE',
  })
}

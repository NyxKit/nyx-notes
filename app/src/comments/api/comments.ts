import { api } from '@/shared/api'
import type { CommentReply, CreateCommentRequest } from '@/shared/types'
import { Comment } from '@/shared/types'

export async function fetchComments(vaultId: string, noteId: string) {
  const comments = await api<unknown[]>(`/api/vaults/${vaultId}/notes/${noteId}/comments`)
  return comments.map(comment => new Comment(comment))
}

export async function createComment(vaultId: string, noteId: string, body: CreateCommentRequest) {
  return new Comment(await api<unknown>(`/api/vaults/${vaultId}/notes/${noteId}/comments`, {
    method: 'POST',
    body,
  }))
}

export function deleteComment(vaultId: string, noteId: string, commentId: string) {
  return api(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}`, {
    method: 'DELETE',
  })
}

export async function patchComment(vaultId: string, noteId: string, commentId: string, resolved: boolean) {
  return new Comment(await api<unknown>(`/api/vaults/${vaultId}/notes/${noteId}/comments/${commentId}`, {
    method: 'PATCH',
    body: { resolved },
  }))
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

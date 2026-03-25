import { ref } from 'vue'
import {
  fetchComments,
  createComment,
  deleteComment,
  patchComment,
  createReply,
  deleteReply,
} from '@/comments/api/comments'
import type { Comment } from '@/shared/types'

const comments = ref<Comment[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

export function useComments() {
  async function load(vaultId: string, noteId: string) {
    loading.value = true
    error.value = null
    try {
      comments.value = await fetchComments(vaultId, noteId)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  function clear() {
    comments.value = []
  }

  async function addComment(vaultId: string, noteId: string, quotedText: string, body: string) {
    const comment = await createComment(vaultId, noteId, { quoted_text: quotedText, body })
    comments.value.push(comment)
    return comment
  }

  async function removeComment(vaultId: string, noteId: string, commentId: string) {
    await deleteComment(vaultId, noteId, commentId)
    comments.value = comments.value.filter(c => c.id !== commentId)
  }

  async function resolveComment(vaultId: string, noteId: string, commentId: string, resolved: boolean) {
    const updated = await patchComment(vaultId, noteId, commentId, resolved)
    const idx = comments.value.findIndex(c => c.id === commentId)
    if (idx !== -1) comments.value[idx] = updated
    return updated
  }

  async function addReply(vaultId: string, noteId: string, commentId: string, body: string) {
    const reply = await createReply(vaultId, noteId, commentId, body)
    const comment = comments.value.find(c => c.id === commentId)
    if (comment) comment.replies.push(reply)
    return reply
  }

  async function removeReply(vaultId: string, noteId: string, commentId: string, replyId: string) {
    await deleteReply(vaultId, noteId, commentId, replyId)
    const comment = comments.value.find(c => c.id === commentId)
    if (comment) comment.replies = comment.replies.filter(r => r.id !== replyId)
  }

  return {
    comments,
    loading,
    error,
    load,
    clear,
    addComment,
    removeComment,
    resolveComment,
    addReply,
    removeReply,
  }
}

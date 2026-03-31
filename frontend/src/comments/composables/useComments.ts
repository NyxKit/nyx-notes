import { computed, ref } from 'vue'
import { getApiRequestEpoch } from '@/shared/api'
import {
  fetchComments,
  createComment,
  deleteComment,
  patchComment,
  createReply,
  deleteReply,
} from '@/comments/api'
import type { Comment, CreateCommentRequest } from '@/shared/types'
import { sortCommentsByAnchor, toNyxAnnotations } from './useCommentAnnotations'

const comments = ref<Comment[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const activeCommentId = ref<string | null>(null)
const draftComment = ref<CreateCommentRequest | null>(null)
const activeTab = ref<'Open' | 'Resolved'>('Open')

export function useComments() {
  const annotations = computed(() =>
    toNyxAnnotations(
      comments.value,
      activeCommentId.value ?? undefined,
      activeTab.value === 'Resolved' ? 'resolved' : 'open'
    )
  )

  async function load(vaultId: string, noteId: string) {
    const requestEpoch = getApiRequestEpoch()
    loading.value = true
    error.value = null
    try {
      const nextComments = sortCommentsByAnchor(await fetchComments(vaultId, noteId))
      if (requestEpoch !== getApiRequestEpoch()) return
      comments.value = nextComments
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  function clear() {
    comments.value = []
    activeCommentId.value = null
    draftComment.value = null
  }

  function clearLoadedComments() {
    comments.value = []
    activeCommentId.value = null
  }

  async function addComment(vaultId: string, noteId: string, request: CreateCommentRequest) {
    const comment = await createComment(vaultId, noteId, request)
    comments.value = sortCommentsByAnchor([...comments.value, comment])
    return comment
  }

  async function removeComment(vaultId: string, noteId: string, commentId: string) {
    await deleteComment(vaultId, noteId, commentId)
    comments.value = comments.value.filter(c => c.id !== commentId)
  }

  async function resolveComment(vaultId: string, noteId: string, commentId: string, resolved: boolean) {
    const updated = await patchComment(vaultId, noteId, commentId, resolved)
    const idx = comments.value.findIndex(c => c.id === commentId)
    if (idx !== -1) {
      comments.value[idx] = updated
      comments.value = sortCommentsByAnchor(comments.value)
    }
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

  function setActiveComment(commentId: string | null) {
    activeCommentId.value = commentId
  }

  function beginComment(request: CreateCommentRequest) {
    draftComment.value = request
    activeTab.value = 'Open'
  }

  function cancelDraftComment() {
    draftComment.value = null
  }
  async function submitDraftComment(vaultId: string, noteId: string, body: string) {
    if (!draftComment.value) {
      throw new Error('No pending comment anchor')
    }

    const comment = await addComment(vaultId, noteId, {
      ...draftComment.value,
      body,
    })
    draftComment.value = null
    activeCommentId.value = comment.id
    return comment
  }

  return {
    comments,
    activeCommentId,
    activeTab,
    draftComment,
    annotations,
    loading,
    error,
    load,
    clear,
    clearLoadedComments,
    addComment,
    removeComment,
    resolveComment,
    addReply,
    removeReply,
    setActiveComment,
    beginComment,
    cancelDraftComment,
    submitDraftComment,
  }
}

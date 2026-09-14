import { computed, ref } from 'vue'
import type { Comment, CreateCommentRequest } from '@/shared/types'
import { toNyxAnnotations } from './useCommentAnnotations'

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
    void vaultId
    void noteId
    loading.value = false
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
    void vaultId
    void noteId
    void request
    return null as unknown as Comment
  }

  async function removeComment(vaultId: string, noteId: string, commentId: string) {
    void vaultId
    void noteId
    void commentId
  }

  async function resolveComment(vaultId: string, noteId: string, commentId: string, resolved: boolean) {
    void vaultId
    void noteId
    void commentId
    void resolved
    return null as unknown as Comment
  }

  async function addReply(vaultId: string, noteId: string, commentId: string, body: string) {
    void vaultId
    void noteId
    void commentId
    void body
    return null as unknown as Comment['replies'][number]
  }

  async function removeReply(vaultId: string, noteId: string, commentId: string, replyId: string) {
    void vaultId
    void noteId
    void commentId
    void replyId
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

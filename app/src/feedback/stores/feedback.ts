import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { getApiRequestEpoch } from '@/shared/api'
import { createFeedback, deleteFeedback, fetchFeedback, fetchFeedbackList, updateFeedback } from '@/feedback/api'
import type { CreateFeedbackRequest, Note, NoteMeta, UpdateFeedbackRequest } from '@/shared/types'

export const useFeedbackStore = defineStore('feedback', () => {
  const feedbackItems = ref<NoteMeta[]>([])
  const activeFeedback = ref<Note | null>(null)
  const listLoading = ref(false)
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)

  async function loadList() {
    const requestEpoch = getApiRequestEpoch()
    listLoading.value = true
    error.value = null
    try {
      const items = await fetchFeedbackList()
      if (requestEpoch !== getApiRequestEpoch()) return
      feedbackItems.value = items
    } catch (e) {
      error.value = String(e)
    } finally {
      listLoading.value = false
    }
  }

  async function load(id: string) {
    const requestEpoch = getApiRequestEpoch()
    loading.value = true
    error.value = null
    try {
      const feedback = await fetchFeedback(id)
      if (requestEpoch !== getApiRequestEpoch()) return
      activeFeedback.value = feedback
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function create(body: CreateFeedbackRequest) {
    saving.value = true
    try {
      const meta = await createFeedback(body)
      feedbackItems.value = [meta, ...feedbackItems.value]
      return meta
    } finally {
      saving.value = false
    }
  }

  async function save(id: string, body: UpdateFeedbackRequest) {
    saving.value = true
    try {
      const meta = await updateFeedback(id, body)
      const idx = feedbackItems.value.findIndex(item => item.id === id)
      if (idx !== -1) feedbackItems.value[idx] = meta
      if (activeFeedback.value?.meta.id === id) {
        activeFeedback.value = { meta, content: body.description }
      }
      return meta
    } finally {
      saving.value = false
    }
  }

  async function remove(id: string) {
    await deleteFeedback(id)
    feedbackItems.value = feedbackItems.value.filter(item => item.id !== id)
    if (activeFeedback.value?.meta.id === id) activeFeedback.value = null
  }

  function clearActive() {
    activeFeedback.value = null
  }

  function $reset() {
    feedbackItems.value = []
    activeFeedback.value = null
    listLoading.value = false
    loading.value = false
    saving.value = false
    error.value = null
  }

  return {
    feedbackItems,
    activeFeedback,
    listLoading,
    loading,
    saving,
    error,
    loadList,
    load,
    create,
    save,
    remove,
    clearActive,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useFeedbackStore, import.meta.hot)

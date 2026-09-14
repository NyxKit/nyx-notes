import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { FeedbackNote, NoteMeta } from '@/shared/types'
import type { CreateFeedbackRequest, UpdateFeedbackRequest } from '@/shared/types'

export const useFeedbackStore = defineStore('feedback', () => {
  const feedbackItems = ref<NoteMeta[]>([])
  const activeFeedback = ref<FeedbackNote | null>(null)
  const saving = ref(false)
  const error = ref<string | null>(null)

  async function create(body: CreateFeedbackRequest) {
    void body
    return null as unknown as NoteMeta
  }

  async function save(id: string, body: UpdateFeedbackRequest) {
    void id
    void body
    return null as unknown as NoteMeta
  }

  async function remove(id: string) {
    void id
  }

  function clearActive() {
    activeFeedback.value = null
  }

  function $reset() {
    feedbackItems.value = []
    activeFeedback.value = null
    saving.value = false
    error.value = null
  }

  return {
    feedbackItems,
    activeFeedback,
    saving,
    error,
    create,
    save,
    remove,
    clearActive,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useFeedbackStore, import.meta.hot)

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxButton } from 'nyx-kit/components'
import { NyxTheme, NyxVariant } from 'nyx-kit/types'
import { useFeedbackStore } from '@/feedback/stores'
import { useFeedbackSubmissionContext } from '@/feedback/composables'
import { CreateEditNote } from '@/notes/components'
import { ImageShelf } from '@/shared/components'
import { RouteName } from '@/shared/types'

const route = useRoute()
const router = useRouter()
const feedbackStore = useFeedbackStore()
const { activeFeedback, saving } = storeToRefs(feedbackStore)
const { save, remove } = feedbackStore
const { defaultFeedbackRequest } = useFeedbackSubmissionContext()

const title = computed({
  get: () => activeFeedback.value?.meta.title ?? '',
  set: (value: string) => {
    if (!activeFeedback.value) return
    activeFeedback.value.meta.title = value
  },
})

const description = computed({
  get: () => activeFeedback.value?.content ?? activeFeedback.value?.meta.description ?? '',
  set: (value: string) => {
    if (!activeFeedback.value) return
    activeFeedback.value.content = value
    activeFeedback.value.meta.description = value || undefined
  },
})

async function persist() {
  if (!activeFeedback.value) return
  await save(activeFeedback.value.meta.id, {
    ...defaultFeedbackRequest({
      title: title.value,
      description: description.value,
      feedback_type: activeFeedback.value.meta.feedback_type ?? 'feedback',
      app_location: activeFeedback.value.meta.app_location ?? route.fullPath,
      storage_path: activeFeedback.value.meta.storage_path ?? route.path,
      console_output: activeFeedback.value.meta.console_output ?? '',
      interaction_trail: activeFeedback.value.meta.interaction_trail ?? null,
    }),
    images: [],
  })
}

async function deleteItem() {
  if (!activeFeedback.value) return
  const id = activeFeedback.value.meta.id
  await remove(id)
  router.push({ name: RouteName.Feedback, params: { server_slug: route.params.server_slug } })
}
</script>

<template>
<div class="feedback-note-view">
    <main class="feedback-note-view__body">
      <template v-if="activeFeedback">
        <CreateEditNote v-model:title="title" v-model:content="description" />

        <div class="feedback-note-view__meta">
          <div class="feedback-note-view__details">
            <div class="feedback-note-view__detail">Type: {{ activeFeedback.meta.feedback_type ?? 'feedback' }}</div>
            <div class="feedback-note-view__detail">Location: {{ activeFeedback.meta.app_location ?? 'Unknown location' }}</div>
            <div class="feedback-note-view__detail">Storage: {{ activeFeedback.meta.storage_path ?? 'Unknown storage path' }}</div>
            <div class="feedback-note-view__detail feedback-note-view__detail--mono">Console: {{ activeFeedback.meta.console_output ?? '' }}</div>
          </div>

          <ImageShelf :images="activeFeedback.meta.images ?? []" title="Images" />
        </div>
      </template>
    </main>

    <Teleport to="#layout-header-actions" defer>
      <NyxButton :variant="NyxVariant.Soft" :disabled="saving" @click="persist">Save</NyxButton>
      <NyxButton :theme="NyxTheme.Danger" :disabled="saving" @click="deleteItem">Delete</NyxButton>
    </Teleport>
  </div>
</template>

<style scoped>
.feedback-note-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.feedback-note-view__body {
  flex: 1;
  overflow: auto;
  padding: 1.5rem;
}

.feedback-note-view__meta {
  max-width: 768px;
  margin: 1rem auto 0;
  display: grid;
  gap: 1rem;
}

.feedback-note-view__detail {
  padding: 0.75rem 0.875rem;
  border-radius: var(--nyx-radius-md);
  background: var(--nyx-c-bg-soft);
}

.feedback-note-view__detail--mono {
  font-family: monospace;
  white-space: pre-wrap;
}
</style>

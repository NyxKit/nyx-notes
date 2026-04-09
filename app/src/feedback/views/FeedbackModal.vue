<script setup lang="ts">
import { reactive, watch } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxModal, NyxSelect } from 'nyx-kit/components'
import { NyxTheme, NyxVariant } from 'nyx-kit/types'
import { CreateEditNote } from '@/notes/components'
import { FeedbackAttachments } from '@/feedback/components'
import { useFeedbackDialog, useFeedbackSubmissionContext } from '@/feedback/composables'
import { useFeedbackStore } from '@/feedback/stores'
import type { FeedbackImageUpload } from '@/shared/types'

const store = useFeedbackStore()
const { feedbackDialogOpen, closeFeedbackDialog } = useFeedbackDialog()
const { defaultFeedbackRequest } = useFeedbackSubmissionContext()

const form = reactive({
  title: '',
  description: '',
  feedback_type: 'feedback',
  images: [] as FeedbackImageUpload[],
})

watch(
  feedbackDialogOpen,
  (open) => {
    if (!open) return
    form.title = ''
    form.description = ''
    form.feedback_type = 'feedback'
    form.images = []
  },
  { immediate: true }
)

async function submit() {
  await store.create(defaultFeedbackRequest({
    title: form.title,
    description: form.description,
    feedback_type: form.feedback_type,
    images: form.images,
  }))

  closeFeedbackDialog()
}
</script>

<template>
  <NyxModal
    :model-value="feedbackDialogOpen"
    title="Feedback"
    @update:model-value="(open) => { if (!open) closeFeedbackDialog() }"
  >
    <NyxForm class="feedback-modal" @submit.prevent="submit">
      <NyxFormField label="Type">
        <template #default="{ id }">
          <NyxSelect
            :id="id"
            v-model="form.feedback_type"
            :variant="NyxVariant.Soft"
            :options="[
              { label: 'Feedback', value: 'feedback' },
              { label: 'Bug', value: 'bug' },
            ]"
          />
        </template>
      </NyxFormField>

      <CreateEditNote
        v-model:title="form.title"
        v-model:content="form.description"
        title-label="Title"
        content-placeholder="Describe what happened, what you expected, and anything else useful."
      />

      <FeedbackAttachments v-model:images="form.images" />

      <div class="feedback-modal__actions">
        <NyxButton type="button" :variant="NyxVariant.Soft" @click="closeFeedbackDialog">Cancel</NyxButton>
        <NyxButton :gradient="true" :theme="NyxTheme.Primary" type="submit">Submit Feedback</NyxButton>
      </div>
    </NyxForm>
  </NyxModal>
</template>

<style scoped>
.feedback-modal {
  display: grid;
  gap: 1rem;
}

.feedback-modal__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}
</style>

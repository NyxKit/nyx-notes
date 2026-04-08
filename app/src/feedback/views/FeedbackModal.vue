<script setup lang="ts">
import { reactive, watch } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxModal, NyxSelect, NyxTextarea } from 'nyx-kit/components'
import { NyxTheme, NyxVariant } from 'nyx-kit/types'
import { CreateEditNote } from '@/notes/components'
import { FeedbackAttachments } from '@/feedback/components'
import { useFeedbackDialog, useFeedbackSubmissionContext } from '@/feedback/composables'
import { useFeedbackStore } from '@/feedback/stores'
import type { FeedbackImageUpload } from '@/shared/types'

const store = useFeedbackStore()
const { feedbackDialogOpen, closeFeedbackDialog } = useFeedbackDialog()
const { appLocation, storagePath } = useFeedbackSubmissionContext()

const form = reactive({
  title: '',
  description: '',
  feedback_type: 'feedback',
  console_output: '',
  images: [] as FeedbackImageUpload[],
})

watch(
  feedbackDialogOpen,
  (open) => {
    if (!open) return
    form.title = ''
    form.description = ''
    form.feedback_type = 'feedback'
    form.console_output = ''
    form.images = []
  },
  { immediate: true }
)

async function submit() {
  await store.create({
    title: form.title,
    description: form.description,
    feedback_type: form.feedback_type,
    app_location: appLocation.value,
    storage_path: storagePath.value,
    console_output: form.console_output,
    interaction_trail: null,
    images: form.images,
  })

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

      <NyxFormField label="Console Output">
        <template #default="{ id }">
          <NyxTextarea
            :id="id"
            v-model="form.console_output"
            :variant="NyxVariant.Soft"
            placeholder="Paste any useful console output here"
            rows="8"
          />
        </template>
      </NyxFormField>

      <div class="feedback-modal__context">
        <NyxFormField label="App Location">
          <template #default="{ id }">
            <input :id="id" class="feedback-modal__readonly" :value="appLocation" readonly />
          </template>
        </NyxFormField>

        <NyxFormField label="Storage Path">
          <template #default="{ id }">
            <input :id="id" class="feedback-modal__readonly" :value="storagePath" readonly />
          </template>
        </NyxFormField>
      </div>

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

.feedback-modal__context {
  display: grid;
  gap: 0.75rem;
}

.feedback-modal__readonly {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid var(--nyx-c-divider);
  border-radius: var(--nyx-radius-md);
  padding: 0.75rem 0.875rem;
  background: var(--nyx-c-bg-soft);
  color: var(--nyx-c-text-2);
  font: inherit;
}

.feedback-modal__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}
</style>

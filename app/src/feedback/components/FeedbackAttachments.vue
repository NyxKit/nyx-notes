<script setup lang="ts">
import { computed, ref } from 'vue'
import { NyxButton } from 'nyx-kit/components'
import type { FeedbackImageUpload } from '@/shared/types'

const props = defineProps<{
  images: FeedbackImageUpload[]
}>()

const emit = defineEmits<{
  'update:images': [value: FeedbackImageUpload[]]
}>()

const fileInput = ref<HTMLInputElement | null>(null)

const hasImages = computed(() => props.images.length > 0)

function removeImage(index: number) {
  emit('update:images', props.images.filter((_, i) => i !== index))
}

async function addFiles(files: FileList | null) {
  if (!files?.length) return

  const next = [...props.images]
  for (const file of Array.from(files)) {
    const dataUrl = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader()
      reader.onerror = () => reject(reader.error ?? new Error('Unable to read image'))
      reader.onload = () => resolve(String(reader.result ?? ''))
      reader.readAsDataURL(file)
    })

    const [, payload = ''] = dataUrl.split('base64,')
    next.push({
      name: file.name,
      mime_type: file.type || 'image/jpeg',
      data: payload,
    })
  }

  emit('update:images', next)
  if (fileInput.value) fileInput.value.value = ''
}
</script>

<template>
  <div class="feedback-attachments">
    <div class="feedback-attachments__header">
      <span class="feedback-attachments__label">Images</span>
      <NyxButton type="button" @click="fileInput?.click()">Add Images</NyxButton>
      <input
        ref="fileInput"
        class="feedback-attachments__input"
        type="file"
        accept="image/*"
        multiple
        @change="addFiles(($event.target as HTMLInputElement).files)"
      />
    </div>

    <div v-if="hasImages" class="feedback-attachments__list">
      <div v-for="(image, index) in images" :key="`${image.name}-${index}`" class="feedback-attachments__item">
        <span class="feedback-attachments__name">{{ image.name }}</span>
        <NyxButton type="button" @click="removeImage(index)">Remove</NyxButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.feedback-attachments {
  display: grid;
  gap: 0.75rem;
}

.feedback-attachments__header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.feedback-attachments__label {
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--nyx-c-text-3);
}

.feedback-attachments__input {
  display: none;
}

.feedback-attachments__list {
  display: grid;
  gap: 0.5rem;
}

.feedback-attachments__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border-radius: var(--nyx-radius-lg);
  background: var(--nyx-c-bg-soft);
}

.feedback-attachments__name {
  font-size: 0.875rem;
  color: var(--nyx-c-text-2);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>

import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useAuth } from '@/auth/composables'
import type { CreateFeedbackRequest, FeedbackImageUpload } from '@/shared/types'

const feedbackDialogOpen = ref(false)

export function useFeedbackDialog() {
  function openFeedbackDialog() {
    feedbackDialogOpen.value = true
  }

  function closeFeedbackDialog() {
    feedbackDialogOpen.value = false
  }

  return {
    feedbackDialogOpen: computed(() => feedbackDialogOpen.value),
    openFeedbackDialog,
    closeFeedbackDialog,
  }
}

function stripDataUrl(data: string) {
  const match = data.match(/^data:([^;]+);base64,(.*)$/)
  return {
    mime_type: match?.[1] ?? 'image/jpeg',
    data: match?.[2] ?? data,
  }
}

export async function filesToFeedbackUploads(files: File[]) {
  const uploads: FeedbackImageUpload[] = []

  for (const file of files) {
    const dataUrl = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader()
      reader.onerror = () => reject(reader.error ?? new Error('Unable to read image'))
      reader.onload = () => resolve(String(reader.result ?? ''))
      reader.readAsDataURL(file)
    })

    const { mime_type, data } = stripDataUrl(dataUrl)
    uploads.push({
      name: file.name,
      mime_type,
      data,
    })
  }

  return uploads
}

export function useFeedbackSubmissionContext() {
  const route = useRoute()
  const { serverMetadata } = useAuth()

  const appLocation = computed(() => route.fullPath)
  const storagePath = computed(() => route.path)
  const serverLabel = computed(() => serverMetadata.value?.name ?? 'Main Server')

  function defaultFeedbackRequest(overrides: Partial<CreateFeedbackRequest> = {}): CreateFeedbackRequest {
    return {
      title: overrides.title ?? '',
      description: overrides.description ?? '',
      feedback_type: overrides.feedback_type ?? 'feedback',
      app_location: overrides.app_location ?? appLocation.value,
      storage_path: overrides.storage_path ?? storagePath.value,
      console_output: overrides.console_output ?? '',
      interaction_trail: overrides.interaction_trail ?? null,
      images: overrides.images ?? [],
    }
  }

  return {
    appLocation,
    storagePath,
    serverLabel,
    defaultFeedbackRequest,
  }
}

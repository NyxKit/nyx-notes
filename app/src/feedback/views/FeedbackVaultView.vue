<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { NyxButton, NyxIcon } from 'nyx-kit/components'
import { NyxGridMode, NyxTheme } from 'nyx-kit/types'
import { noteRoute } from '@/shared/utils'
import { useAuth } from '@/auth/composables'
import { useFeedbackStore } from '@/feedback/stores'
import { useFeedbackSubmissionContext } from '@/feedback/composables'
import { NotesGrid } from '@/notes/components'
import NoteCard from '@/notes/components/NoteCard.vue'
import { NotePermission, VaultOwnerType } from '@/shared/types'
import type { BrowseNoteCardModel, Vault } from '@/shared/types'

const { serverMetadata } = useAuth()
const feedbackStore = useFeedbackStore()
const { feedbackItems, error } = storeToRefs(feedbackStore)
const { create } = feedbackStore
const { defaultFeedbackRequest } = useFeedbackSubmissionContext()

const feedbackVault = computed<Vault>(() => ({
  id: 'feedback',
  slug: 'feedback',
  name: 'Feedback',
  owner: {
    type: VaultOwnerType.Server,
    server_slug: serverMetadata.value?.slug ?? 'main-server',
  },
  permission: NotePermission.Restricted,
  icon: 'message-circle',
}))

const sortedFeedback = computed<BrowseNoteCardModel[]>(() =>
  feedbackItems.value
    .slice()
    .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
    .map(item => ({
        note_id: item.id,
        vault_id: item.vault_id,
        profile_id: serverMetadata.value?.slug ?? 'main-server',
        title: item.title || 'Untitled',
        description: item.description,
        tags: item.tags,
        images: item.images ?? [],
        updated_at: item.updated_at,
        updated_label: formatDate(item.updated_at),
        href: noteRoute(feedbackVault.value, item.id),
        server_label: serverMetadata.value?.name ?? 'Main Server',
        server_id: serverMetadata.value?.slug,
        vault_name: 'Feedback',
        vault_slug: 'feedback',
        is_favorite: false,
      }))
)

function formatDate(iso: string) {
  const d = new Date(iso)
  const now = Date.now()
  const diff = now - d.getTime()
  const mins = Math.floor(diff / 60_000)
  if (mins < 60) return mins <= 1 ? 'just now' : `${mins}m ago`
  const hrs = Math.floor(mins / 60)
  if (hrs < 24) return `${hrs}h ago`
  const days = Math.floor(hrs / 24)
  if (days < 7) return `${days}d ago`
  return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
}

async function createDraft() {
  await create(defaultFeedbackRequest({
    title: 'Untitled',
    description: '',
  }))
}
</script>

<template>
  <div class="feedback-vault-view">
      <Teleport to="#layout-header-actions" defer>
      <NyxButton :gradient="true" @click="createDraft">New Feedback</NyxButton>
    </Teleport>

    <main class="feedback-vault-view__body">
      <div v-if="sortedFeedback.length > 0" class="feedback-vault-view__canvas feedback-vault-view__canvas--overview">
        <NotesGrid title="Feedback" :vault="feedbackVault" :notes="sortedFeedback" :mode="NyxGridMode.Masonry" :columns="5">
          <template #card="{ note }">
            <NoteCard
              :note="note"
              :theme="note.tags.includes('bug') ? NyxTheme.Danger : NyxTheme.Info"
              :image="note.images?.[0]"
            />
          </template>
        </NotesGrid>
      </div>

      <div v-else-if="error" class="feedback-vault-view__canvas feedback-vault-view__canvas--center">
        <div class="feedback-vault-view__welcome-card">
          <div class="feedback-vault-view__welcome-icon">
            <NyxIcon name="alert-triangle" :size="32" />
          </div>
          <h1 class="feedback-vault-view__heading">Unable to load feedback.</h1>
          <p class="feedback-vault-view__desc">{{ error }}</p>
        </div>
      </div>

      <div v-else class="feedback-vault-view__canvas feedback-vault-view__canvas--center">
        <div class="feedback-vault-view__welcome-card">
          <div class="feedback-vault-view__welcome-icon">
            <NyxIcon name="message-circle" :size="32" />
          </div>
          <h1 class="feedback-vault-view__heading">No feedback yet.</h1>
          <p class="feedback-vault-view__desc">Feedback submitted by users will appear here.</p>
          <NyxButton :gradient="true" @click="createDraft">New Feedback</NyxButton>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.feedback-vault-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.feedback-vault-view__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.feedback-vault-view__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.feedback-vault-view__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.feedback-vault-view__canvas--overview {
  display: flex;
  flex-direction: column;
}

.feedback-vault-view__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.feedback-vault-view__skeleton-card {
  height: 120px;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: feedback-vault-view-pulse 1.4s ease-in-out infinite;
}

@keyframes feedback-vault-view-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}

.feedback-vault-view__welcome-card {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 2.5rem 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: flex-start;
  max-width: 400px;
  width: 100%;
}

.feedback-vault-view__welcome-icon {
  color: var(--nyx-c-primary);
  opacity: 0.7;
  line-height: 0;
}

.feedback-vault-view__heading {
  font-family: 'Manrope', sans-serif;
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1.25;
  color: var(--nyx-c-text-1);
  margin: 0;
}

.feedback-vault-view__desc {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--nyx-c-text-2);
  margin: 0;
}
</style>

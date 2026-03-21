<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useComments } from '@/composables/useComments'
import CommentThread from '@/components/CommentThread.vue'
import CommentComposer from '@/components/CommentComposer.vue'
import type { Note } from '@/types'
import { NyxButton, NyxTabs } from 'nyx-kit/components'
import { NyxVariant, NyxShape } from 'nyx-kit/types'

const props = defineProps<{
  note: Note
}>()

const { authMode, currentUser } = useAuth()
const { comments, loading, load, clear, addComment } = useComments()

const showComposer = ref(false)
const submitting = ref(false)
const activeTab = ref('Open')

const isNoteAuthor = computed(() =>
  authMode.value === 'local' || currentUser.value?.id === props.note.meta.author_id
)

const canComment = computed(() =>
  isNoteAuthor.value || props.note.meta.permission !== 'restricted'
)

const openComments = computed(() => comments.value.filter(c => !c.resolved))
const resolvedComments = computed(() => comments.value.filter(c => c.resolved))

watch(
  () => props.note.meta.id,
  async (noteId) => {
    clear()
    if (noteId) await load(props.note.meta.vault_id, noteId)
  },
  { immediate: true }
)

async function onSubmitComment(body: string) {
  submitting.value = true
  try {
    await addComment(props.note.meta.vault_id, props.note.meta.id, '', body)
    showComposer.value = false
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="comment-sidebar">

    <!-- Header -->
    <div class="comment-sidebar__header">
      <div class="comment-sidebar__header-row">
        <span class="comment-sidebar__title">Review</span>
        <span v-if="openComments.length" class="comment-sidebar__thread-count">
          {{ openComments.length }} active {{ openComments.length === 1 ? 'thread' : 'threads' }}
        </span>
        <NyxButton
          v-if="canComment"
          :variant="NyxVariant.Ghost"
          :shape="NyxShape.Square"
          title="New comment"
          @click="showComposer = !showComposer"
        >
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
            <path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </NyxButton>
      </div>

    </div>

    <NyxTabs v-model="activeTab" :tabs="['Open', 'Resolved']">

      <template #tab-Open>
        <div v-if="loading" class="comment-sidebar__state">Loading…</div>
        <template v-else>
          <!-- New comment composer -->
          <div v-if="showComposer" class="comment-sidebar__composer">
            <CommentComposer
              :submitting="submitting"
              @submit="onSubmitComment"
              @cancel="showComposer = false"
            />
          </div>

          <!-- Open threads -->
          <div v-if="openComments.length" class="comment-sidebar__threads">
            <CommentThread
              v-for="comment in openComments"
              :key="comment.id"
              :comment="comment"
              :vault-id="note.meta.vault_id"
              :note-id="note.meta.id"
              :is-note-author="isNoteAuthor"
            />
          </div>

          <div
            v-if="!openComments.length && !showComposer"
            class="comment-sidebar__state"
          >
            No open comments
          </div>
        </template>
      </template>

      <template #tab-Resolved>
        <div v-if="loading" class="comment-sidebar__state">Loading…</div>
        <div v-else-if="resolvedComments.length" class="comment-sidebar__threads">
          <CommentThread
            v-for="comment in resolvedComments"
            :key="comment.id"
            :comment="comment"
            :vault-id="note.meta.vault_id"
            :note-id="note.meta.id"
            :is-note-author="isNoteAuthor"
          />
        </div>
        <div v-else class="comment-sidebar__state">No resolved comments</div>
      </template>

    </NyxTabs>

  </div>
</template>

<style scoped>
.comment-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

/* Header */
.comment-sidebar__header {
  padding: 1.25rem 1rem 0;
  flex-shrink: 0;
}

.comment-sidebar__header-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.875rem;
}

.comment-sidebar__title {
  font-weight: 700;
  font-size: 0.9375rem;
  color: var(--nyx-c-text-1);
  flex: 1;
}

.comment-sidebar__thread-count {
  font-size: 0.6875rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

/* Composer */
.comment-sidebar__composer {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-c-divider);
}

/* Threads */
.comment-sidebar__threads {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
  padding: 0.875rem 1rem;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

.comment-sidebar__threads--resolved {
  flex: unset;
  overflow-y: unset;
  padding-top: 0.5rem;
}

/* States */
.comment-sidebar__state {
  padding: 2rem 1rem;
  text-align: center;
  font-size: 0.875rem;
  color: var(--nyx-c-text-3);
}

</style>

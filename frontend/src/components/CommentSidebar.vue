<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useComments } from '@/composables/useComments'
import CommentThread from '@/components/CommentThread.vue'
import CommentComposer from '@/components/CommentComposer.vue'
import type { Note } from '@/types'

const props = defineProps<{
  note: Note
}>()

const { authMode, currentUser } = useAuth()
const { comments, loading, load, clear, addComment } = useComments()

const showComposer = ref(false)
const submitting = ref(false)
const activeTab = ref<'comments' | 'history'>('comments')

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
        <button
          v-if="canComment"
          class="comment-sidebar__add-btn"
          title="New comment"
          @click="showComposer = !showComposer"
        >
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
            <path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>

      <!-- Tab bar -->
      <div class="comment-sidebar__tabs">
        <button
          class="comment-sidebar__tab"
          :class="{ 'comment-sidebar__tab--active': activeTab === 'comments' }"
          @click="activeTab = 'comments'"
        >
          Comments
        </button>
        <button
          class="comment-sidebar__tab"
          :class="{ 'comment-sidebar__tab--active': activeTab === 'history' }"
          @click="activeTab = 'history'"
        >
          History
        </button>
      </div>
    </div>

    <div v-if="loading" class="comment-sidebar__state">Loading…</div>

    <template v-else-if="activeTab === 'comments'">

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

      <!-- Resolved threads -->
      <details v-if="resolvedComments.length" class="comment-sidebar__resolved">
        <summary class="comment-sidebar__resolved-label">
          Resolved ({{ resolvedComments.length }})
        </summary>
        <div class="comment-sidebar__threads comment-sidebar__threads--resolved">
          <CommentThread
            v-for="comment in resolvedComments"
            :key="comment.id"
            :comment="comment"
            :vault-id="note.meta.vault_id"
            :note-id="note.meta.id"
            :is-note-author="isNoteAuthor"
          />
        </div>
      </details>

    </template>

    <div v-else-if="activeTab === 'history'" class="comment-sidebar__state">
      History coming soon
    </div>

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

.comment-sidebar__add-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--nyx-c-text-3);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  border-radius: var(--nyx-radius-md);
  transition: background 0.2s, color 0.2s;
  line-height: 0;
  flex-shrink: 0;
}

.comment-sidebar__add-btn:hover {
  background: var(--nyx-c-bg-mute);
  color: var(--nyx-c-text-1);
}

/* Tabs */
.comment-sidebar__tabs {
  display: flex;
  gap: 0;
  border-bottom: 1px solid var(--nyx-c-divider);
}

.comment-sidebar__tab {
  font-size: 0.8125rem;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  margin-bottom: -1px;
  cursor: pointer;
  color: var(--nyx-c-text-3);
  padding: 0.375rem 0.875rem 0.375rem 0;
  font-family: inherit;
  transition: color 0.2s, border-color 0.2s;
}

.comment-sidebar__tab:hover {
  color: var(--nyx-c-text-2);
}

.comment-sidebar__tab--active {
  color: var(--nyx-c-text-1);
  border-bottom-color: var(--nyx-c-primary);
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

/* Resolved section */
.comment-sidebar__resolved {
  flex-shrink: 0;
  border-top: 1px solid var(--nyx-c-divider);
}

.comment-sidebar__resolved-label {
  padding: 0.625rem 1rem;
  font-size: 0.8125rem;
  color: var(--nyx-c-text-3);
  cursor: pointer;
  list-style: none;
}

.comment-sidebar__resolved-label::-webkit-details-marker {
  display: none;
}
</style>

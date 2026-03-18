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
    <div class="comment-sidebar__header">
      <span class="comment-sidebar__title">Comments</span>
      <span v-if="comments.length" class="comment-sidebar__count">{{ openComments.length }}</span>
      <button
        v-if="canComment"
        class="comment-sidebar__new-btn"
        @click="showComposer = !showComposer"
      >
        +
      </button>
    </div>

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

      <!-- Resolved threads (collapsed by default) -->
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
  </div>
</template>

<style scoped>
.comment-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.comment-sidebar__header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
  flex-shrink: 0;
}

.comment-sidebar__title {
  font-weight: 600;
  font-size: 0.875rem;
  flex: 1;
}

.comment-sidebar__count {
  font-size: 0.75rem;
  background: var(--nyx-color-accent, #6366f1);
  color: #fff;
  border-radius: 9999px;
  padding: 0.0625rem 0.4375rem;
  font-weight: 600;
}

.comment-sidebar__new-btn {
  background: none;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.25rem;
  width: 1.5rem;
  height: 1.5rem;
  cursor: pointer;
  font-size: 1rem;
  line-height: 1;
  color: var(--nyx-color-muted, #718096);
  display: flex;
  align-items: center;
  justify-content: center;
}

.comment-sidebar__composer {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.comment-sidebar__threads {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  overflow-y: auto;
  flex: 1;
}

.comment-sidebar__threads--resolved {
  flex: unset;
  overflow-y: unset;
  padding-top: 0.5rem;
}

.comment-sidebar__state {
  padding: 2rem 1rem;
  text-align: center;
  font-size: 0.875rem;
  color: var(--nyx-color-muted, #718096);
}

.comment-sidebar__resolved {
  flex-shrink: 0;
  border-top: 1px solid var(--nyx-color-border, #e2e8f0);
}

.comment-sidebar__resolved-label {
  padding: 0.625rem 1rem;
  font-size: 0.8125rem;
  color: var(--nyx-color-muted, #718096);
  cursor: pointer;
  list-style: none;
}

.comment-sidebar__resolved-label::-webkit-details-marker {
  display: none;
}
</style>

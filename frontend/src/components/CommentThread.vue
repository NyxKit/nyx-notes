<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useComments } from '@/composables/useComments'
import CommentComposer from '@/components/CommentComposer.vue'
import type { Comment } from '@/types'

const props = defineProps<{
  comment: Comment
  vaultId: string
  noteId: string
  isNoteAuthor: boolean
}>()

const { currentUser, authMode } = useAuth()
const { resolveComment, removeComment, addReply, removeReply } = useComments()

const showReplyComposer = ref(false)
const submittingReply = ref(false)

const isAuthor = (authorId: string) =>
  authMode.value === 'local' || currentUser.value?.id === authorId

async function onResolve() {
  await resolveComment(props.vaultId, props.noteId, props.comment.id, !props.comment.resolved)
}

async function onDelete() {
  await removeComment(props.vaultId, props.noteId, props.comment.id)
}

async function onSubmitReply(body: string) {
  submittingReply.value = true
  try {
    await addReply(props.vaultId, props.noteId, props.comment.id, body)
    showReplyComposer.value = false
  } finally {
    submittingReply.value = false
  }
}

async function onDeleteReply(replyId: string) {
  await removeReply(props.vaultId, props.noteId, props.comment.id, replyId)
}
</script>

<template>
  <div class="thread" :class="{ 'thread--resolved': comment.resolved }">
    <!-- Quoted text anchor -->
    <div v-if="comment.quoted_text" class="thread__quote">
      {{ comment.quoted_text }}
    </div>

    <!-- Root comment -->
    <div class="thread__comment">
      <div class="thread__header">
        <span class="thread__author">{{ comment.author_name }}</span>
        <div class="thread__actions">
          <button
            v-if="isNoteAuthor"
            class="thread__action"
            :title="comment.resolved ? 'Unresolve' : 'Resolve'"
            @click="onResolve"
          >
            {{ comment.resolved ? '↩' : '✓' }}
          </button>
          <button
            v-if="isAuthor(comment.author_id) || isNoteAuthor"
            class="thread__action thread__action--danger"
            title="Delete"
            @click="onDelete"
          >
            ✕
          </button>
        </div>
      </div>
      <p class="thread__body">{{ comment.body }}</p>
    </div>

    <!-- Replies -->
    <div v-if="comment.replies.length" class="thread__replies">
      <div
        v-for="reply in comment.replies"
        :key="reply.id"
        class="thread__reply"
      >
        <div class="thread__header">
          <span class="thread__author">{{ reply.author_name }}</span>
          <button
            v-if="isAuthor(reply.author_id) || isNoteAuthor"
            class="thread__action thread__action--danger"
            title="Delete reply"
            @click="onDeleteReply(reply.id)"
          >
            ✕
          </button>
        </div>
        <p class="thread__body">{{ reply.body }}</p>
      </div>
    </div>

    <!-- Reply composer -->
    <div v-if="!comment.resolved" class="thread__reply-area">
      <CommentComposer
        v-if="showReplyComposer"
        placeholder="Reply…"
        :submitting="submittingReply"
        @submit="onSubmitReply"
        @cancel="showReplyComposer = false"
      />
      <button
        v-else
        class="thread__reply-btn"
        @click="showReplyComposer = true"
      >
        Reply
      </button>
    </div>
  </div>
</template>

<style scoped>
.thread {
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.5rem;
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.thread--resolved {
  opacity: 0.5;
}

.thread__quote {
  border-left: 3px solid var(--nyx-color-accent, #6366f1);
  padding: 0.2rem 0.5rem;
  font-size: 0.8rem;
  color: var(--nyx-color-muted, #718096);
  background: var(--nyx-color-surface-raised, #f7fafc);
  border-radius: 0 0.25rem 0.25rem 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.thread__comment,
.thread__reply {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.thread__replies {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding-left: 0.75rem;
  border-left: 2px solid var(--nyx-color-border, #e2e8f0);
}

.thread__header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.thread__author {
  font-weight: 500;
  font-size: 0.8125rem;
  flex: 1;
}

.thread__actions {
  display: flex;
  gap: 0.25rem;
}

.thread__action {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.125rem 0.25rem;
  font-size: 0.75rem;
  border-radius: 0.25rem;
  color: var(--nyx-color-muted, #718096);
  line-height: 1;
}

.thread__action:hover {
  background: var(--nyx-color-surface-raised, #f7fafc);
}

.thread__action--danger:hover {
  color: #e53e3e;
}

.thread__body {
  margin: 0;
  line-height: 1.5;
  white-space: pre-wrap;
  color: inherit;
}

.thread__reply-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.8rem;
  color: var(--nyx-color-accent, #6366f1);
  padding: 0;
  font-family: inherit;
}
</style>

<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/auth/composables'
import { useComments } from '@/comments/composables'
import type { Comment } from '@/shared/types'
import { NyxButton, NyxForm, NyxFormField, NyxInput, NyxModal } from 'nyx-kit/components'
import { NyxTheme, NyxSize, NyxVariant } from 'nyx-kit/types'

const props = defineProps<{
  comment: Comment
  vaultId: string
  noteId: string
  isNoteAuthor: boolean
  active?: boolean
}>()

const emit = defineEmits<{
  focus: []
}>()

const { currentUserId, authMode } = useAuth()
const { resolveComment, removeComment, addReply, removeReply } = useComments()

const replyBody = ref('')
const submittingReply = ref(false)
const confirmDeleteThread = ref(false)
const confirmDeleteReplyId = ref<string | null>(null)

const replyPendingDelete = () => props.comment.replies.find(reply => reply.id === confirmDeleteReplyId.value) ?? null

const isAuthor = (authorId: string) =>
  authMode.value === 'local' || currentUserId.value === authorId

async function onResolve() {
  await resolveComment(props.vaultId, props.noteId, props.comment.id, !props.comment.resolved)
}

async function onDelete() {
  await removeComment(props.vaultId, props.noteId, props.comment.id)
  confirmDeleteThread.value = false
}

async function onSubmitReply(body: string) {
  submittingReply.value = true
  try {
    await addReply(props.vaultId, props.noteId, props.comment.id, body)
    replyBody.value = ''
  } finally {
    submittingReply.value = false
  }
}

async function onSubmitReplyForm() {
  const trimmed = replyBody.value.trim()
  if (!trimmed || submittingReply.value) return
  await onSubmitReply(trimmed)
}

async function onDeleteReply(replyId: string) {
  await removeReply(props.vaultId, props.noteId, props.comment.id, replyId)
  confirmDeleteReplyId.value = null
}
</script>

<template>
  <div
    class="thread"
    :class="{ 'thread--resolved': comment.resolved, 'thread--active': props.active }"
    @click="emit('focus')"
  >

    <!-- Quoted text anchor -->
    <div v-if="comment.anchor.line_preview" class="thread__quote">
      {{ comment.anchor.line_preview }}
    </div>

    <!-- Root comment -->
    <div class="thread__comment">
      <div class="thread__header">
        <span class="thread__author">{{ comment.author_name }}</span>
        <div class="thread__actions" @click.stop>
          <NyxButton
            v-if="isNoteAuthor"
            :size="NyxSize.Small"
            :title="comment.resolved ? 'Unresolve' : 'Resolve'"
            @click="onResolve"
          >
            {{ comment.resolved ? '↩' : '✓' }}
          </NyxButton>
          <NyxButton
            v-if="isAuthor(comment.author_id) || isNoteAuthor"
            :theme="NyxTheme.Danger"
            :size="NyxSize.Small"
            title="Delete"
            @click="confirmDeleteThread = true"
          >
            ✕
          </NyxButton>
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
          <NyxButton
            v-if="isAuthor(reply.author_id) || isNoteAuthor"
            :theme="NyxTheme.Danger"
            :size="NyxSize.Small"
            title="Delete reply"
            @click="confirmDeleteReplyId = reply.id"
          >
            ✕
          </NyxButton>
        </div>
        <p class="thread__body">{{ reply.body }}</p>
      </div>
    </div>

    <!-- Reply area -->
    <div v-if="!comment.resolved" class="thread__reply-area">
      <NyxForm class="thread__reply-form" @submit.prevent="onSubmitReplyForm" @click.stop>
        <NyxFormField class="thread__reply-field">
          <template #default="{ id }">
            <NyxInput
              :id="id"
              v-model="replyBody"
              class="thread__reply-input"
              placeholder="Reply…"
            />
          </template>
        </NyxFormField>
      </NyxForm>
    </div>

  </div>

  <NyxModal v-model="confirmDeleteThread" title="Delete comment thread">
    <p>This comment thread and all of its replies will be permanently deleted.</p>
    <div class="thread__confirm-preview">
      <div v-if="comment.anchor.line_preview" class="thread__quote">
        {{ comment.anchor.line_preview }}
      </div>
      <p class="thread__confirm-body">{{ comment.body }}</p>
    </div>
    <template #footer>
      <NyxButton @click="confirmDeleteThread = false">Cancel</NyxButton>
      <NyxButton :variant="NyxVariant.Soft" :theme="NyxTheme.Danger" @click="onDelete">Delete</NyxButton>
    </template>
  </NyxModal>

  <NyxModal :model-value="confirmDeleteReplyId !== null" title="Delete reply" @update:model-value="(open) => { if (!open) confirmDeleteReplyId = null }">
    <p>This reply will be permanently deleted.</p>
    <div v-if="replyPendingDelete()" class="thread__confirm-preview">
      <p class="thread__confirm-body">{{ replyPendingDelete()?.body }}</p>
    </div>
    <template #footer>
      <NyxButton @click="confirmDeleteReplyId = null">Cancel</NyxButton>
      <NyxButton :variant="NyxVariant.Soft" :theme="NyxTheme.Danger" @click="confirmDeleteReplyId && onDeleteReply(confirmDeleteReplyId)">Delete</NyxButton>
    </template>
  </NyxModal>
</template>

<style scoped>
.thread {
  background: rgba(31, 31, 36, 0.5);
  border-radius: var(--nyx-radius-lg);
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.thread--resolved {
  opacity: 1;
}

.thread--active {
  background: color-mix(in srgb, var(--nyx-c-primary) 8%, rgba(31, 31, 36, 0.5));
  outline: 1px solid color-mix(in srgb, var(--nyx-c-primary) 60%, white 0%);
  outline-offset: -1px;
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--nyx-c-primary) 18%, transparent);
}

.thread__quote {
  border-left: 2px solid var(--nyx-c-primary-dark);
  padding: 0.2rem 0.625rem;
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
  background: rgba(73, 67, 95, 0.2);
  border-radius: 0 var(--nyx-radius-sm) var(--nyx-radius-sm) 0;
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
  border-left: 1px solid var(--nyx-c-divider);
}

.thread__header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.thread__author {
  font-weight: 600;
  font-size: 0.75rem;
  color: var(--nyx-c-text-1);
  flex: 1;
}

.thread__actions {
  display: flex;
  gap: 0.25rem;
}

.thread__body {
  margin: 0;
  line-height: 1.5;
  white-space: pre-wrap;
  color: var(--nyx-c-text-2);
  font-size: 0.8125rem;
}

.thread__reply-area {
  padding-top: 0.25rem;
}

.thread__reply-form,
.thread__reply-field,
.thread__reply-input {
  width: 100%;
}

.thread__confirm-preview {
  margin-top: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.thread__confirm-body {
  margin: 0;
  padding: 0.625rem 0.75rem;
  border-radius: var(--nyx-radius-md);
  background: rgba(31, 31, 36, 0.5);
  color: var(--nyx-c-text-2);
  white-space: pre-wrap;
}

</style>

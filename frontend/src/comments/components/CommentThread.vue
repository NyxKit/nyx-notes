<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/auth/composables'
import { useComments } from '@/comments/composables'
import { CommentComposer } from '@/comments/components'
import type { Comment } from '@/shared/types'
import { NyxButton } from 'nyx-kit/components'
import { NyxVariant, NyxTheme, NyxSize } from 'nyx-kit/types'

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
          <NyxButton
            v-if="isNoteAuthor"
            :variant="NyxVariant.Ghost"
            :size="NyxSize.Small"
            :title="comment.resolved ? 'Unresolve' : 'Resolve'"
            @click="onResolve"
          >
            {{ comment.resolved ? '↩' : '✓' }}
          </NyxButton>
          <NyxButton
            v-if="isAuthor(comment.author_id) || isNoteAuthor"
            :variant="NyxVariant.Ghost"
            :theme="NyxTheme.Danger"
            :size="NyxSize.Small"
            title="Delete"
            @click="onDelete"
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
            :variant="NyxVariant.Ghost"
            :theme="NyxTheme.Danger"
            :size="NyxSize.Small"
            title="Delete reply"
            @click="onDeleteReply(reply.id)"
          >
            ✕
          </NyxButton>
        </div>
        <p class="thread__body">{{ reply.body }}</p>
      </div>
    </div>

    <!-- Reply area -->
    <div v-if="!comment.resolved" class="thread__reply-area">
      <CommentComposer
        v-if="showReplyComposer"
        placeholder="Reply…"
        :submitting="submittingReply"
        @submit="onSubmitReply"
        @cancel="showReplyComposer = false"
      />
      <NyxButton
        v-else
        :variant="NyxVariant.Ghost"
        @click="showReplyComposer = true"
      >
        Reply
      </NyxButton>
    </div>

  </div>
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
  opacity: 0.45;
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

</style>

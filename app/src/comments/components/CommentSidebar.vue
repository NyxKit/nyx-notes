<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { useAuth } from '@/auth/composables'
import { useComments } from '@/comments/composables'
import { CommentThread, CommentComposer } from '@/comments/components'
import type { Note } from '@/shared/types'
import { NyxTabs } from 'nyx-kit/components'

const props = defineProps<{
  note: Note
}>()

const { authMode, currentUserId } = useAuth()
const { comments, loading, draftComment, cancelDraftComment, submitDraftComment, setActiveComment, activeCommentId, activeTab } = useComments()

const showComposer = ref(false)
const submitting = ref(false)
const threadRefs = ref<Record<string, HTMLElement | null>>({})

const isNoteAuthor = computed(() =>
  authMode.value === 'local' || currentUserId.value === props.note.meta.author_id
)

const openComments = computed(() => comments.value.filter(c => !c.resolved))
const resolvedComments = computed(() => comments.value.filter(c => c.resolved))

watch(draftComment, (value) => {
  showComposer.value = value !== null
  if (value) {
    activeTab.value = 'Open'
  }
})

watch(activeCommentId, async (commentId) => {
  if (!commentId) return
  await nextTick()
  threadRefs.value[commentId]?.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
})

async function onSubmitComment(body: string) {
  submitting.value = true
  try {
    await submitDraftComment(props.note.meta.vault_id, props.note.meta.id, body)
    showComposer.value = false
  } finally {
    submitting.value = false
  }
}

function onCancelComment() {
  cancelDraftComment()
  showComposer.value = false
}

function setThreadRef(commentId: string) {
  return (el: Element | { $el?: Element } | null) => {
    const target = el instanceof Element ? el : el?.$el ?? null
    threadRefs.value[commentId] = target as HTMLElement | null
  }
}
</script>

<template>
  <div class="comment-sidebar">
    <NyxTabs v-model="activeTab" :tabs="['Open', 'Resolved']">

      <template #tab-Open>
        <!-- New comment composer -->
        <div v-if="showComposer" class="comment-sidebar__composer">
          <CommentComposer
            :quoted-text="draftComment?.anchor.line_preview"
            :submitting="submitting"
            :autofocus="showComposer"
            @submit="onSubmitComment"
            @cancel="onCancelComment"
          />
        </div>

        <div v-if="loading && !openComments.length && !showComposer" class="comment-sidebar__state">Loading…</div>
        <template v-else>
          <!-- Open threads -->
          <div v-if="openComments.length" class="comment-sidebar__threads">
            <CommentThread
              v-for="comment in openComments"
              :key="comment.id"
              :ref="setThreadRef(comment.id)"
              :comment="comment"
              :vault-id="note.meta.vault_id"
              :note-id="note.meta.id"
              :is-note-author="isNoteAuthor"
              :active="activeCommentId === comment.id"
              @focus="setActiveComment(comment.id)"
            />
          </div>

          <div
            v-if="!loading && !openComments.length && !showComposer"
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
            :ref="setThreadRef(comment.id)"
            :comment="comment"
              :vault-id="note.meta.vault_id"
              :note-id="note.meta.id"
              :is-note-author="isNoteAuthor"
              :active="activeCommentId === comment.id"
              @focus="setActiveComment(comment.id)"
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

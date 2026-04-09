<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { NyxEditor } from 'nyx-kit/components'
import {
  NyxEditorFormat,
  NyxEditorMode,
  NyxEditorToolbar,
  NyxVariant,
  type NyxAnnotation,
  type NyxAnnotationAnchor,
} from 'nyx-kit/types'
import { useAuth } from '@/auth/composables'
import { useNotesStore } from '@/notes/stores'
import { useEditorStore } from '@/notes/stores'
import NoteToolbar from './NoteToolbar.vue'
import { AuthMode, NotePermission } from '@/shared/types'
import type { Note } from '@/shared/types'

const props = defineProps<{
  note: Note
  annotations?: NyxAnnotation[]
}>()

const emit = defineEmits<{
  'comment': [selection: NyxAnnotationAnchor]
  'focus-comment': [commentId: string]
  'blur-comment': [commentId: string]
}>()

const { authMode, currentUser, serverMetadata } = useAuth()
const notesStore = useNotesStore()
const { saving } = storeToRefs(notesStore)
const { save, updatePermission } = notesStore
const editorStore = useEditorStore()

// Editable copies — reset when the note changes
const localTitle = ref(props.note.meta.title)
const localContent = ref(props.note.meta.is_encrypted ? '' : props.note.content)

watch(
  () => props.note.meta.id,
  () => {
    localTitle.value = props.note.meta.title
    localContent.value = props.note.meta.is_encrypted ? '' : props.note.content
    pendingSave = false
    clearTimeout(saveTimer)
  }
)

// Read-only when non-author and vault permission is not 'edit'
const isAuthor = computed(() =>
  authMode.value === AuthMode.Local
  || currentUser.value?.id === props.note.meta.author_id
  || serverMetadata.value?.current_user_id === props.note.meta.author_id
)

const readonly = computed(() =>
    props.note.meta.vault_id !== 'feedback'
    && !isAuthor.value
    && props.note.meta.permission !== NotePermission.Edit
)

// Auto-save with 1.5s debounce
let saveTimer: ReturnType<typeof setTimeout>
let pendingSave = false

function scheduleAutoSave() {
  if (readonly.value) return
  pendingSave = true
  clearTimeout(saveTimer)
  saveTimer = setTimeout(flushSave, 1500)
}

async function flushSave() {
  if (!pendingSave || readonly.value) return
  pendingSave = false
    await save(props.note.meta.vault_id, props.note.meta.id, {
      title: localTitle.value,
      content: localContent.value,
      tags: props.note.meta.tags,
      category: props.note.meta.category ?? undefined,
    })
}

function onTitleChange(value: string) {
  localTitle.value = value
  scheduleAutoSave()
}

function onContentChange(value: string) {
  localContent.value = value
  scheduleAutoSave()
}

async function onPermissionChange(permission: NotePermission) {
  await updatePermission(props.note.meta.vault_id, props.note.meta.id, permission)
}

function onComment(selection: NyxAnnotationAnchor) {
  emit('comment', selection)
}

function onFocusComment(commentId: string) {
  emit('focus-comment', commentId)
}

function onBlurComment(commentId: string) {
  emit('blur-comment', commentId)
}
</script>

<template>
  <div class="note-editor">
    <NoteToolbar
      :title="localTitle"
      :tags="note.meta.tags"
      :permission="note.meta.permission"
      :saving="saving"
      :readonly="readonly"
      @update:title="onTitleChange"
      @update:permission="onPermissionChange"
    />

    <div v-if="note.meta.is_encrypted" class="note-editor__encrypted">
      This note is encrypted and cannot be edited in the browser.
    </div>

    <NyxEditor
      v-else
      class="note-editor__body"
      :model-value="localContent"
      :source="editorStore.isSourceView"
      :variant="NyxVariant.Text"
      :toolbar="NyxEditorToolbar.Full"
      :format="NyxEditorFormat.Markdown"
      :mode="NyxEditorMode.Zen"
      :annotations="props.annotations ?? []"
      :disabled="readonly"
      placeholder="Start writing…"
      @change="onContentChange"
      @annotation:create="onComment"
      @annotation:focus="onFocusComment"
      @annotation:blur="onBlurComment"
    />
  </div>
</template>

<style scoped>
.note-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  padding: 0 1.5rem 2rem;
}

.note-editor__body {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
  max-width: 768px;
  width: 100%;
  margin: 0 auto;
}

.note-editor__encrypted {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--nyx-c-text-3);
  font-size: 0.9rem;
}
</style>

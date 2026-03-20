<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { NyxEditor } from 'nyx-kit/components'
import { NyxEditorFormat, NyxEditorMode, NyxEditorToolbar, NyxVariant, NyxEditorSelection } from 'nyx-kit/types'
import { useAuth } from '@/composables/useAuth'
import { useNotes } from '@/composables/useNotes'
import NoteToolbar from '@/components/NoteToolbar.vue'
import type { Note, NotePermission } from '@/types'

const props = defineProps<{
  note: Note
}>()

const emit = defineEmits<{
  'comment': [selection: NyxEditorSelection]
}>()

const { authMode, currentUser } = useAuth()
const { save, saving, updatePermission } = useNotes()

// Local editable copies — reset when the note changes
const localTitle = ref(props.note.meta.title)
const localContent = ref(props.note.meta.is_encrypted ? '' : props.note.content)
const isSourceView = ref(false)

watch(
  () => props.note.meta.id,
  () => {
    localTitle.value = props.note.meta.title
    localContent.value = props.note.meta.is_encrypted ? '' : props.note.content
    isSourceView.value = false
    pendingSave = false
    clearTimeout(saveTimer)
  }
)

// Read-only when non-author and vault permission is not 'edit'
const isAuthor = computed(() =>
  authMode.value === 'local' || currentUser.value?.id === props.note.meta.author_id
)

const readonly = computed(() =>
  !isAuthor.value && props.note.meta.permission !== 'edit'
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

function onComment(selection: NyxEditorSelection) {
  console.log('onComment', selection)
  emit('comment', selection)
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
      :is-author="isAuthor"
      :is-source-view="isSourceView"
      @update:title="onTitleChange"
      @update:permission="onPermissionChange"
      @toggle:source="isSourceView = !isSourceView"
    />

    <div v-if="note.meta.is_encrypted" class="note-editor__encrypted">
      This note is encrypted and cannot be edited in the browser.
    </div>

    <NyxEditor
      v-else
      class="note-editor__body"
      :model-value="localContent"
      :source="isSourceView"
      :variant="NyxVariant.Text"
      :toolbar="NyxEditorToolbar.Full"
      :format="NyxEditorFormat.Markdown"
      :mode="NyxEditorMode.Zen"
      :disabled="readonly"
      placeholder="Start writing…"
      @change="onContentChange"
      @comment="onComment"
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

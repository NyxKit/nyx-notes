<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter, onBeforeRouteLeave } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { useEditorStore } from '@/notes/stores'
import { useComments, toCommentAnchor } from '@/comments/composables'
import { NyxModal, NyxButton, NyxIcon } from 'nyx-kit/components'
import { NyxTheme, NyxShape, NyxVariant } from 'nyx-kit/types'
import { NoteEditor } from '@/notes/components'
import { CommentSidebar } from '@/comments/components'

const route = useRoute()
const router = useRouter()
const isCommentsOpen = ref(false)
const showDeleteConfirm = ref(false)
const editorStore = useEditorStore()
const commentsStore = useComments()
const { annotations, setActiveComment, beginComment, load: loadComments, clearLoadedComments } = commentsStore

const FAVORITES_KEY = 'nyx_favorites'

function toggleFavorite() {
  const note = activeNote.value
  if (!note) return
  try {
    const ids: string[] = JSON.parse(localStorage.getItem(FAVORITES_KEY) ?? '[]')
    const idx = ids.indexOf(note.meta.id)
    if (idx === -1) ids.push(note.meta.id)
    else ids.splice(idx, 1)
    localStorage.setItem(FAVORITES_KEY, JSON.stringify(ids))
  } catch { /* ignore */ }
}

async function onCreateComment(selection: { text: string; context: { prefix: string; suffix: string }; range: { from: number; to: number } }) {
  if (!activeNote.value) return
  isCommentsOpen.value = true
  beginComment({
    body: '',
    anchor: toCommentAnchor(selection),
  })
}

function onFocusComment(commentId: string) {
  isCommentsOpen.value = true
  setActiveComment(commentId)
}

async function confirmDelete() {
  const note = activeNote.value
  if (!note) return
  await remove(note.meta.vault_id, note.meta.id)
  showDeleteConfirm.value = false
  router.replace(`/vaults/${note.meta.vault_id}/notes`)
}

const vaultStore = useVaultStore()
const { vaults } = storeToRefs(vaultStore)
const { load: loadVaults, setActive } = vaultStore
const notesStore = useNotesStore()
const { activeNote } = storeToRefs(notesStore)
const { loadNote, remove } = notesStore

const LAST_NOTE_KEY = 'nyx_last_note'

const section = computed(() => {
  if (route.path.includes('/favorites')) return 'favorites'
  if (route.path.includes('/drafts')) return 'drafts'
  return 'notes'
})

const noteTitle = computed(() => {
  if (section.value === 'favorites') return 'Favorites'
  if (section.value === 'drafts') return 'Drafts'
  return activeNote.value?.meta.title || 'Untitled Note'
})

async function pruneIfEmpty() {
  const note = activeNote.value
  if (!note) return
  if (!note.meta.title.trim() && !note.content.trim()) {
    await remove(note.meta.vault_id, note.meta.id)
  }
}

// When leaving NoteView entirely (e.g. to settings or home)
onBeforeRouteLeave(pruneIfEmpty)

watch(
  () => [route.params.vault_id, route.params.id] as [string, string],
  async ([vaultId, noteId], prev) => {
    // When switching notes, prune the previous one if it was empty
    if (prev?.[1]) await pruneIfEmpty()

    if (!vaultId || !noteId) return

    if (!vaults.value.length) await loadVaults()

    if (!vaults.value.length) {
      localStorage.removeItem(LAST_NOTE_KEY)
      router.replace('/')
      return
    }

    const vault = vaults.value.find(v => v.id === vaultId)
    if (vault) setActive(vault)

    await loadNote(vaultId, noteId)
    clearLoadedComments()
    await loadComments(vaultId, noteId)
    editorStore.reset()

    localStorage.setItem(LAST_NOTE_KEY, JSON.stringify({ vaultId, noteId }))
  },
  { immediate: true }
)

// Ensure vault is set when navigating to section pages without a note id
watch(
  () => route.params.vault_id as string,
  async (vaultId) => {
    if (!vaultId || route.params.id) return
    if (!vaults.value.length) await loadVaults()
    const vault = vaults.value.find(v => v.id === vaultId)
    if (vault) setActive(vault)
  },
  { immediate: true }
)
</script>

<template>
  <div class="app-shell__main">

    <!-- Top header bar -->
    <header class="app-shell__header">
      <div class="app-shell__header-left">
        <span class="app-shell__note-title">{{ noteTitle }}</span>
      </div>
      <div v-if="section === 'notes' && activeNote" class="app-shell__header-right">
        <!-- Source view -->
        <NyxButton
          :shape="NyxShape.Square"
          title="Toggle source view"
          @click="editorStore.toggleSourceView()"
        >
          <NyxIcon name="code" :size="18" />
        </NyxButton>
        <!-- Favorite -->
        <NyxButton
          :shape="NyxShape.Square"
          title="Toggle favorite"
          @click="toggleFavorite()"
        >
          <NyxIcon name="star" :size="18" />
        </NyxButton>
        <!-- Delete -->
        <NyxButton
          :shape="NyxShape.Square"
          :theme="NyxTheme.Danger"
          title="Delete note"
          @click="showDeleteConfirm = true"
        >
          <NyxIcon name="trash-2" :size="18" />
        </NyxButton>
        <!-- Comments -->
        <NyxButton
          :shape="NyxShape.Square"
          title="Toggle comments"
          @click="isCommentsOpen = !isCommentsOpen"
        >
          <NyxIcon name="message-circle" :size="18" />
        </NyxButton>
      </div>
    </header>

    <!-- Body row: canvas + right comments panel -->
    <div class="app-shell__body">

      <!-- Main writing canvas -->
      <main class="app-shell__canvas">
        <template v-if="section === 'notes'">
          <NoteEditor
            v-if="activeNote"
            :note="activeNote"
            :annotations="annotations"
            @comment="onCreateComment"
            @focus-comment="onFocusComment"
            @blur-comment="setActiveComment(null)"
          />
          <div v-else class="app-shell__placeholder">Select a note</div>
        </template>
        <div v-else class="app-shell__wip">
          <div class="app-shell__wip-icon" aria-hidden="true">
            <NyxIcon v-if="section === 'favorites'" name="star" :size="32" />
            <NyxIcon v-else name="file-edit" :size="32" />
          </div>
          <span class="app-shell__wip-label">{{ section === 'favorites' ? 'Favorites' : 'Drafts' }}</span>
          <span class="app-shell__wip-sub">Coming soon</span>
        </div>
      </main>

      <!-- Right sidebar (comments) -->
      <aside class="app-shell__comments" :class="{ 'app-shell__comments--open': isCommentsOpen }">
        <div class="app-shell__comments-inner">
          <CommentSidebar v-if="activeNote && isCommentsOpen" :note="activeNote" />
        </div>
      </aside>

    </div>

    <!-- Delete confirmation modal -->
    <NyxModal v-model="showDeleteConfirm" title="Delete note">
      <p>This note will be permanently deleted. This cannot be undone.</p>
      <template #footer>
        <NyxButton :variant="NyxVariant.Subtle" @click="showDeleteConfirm = false">Cancel</NyxButton>
        <NyxButton :theme="NyxTheme.Danger" @click="confirmDelete">Delete</NyxButton>
      </template>
    </NyxModal>

  </div>
</template>

<style scoped>
.app-shell__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

/* ── Header ─────────────────────────────────────────────────── */
.app-shell__header {
  height: 64px;
  flex-shrink: 0;
  background: var(--nyx-c-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1rem;
  box-shadow: 0 1px 0 0 var(--nyx-c-divider);
}

.app-shell__header-left,
.app-shell__header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.app-shell__note-title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 320px;
}

/* ── Body row ────────────────────────────────────────────────── */
.app-shell__body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* ── Main canvas ────────────────────────────────────────────── */
.app-shell__canvas {
  flex: 1;
  background: var(--nyx-c-bg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.app-shell__placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--nyx-c-text-3);
  font-size: 0.875rem;
}

.app-shell__wip {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.app-shell__wip-icon {
  color: var(--nyx-c-primary);
  opacity: 0.4;
  line-height: 0;
  margin-bottom: 0.5rem;
}

.app-shell__wip-label {
  font-family: 'Manrope', sans-serif;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--nyx-c-text-2);
}

.app-shell__wip-sub {
  font-size: 0.8125rem;
  color: var(--nyx-c-text-3);
}

/* ── Right sidebar (comments) ───────────────────────────────── */
.app-shell__comments {
  width: 0;
  overflow: hidden;
  flex-shrink: 0;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.app-shell__comments--open {
  width: 320px;
}

.app-shell__comments-inner {
  width: 320px;
  height: 100%;
  background: var(--nyx-c-bg-soft);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ── Footer ─────────────────────────────────────────────────── */
.app-shell__footer {
  height: 40px;
  flex-shrink: 0;
  background: var(--nyx-c-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.app-shell__footer-left,
.app-shell__footer-right {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.app-shell__stat {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}


</style>

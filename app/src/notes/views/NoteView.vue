<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter, onBeforeRouteLeave } from 'vue-router'
import { storeToRefs } from 'pinia'
import { vaultRoute } from '@/shared/utils'
import { useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore, useNoteBrowsingStore } from '@/notes/stores'
import { useEditorStore } from '@/notes/stores'
import { useComments, toCommentAnchor } from '@/comments/composables'
import { NyxModal, NyxButton, NyxIcon } from 'nyx-kit/components'
import { NyxTheme, NyxShape, NyxVariant } from 'nyx-kit/types'
import { NoteEditor } from '@/notes/components'
import { CommentSidebar } from '@/comments/components'
import { ImageShelf } from '@/shared/components'
import { RoutePath } from '@/shared/types'

const route = useRoute()
const router = useRouter()
const isCommentsOpen = ref(false)
const showDeleteConfirm = ref(false)
const editorStore = useEditorStore()
const commentsStore = useComments()
const { annotations, setActiveComment, beginComment, clearLoadedComments } = commentsStore

const { activeProfile } = useWorkspaceProfiles()
const noteBrowsingStore = useNoteBrowsingStore()

function toggleFavorite() {
  const note = activeNote.value
  const profileId = activeProfile.value?.id
  if (!note || !profileId) return
  noteBrowsingStore.toggleFavoriteForNote(profileId, note.meta)
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
  const currentVault = vaultStore.activeVault ?? vaults.value.find(v => v.slug === note.meta.vault_id)
  if (currentVault) router.replace(vaultRoute(currentVault))
}

const vaultStore = useVaultStore()
const { vaults } = storeToRefs(vaultStore)
const { setActive } = vaultStore
const notesStore = useNotesStore()
const { activeNote } = storeToRefs(notesStore)
const { remove, clearActive } = notesStore

const LAST_NOTE_KEY = 'nyx_last_note'

const section = computed(() => {
  if (route.path.includes(RoutePath.Favorites)) return 'favorites'
  if (route.path.includes('/drafts')) return 'drafts'
  return 'notes'
})

const favoriteActive = computed(() => {
  const profileId = activeProfile.value?.id
  const note = activeNote.value
  if (!profileId || !note) return false
  return noteBrowsingStore.isFavorite(profileId, note.meta.vault_id, note.meta.id)
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
  () => [route.params.vault_id, route.params.note_id] as [string, string],
  async ([vaultId, noteId], prev) => {
    // When switching notes, prune the previous one if it was empty
    if (prev?.[1]) await pruneIfEmpty()

    if (!vaultId || !noteId) {
      clearActive()
      clearLoadedComments()
      editorStore.reset()
      return
    }

    if (!vaults.value.length) {
      localStorage.removeItem(LAST_NOTE_KEY)
      router.replace({ name: 'home' })
      return
    }

    clearLoadedComments()
    editorStore.reset()
    const currentVault = vaults.value.find(v => v.slug === vaultId) ?? null
    if (currentVault) setActive(currentVault)

    localStorage.setItem(LAST_NOTE_KEY, JSON.stringify({ vaultId, noteId }))
  },
  { immediate: true }
)

// Ensure vault is set when navigating to section pages without a note id
watch(
  () => route.params.vault_id as string,
  async (vaultId) => {
    if (!vaultId || route.params.note_id) return
    const vault = vaults.value.find(v => v.slug === vaultId)
    if (vault) setActive(vault)
  },
  { immediate: true }
)
</script>

<template>
  <div class="note-view">
    <Teleport to="#layout-header-actions" defer>
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
        <NyxIcon name="star" :size="18" :style="favoriteActive ? 'color: var(--nyx-c-primary); fill: currentColor;' : undefined" />
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
    </Teleport>

    <!-- Body row: canvas + right comments panel -->
    <div class="note-view__body">

      <!-- Main writing canvas -->
      <main class="note-view__canvas">
        <template v-if="section === 'notes'">
          <NoteEditor
            v-if="activeNote"
            :key="activeNote.meta.id"
            :note="activeNote"
            :annotations="annotations"
            @comment="onCreateComment"
            @focus-comment="onFocusComment"
            @blur-comment="setActiveComment(null)"
          />
          <ImageShelf v-if="activeNote" :images="activeNote.meta.images ?? []" title="Images" />
          <div v-else class="note-view__placeholder">Select a note</div>
        </template>
        <div v-else class="note-view__wip">
          <div class="note-view__wip-icon" aria-hidden="true">
            <NyxIcon v-if="section === 'favorites'" name="star" :size="32" />
            <NyxIcon v-else name="file-edit" :size="32" />
          </div>
          <span class="note-view__wip-label">{{ section === 'favorites' ? 'Favorites' : 'Drafts' }}</span>
          <span class="note-view__wip-sub">Coming soon</span>
        </div>
      </main>

      <!-- Right sidebar (comments) -->
      <aside class="note-view__comments" :class="{ 'note-view__comments--open': isCommentsOpen }">
        <div class="note-view__comments-inner">
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
.note-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

/* ── Body row ────────────────────────────────────────────────── */
.note-view__body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* ── Main canvas ────────────────────────────────────────────── */
.note-view__canvas {
  flex: 1;
  background: var(--nyx-c-bg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.note-view__placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--nyx-c-text-3);
  font-size: 0.875rem;
}

.note-view__wip {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.note-view__wip-icon {
  color: var(--nyx-c-primary);
  opacity: 0.4;
  line-height: 0;
  margin-bottom: 0.5rem;
}

.note-view__wip-label {
  font-family: 'Manrope', sans-serif;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--nyx-c-text-2);
}

.note-view__wip-sub {
  font-size: 0.8125rem;
  color: var(--nyx-c-text-3);
}

/* ── Right sidebar (comments) ───────────────────────────────── */
.note-view__comments {
  width: 0;
  overflow: hidden;
  flex-shrink: 0;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.note-view__comments--open {
  width: 320px;
}

.note-view__comments-inner {
  width: 320px;
  height: 100%;
  background: var(--nyx-c-bg-soft);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>

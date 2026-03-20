<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter, onBeforeRouteLeave } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'
import { useEditorStore } from '@/stores/editor'
import VaultSwitcher from '@/components/VaultSwitcher.vue'
import SidebarNav from '@/components/SidebarNav.vue'
import NoteList from '@/components/NoteList.vue'
import NoteEditor from '@/components/NoteEditor.vue'
import CommentSidebar from '@/components/CommentSidebar.vue'

const route = useRoute()
const router = useRouter()
const isSidebarOpen = ref(false)
const isCommentsOpen = ref(false)
const showDeleteConfirm = ref(false)
const editorStore = useEditorStore()

const FAVORITES_KEY = 'nyx_favorites'

function isFavorited(noteId: string): boolean {
  try {
    const ids: string[] = JSON.parse(localStorage.getItem(FAVORITES_KEY) ?? '[]')
    return ids.includes(noteId)
  } catch { return false }
}

const favorited = computed(() => activeNote.value ? isFavorited(activeNote.value.meta.id) : false)

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

async function confirmDelete() {
  const note = activeNote.value
  if (!note) return
  showDeleteConfirm.value = false
  await remove(note.meta.vault_id, note.meta.id)
  router.replace(`/vaults/${note.meta.vault_id}/notes`)
}

const { vaults, load: loadVaults, setActive, activeVault } = useVaults()
const { loadNote, activeNote, saving, remove } = useNotes()

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

const wordCount = computed(() => {
  const text = activeNote.value?.content ?? ''
  return text.trim() ? text.trim().split(/\s+/).length : 0
})

// Sidebar is always visible on non-notes sections; toggleable on the editor
const sidebarVisible = computed(() => section.value !== 'notes' || isSidebarOpen.value)

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
  <div class="app-shell">

    <!-- Left sidebar — full height, outside main column -->
    <aside class="app-shell__sidebar" :class="{ 'app-shell__sidebar--open': sidebarVisible }">
      <div class="app-shell__sidebar-inner">
        <VaultSwitcher />
        <SidebarNav />
        <NoteList />
        <div class="app-shell__sidebar-footer">
          <RouterLink
            v-if="activeVault"
            :to="`/vaults/${activeVault.id}/settings`"
            class="app-shell__footer-nav-item"
          >
            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" aria-hidden="true">
              <circle cx="7.5" cy="7.5" r="2" stroke="currentColor" stroke-width="1.25"/>
              <path d="M7.5 1v1.5M7.5 12.5V14M1 7.5h1.5M12.5 7.5H14M2.75 2.75l1.06 1.06M11.19 11.19l1.06 1.06M2.75 12.25l1.06-1.06M11.19 3.81l1.06-1.06" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
            </svg>
            Settings
          </RouterLink>
          <a href="#" class="app-shell__footer-nav-item">
            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" aria-hidden="true">
              <circle cx="7.5" cy="7.5" r="6" stroke="currentColor" stroke-width="1.25"/>
              <path d="M7.5 10.5v-1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <path d="M7.5 8.5c0-1 .75-1.5 1.25-2A2.25 2.25 0 105.25 5" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
            </svg>
            Help
          </a>
        </div>
      </div>
    </aside>

    <!-- Main column: header + body + footer -->
    <div class="app-shell__main">

      <!-- Top header bar -->
      <header class="app-shell__header">
        <div class="app-shell__header-left">
          <button
            v-if="section === 'notes'"
            class="app-shell__icon-btn"
            :class="{ 'app-shell__icon-btn--active': isSidebarOpen }"
            title="Toggle sidebar"
            @click="isSidebarOpen = !isSidebarOpen"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
              <path d="M2 4.5h14M2 9h14M2 13.5h14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </button>
          <span class="app-shell__note-title">{{ noteTitle }}</span>
        </div>
        <div v-if="section === 'notes' && activeNote" class="app-shell__header-right">
          <!-- Source view -->
          <button
            class="app-shell__icon-btn"
            :class="{ 'app-shell__icon-btn--active': editorStore.isSourceView }"
            title="Toggle source view"
            @click="editorStore.toggleSourceView()"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
              <path d="M5 4L1 9l4 5M13 4l4 5-4 5M10 2l-2 14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          <!-- Favorite -->
          <button
            class="app-shell__icon-btn"
            :class="{ 'app-shell__icon-btn--active': favorited }"
            title="Toggle favorite"
            @click="toggleFavorite()"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
              <path d="M9 2l1.5 4.5H15l-3.5 2.75 1.5 4.75L9 11.25 5 14l1.5-4.75L3 6.5h4.5L9 2z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" :fill="favorited ? 'currentColor' : 'none'"/>
            </svg>
          </button>
          <!-- Delete -->
          <button
            class="app-shell__icon-btn app-shell__icon-btn--danger"
            title="Delete note"
            @click="showDeleteConfirm = true"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
              <path d="M3 5h12M7 5V3h4v2M6 5l.75 10h4.5L12 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          <!-- Comments -->
          <button
            class="app-shell__icon-btn"
            :class="{ 'app-shell__icon-btn--active': isCommentsOpen }"
            title="Toggle comments"
            @click="isCommentsOpen = !isCommentsOpen"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
              <path d="M15 9C15 12.31 12.31 15 9 15c-.72 0-1.41-.12-2.05-.33L4 15.5l.61-2.84C4.22 11.79 3 10.52 3 9c0-3.31 2.69-6 6-6s6 2.69 6 6z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </header>

      <!-- Body row: canvas + right comments panel -->
      <div class="app-shell__body">

        <!-- Main writing canvas -->
        <main class="app-shell__canvas">
          <template v-if="section === 'notes'">
            <NoteEditor v-if="activeNote" :note="activeNote" />
            <div v-else class="app-shell__placeholder">Select a note</div>
          </template>
          <div v-else class="app-shell__wip">
            <div class="app-shell__wip-icon" aria-hidden="true">
              <svg v-if="section === 'favorites'" width="32" height="32" viewBox="0 0 32 32" fill="none">
                <path d="M16 4l3 9h9.5L21 18.5 24 28l-8-5.5L8 28l3-9.5L4 13h9.5L16 4z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
              </svg>
              <svg v-else width="32" height="32" viewBox="0 0 32 32" fill="none">
                <path d="M20 6l6 6-14 14H6v-6L20 6z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
                <path d="M17 9l6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              </svg>
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

      <!-- Footer status bar -->
      <footer class="app-shell__footer">
        <div class="app-shell__footer-left">
          <span v-if="section === 'notes'" class="app-shell__stat">{{ wordCount }} words</span>
        </div>
        <div class="app-shell__footer-right">
          <span v-if="section === 'notes' && saving" class="app-shell__stat">Saving…</span>
          <span v-else-if="section === 'notes'" class="app-shell__stat">Saved</span>
        </div>
      </footer>

    </div>

    <!-- Delete confirmation modal -->
    <Teleport to="body">
      <div v-if="showDeleteConfirm" class="app-modal-backdrop" @click.self="showDeleteConfirm = false">
        <div class="app-modal" role="dialog" aria-modal="true">
          <p class="app-modal__message">Delete this note? This cannot be undone.</p>
          <div class="app-modal__actions">
            <button class="app-modal__btn app-modal__btn--cancel" @click="showDeleteConfirm = false">Cancel</button>
            <button class="app-modal__btn app-modal__btn--confirm" @click="confirmDelete">Delete</button>
          </div>
        </div>
      </div>
    </Teleport>

  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: row;
  height: 100vh;
  overflow: hidden;
  background: var(--nyx-c-bg);
  color: var(--nyx-c-text-1);
}

/* ── Main column (header + body + footer) ────────────────────── */
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

.app-shell__icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--nyx-c-text-2);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border-radius: var(--nyx-radius-md);
  transition: background 0.2s, color 0.2s;
  line-height: 0;
}

.app-shell__icon-btn:hover {
  background: #2b2c32;
  color: var(--nyx-c-text-1);
}

.app-shell__icon-btn--active {
  color: var(--nyx-c-primary);
}

/* ── Body row ────────────────────────────────────────────────── */
.app-shell__body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* ── Left sidebar ───────────────────────────────────────────── */
.app-shell__sidebar {
  width: 0;
  overflow: hidden;
  flex-shrink: 0;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.app-shell__sidebar--open {
  width: 288px;
}

.app-shell__sidebar-inner {
  width: 288px;
  height: 100%;
  background: var(--nyx-c-bg-soft);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Sidebar footer: Settings + Help as nav items */
.app-shell__sidebar-footer {
  margin-top: auto;
  padding: 0.5rem 0.75rem;
  flex-shrink: 0;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.app-shell__footer-nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  border-radius: var(--nyx-radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
  text-decoration: none;
  transition: background 0.2s, color 0.2s;
}

.app-shell__footer-nav-item:hover {
  background: #25252b;
  color: var(--nyx-c-text-1);
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

.app-shell__icon-btn--danger:hover {
  color: #ec7c8a;
  background: rgba(236, 124, 138, 0.08);
}

/* ── Delete confirm modal ────────────────────────────────────── */
.app-modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.app-modal {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-lg);
  padding: 1.5rem;
  width: 320px;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.app-modal__message {
  margin: 0;
  font-size: 0.875rem;
  color: var(--nyx-c-text-1);
  line-height: 1.5;
}

.app-modal__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

.app-modal__btn {
  border: none;
  cursor: pointer;
  padding: 0.5rem 1rem;
  border-radius: var(--nyx-radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  font-family: inherit;
  transition: background 0.2s, color 0.2s;
}

.app-modal__btn--cancel {
  background: var(--nyx-c-bg-mute);
  color: var(--nyx-c-text-2);
}

.app-modal__btn--cancel:hover {
  color: var(--nyx-c-text-1);
}

.app-modal__btn--confirm {
  background: rgba(236, 124, 138, 0.15);
  color: #ec7c8a;
}

.app-modal__btn--confirm:hover {
  background: rgba(236, 124, 138, 0.25);
}
</style>

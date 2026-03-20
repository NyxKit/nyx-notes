<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'
import VaultSwitcher from '@/components/VaultSwitcher.vue'
import NoteList from '@/components/NoteList.vue'
import NoteEditor from '@/components/NoteEditor.vue'
import CommentSidebar from '@/components/CommentSidebar.vue'

const route = useRoute()
const router = useRouter()
const isCommentsOpen = ref(false)

const { vaults, load: loadVaults, setActive } = useVaults()
const { loadNote, activeNote } = useNotes()

const LAST_NOTE_KEY = 'nyx_last_note'

watch(
  () => [route.params.vault_id, route.params.id] as [string, string],
  async ([vaultId, noteId]) => {
    if (!vaultId || !noteId) return

    // Ensure vaults are loaded before trying to find the active vault
    if (!vaults.value.length) await loadVaults()

    // No vaults at all — stale URL, go back to home
    if (!vaults.value.length) {
      localStorage.removeItem(LAST_NOTE_KEY)
      router.replace('/')
      return
    }

    // Sync active vault
    const vault = vaults.value.find(v => v.id === vaultId)
    if (vault) setActive(vault)

    // Load the note
    await loadNote(vaultId, noteId)

    // Persist last visited location
    localStorage.setItem(LAST_NOTE_KEY, JSON.stringify({ vaultId, noteId }))
  },
  { immediate: true }
)
</script>

<template>
  <div class="note-view">
    <aside class="note-view__sidebar">
      <VaultSwitcher />
      <NoteList />
    </aside>

    <main class="note-view__editor">
      <NoteEditor v-if="activeNote" :note="activeNote" :is-comments-open="isCommentsOpen" @toggle:comments="isCommentsOpen = !isCommentsOpen" />
      <div v-else class="note-view__placeholder note-view__placeholder--empty">
        Select a note
      </div>
    </main>

    <aside v-if="isCommentsOpen" class="note-view__comments">
      <CommentSidebar v-if="activeNote" :note="activeNote" />
    </aside>
  </div>
</template>

<style scoped>
.note-view {
  display: grid;
  grid-template-columns: 260px 1fr;
  height: 100vh;
  overflow: hidden;
}

.note-view:has(.note-view__comments) {
  grid-template-columns: 260px 1fr 280px;
}

.note-view__sidebar {
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--nyx-color-border, #e2e8f0);
  overflow: hidden;
}

.note-view__editor {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.note-view__comments {
  border-left: 1px solid var(--nyx-color-border, #e2e8f0);
}

.note-view__placeholder--empty {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--nyx-color-muted, #718096);
}

</style>

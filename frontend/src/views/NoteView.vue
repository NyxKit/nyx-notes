<script setup lang="ts">
import { watch } from 'vue'
import { useRoute } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'
import VaultSwitcher from '@/components/VaultSwitcher.vue'
import NoteList from '@/components/NoteList.vue'

const route = useRoute()
const { vaults, load: loadVaults, setActive } = useVaults()
const { loadNote, activeNote } = useNotes()

const LAST_NOTE_KEY = 'nyx_last_note'

// Load vaults once if not yet loaded
if (!vaults.value.length) loadVaults()

watch(
  () => [route.params.vault_id, route.params.id] as [string, string],
  async ([vaultId, noteId]) => {
    if (!vaultId || !noteId) return

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
      <!-- NoteEditor + NoteToolbar — Layer 6 -->
      <div v-if="activeNote" class="note-view__placeholder">
        <h2>{{ activeNote.meta.title || 'Untitled' }}</h2>
        <pre class="note-view__content">{{ activeNote.content }}</pre>
      </div>
      <div v-else class="note-view__placeholder note-view__placeholder--empty">
        Select a note
      </div>
    </main>

    <aside class="note-view__comments">
      <!-- CommentSidebar — Layer 7 -->
    </aside>
  </div>
</template>

<style scoped>
.note-view {
  display: grid;
  grid-template-columns: 260px 1fr 280px;
  height: 100vh;
  overflow: hidden;
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

.note-view__placeholder {
  padding: 2rem;
  flex: 1;
  overflow-y: auto;
}

.note-view__placeholder--empty {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--nyx-color-muted, #718096);
}

.note-view__content {
  white-space: pre-wrap;
  font-family: inherit;
  line-height: 1.7;
}
</style>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NyxButton } from 'nyx-kit/components'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'

const router = useRouter()
const { vaults, load: loadVaults, setActive, create: createVault } = useVaults()
const { loadList, notes, create: createNote } = useNotes()

const LAST_NOTE_KEY = 'nyx_last_note'

onMounted(async () => {
  // Load vaults first — needed for both the restore path and the fallback scan
  await loadVaults()

  // Try restoring the last visited note, but only if the vault still exists
  const last = localStorage.getItem(LAST_NOTE_KEY)
  if (last) {
    try {
      const { vaultId, noteId } = JSON.parse(last)
      if (vaults.value.some(v => v.id === vaultId)) {
        router.replace(`/vaults/${vaultId}/notes/${noteId}`)
        return
      }
    } catch { /* malformed entry */ }
    localStorage.removeItem(LAST_NOTE_KEY)
  }

  // Otherwise find the most recently updated note across all vaults
  for (const vault of vaults.value) {
    await loadList(vault.id)
    if (notes.value.length > 0) {
      setActive(vault)
      const latest = notes.value[0] // already sorted by updated_at desc
      router.replace(`/vaults/${vault.id}/notes/${latest.id}`)
      return
    }
  }
  // No notes found — stay on home, show empty state
})

async function createFirst() {
  await loadVaults()

  let vault = vaults.value.find(v => v.slug === 'home') ?? vaults.value[0]
  if (!vault) {
    vault = await createVault({ slug: 'home', name: 'Home' })
  }

  setActive(vault)
  const meta = await createNote(vault.id, { title: 'My first note', content: '' })
  router.push(`/vaults/${vault.id}/notes/${meta.id}`)
}
</script>

<template>
  <div class="home">
    <div class="home__empty">
      <h2>No notes yet</h2>
      <p>Create your first note to get started.</p>
      <NyxButton @click="createFirst">Create note</NyxButton>
    </div>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

.home__empty {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}
</style>

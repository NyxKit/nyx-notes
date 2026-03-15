<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NyxButton } from 'nyx-kit/components'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'

const router = useRouter()
const { vaults, load: loadVaults, setActive } = useVaults()
const { loadList, notes, create } = useNotes()

const LAST_NOTE_KEY = 'nyx_last_note'

onMounted(async () => {
  // Try restoring the last visited note
  const last = localStorage.getItem(LAST_NOTE_KEY)
  if (last) {
    const { vaultId, noteId } = JSON.parse(last)
    router.replace(`/vaults/${vaultId}/notes/${noteId}`)
    return
  }

  // Otherwise find the most recently updated note across all vaults
  await loadVaults()
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
  if (!vaults.value.length) return

  const vault = vaults.value.find(v => v.slug === 'home') ?? vaults.value[0]
  setActive(vault)
  const meta = await create(vault.id, { title: 'My first note', content: '' })
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

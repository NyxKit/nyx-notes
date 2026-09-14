import { computed } from 'vue'
import { useNotesStore } from '@/notes/stores'

export function useNotes(
  vaultId: () => string | undefined,
  serverSlug: () => string | undefined,
) {
  const store = useNotesStore()
  void serverSlug

  return {
    notes: computed(() => {
      const currentVaultId = vaultId()
      return currentVaultId ? store.notesFor(currentVaultId) : []
    }),
    loading: computed(() => false),
    error: computed(() => store.error),
  }
}

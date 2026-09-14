import { storeToRefs } from 'pinia'
import { useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'

export function useVaults() {
  const store = useVaultStore()
  const refs = storeToRefs(store)
  useWorkspaceProfiles()

  return {
    ...refs,
    ...store,
  }
}

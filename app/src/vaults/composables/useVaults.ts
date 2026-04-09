import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useSubscription, useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'

export function useVaults() {
  const store = useVaultStore()
  const refs = storeToRefs(store)
  const { activeProfile } = useWorkspaceProfiles()

  const serverSlug = computed(() => activeProfile.value?.id || 'main-server')
  const userContext = computed(() => activeProfile.value?.id)

  useSubscription(serverSlug, value => store.subscribeList(value, userContext.value))

  return {
    ...refs,
    ...store,
  }
}

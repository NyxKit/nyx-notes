import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { getApiRequestEpoch, NyxBase } from '@/shared/api'
import { fetchVaults, createVault, createServerVault, deleteServerVault, deleteVault, updateVault } from '@/vaults/api'
import type { Vault, CreateVaultRequest, UpdateVaultRequest } from '@/shared/types'

export const useVaultStore = defineStore('vaults', () => {
  const vaults = ref<Vault[]>([])
  const activeVault = ref<Vault | null>(null)
  const vaultSubscriptionKey = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    const requestEpoch = getApiRequestEpoch()
    const hasCachedVaults = vaults.value.length > 0
    loading.value = !hasCachedVaults
    error.value = null
    try {
      const nextVaults = await fetchVaults()
      if (requestEpoch !== getApiRequestEpoch()) return
      vaults.value = nextVaults
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  function subscribeList(serverSlug: string, userContext?: string) {
    const handle = NyxBase.subscribe<Vault[]>(
      NyxBase.createVaultListPersonalQuery(serverSlug, userContext),
      snapshot => {
        vaults.value = snapshot
      },
    )
    vaultSubscriptionKey.value = handle.key
    return handle
  }

  async function create(body: CreateVaultRequest) {
    const vault = await createVault(body)
    vaults.value.push(vault)
    return vault
  }

  async function remove(vaultId: string) {
    await deleteVault(vaultId)
    vaults.value = vaults.value.filter(v => v.slug !== vaultId)
    if (activeVault.value?.slug === vaultId) activeVault.value = null
  }

  function setActive(vault: Vault | null) {
    activeVault.value = vault
  }

  async function update(vaultId: string, body: UpdateVaultRequest) {
    const updated = await updateVault(vaultId, body)
    const idx = vaults.value.findIndex(v => v.slug === vaultId)
    if (idx !== -1) vaults.value[idx] = updated
    if (activeVault.value?.slug === vaultId) activeVault.value = updated
    return updated
  }

  async function addServerVault(body: CreateVaultRequest) {
    const vault = await createServerVault(body)
    vaults.value.push(vault)
    return vault
  }

  async function removeServerVault(vaultId: string) {
    await deleteServerVault(vaultId)
    vaults.value = vaults.value.filter(v => v.slug !== vaultId)
    if (activeVault.value?.slug === vaultId) activeVault.value = null
  }

  function $reset() {
    vaults.value = []
    activeVault.value = null
    vaultSubscriptionKey.value = null
    loading.value = false
    error.value = null
  }

  return {
    vaults,
    activeVault,
    vaultSubscriptionKey,
    loading,
    error,
    load,
    subscribeList,
    create,
    update,
    remove,
    setActive,
    addServerVault,
    removeServerVault,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)

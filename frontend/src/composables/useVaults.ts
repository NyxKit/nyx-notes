import { ref } from 'vue'
import { fetchVaults, createVault, deleteVault } from '@/api/vaults'
import type { Vault, CreateVaultRequest } from '@/types'

const vaults = ref<Vault[]>([])
const activeVault = ref<Vault | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

export function useVaults() {
  async function load() {
    loading.value = true
    error.value = null
    try {
      vaults.value = await fetchVaults()
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function create(body: CreateVaultRequest) {
    const vault = await createVault(body)
    vaults.value.push(vault)
    return vault
  }

  async function remove(vaultId: string) {
    await deleteVault(vaultId)
    vaults.value = vaults.value.filter(v => v.id !== vaultId)
    if (activeVault.value?.id === vaultId) activeVault.value = null
  }

  function setActive(vault: Vault) {
    activeVault.value = vault
  }

  return {
    vaults,
    activeVault,
    loading,
    error,
    load,
    create,
    remove,
    setActive,
  }
}

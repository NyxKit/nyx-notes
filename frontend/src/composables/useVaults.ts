import { ref } from 'vue'
import { fetchVaults, createVault, deleteVault, patchVaultPermission } from '@/api/vaults'
import { createTeamVault, deleteTeamVault } from '@/api/teams'
import type { Vault, CreateVaultRequest, NotePermission } from '@/types'

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

  function setActive(vault: Vault | null) {
    activeVault.value = vault
  }

  async function patchPermission(teamId: string, vaultId: string, permission: NotePermission) {
    await patchVaultPermission(teamId, vaultId, permission)
    const v = vaults.value.find(v => v.id === vaultId)
    if (v) v.permission = permission
  }

  async function addTeamVault(teamId: string, body: CreateVaultRequest) {
    const vault = await createTeamVault(teamId, body)
    vaults.value.push(vault)
    return vault
  }

  async function removeTeamVault(teamId: string, vaultId: string) {
    await deleteTeamVault(teamId, vaultId)
    vaults.value = vaults.value.filter(v => v.id !== vaultId)
    if (activeVault.value?.id === vaultId) activeVault.value = null
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
    patchPermission,
    addTeamVault,
    removeTeamVault,
  }
}

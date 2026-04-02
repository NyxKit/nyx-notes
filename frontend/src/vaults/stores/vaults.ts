import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { getApiRequestEpoch } from '@/shared/api'
import { fetchVaults, createVault, deleteVault, patchVaultPermission, updateVault } from '@/vaults/api'
import { createTeamVault, deleteTeamVault } from '@/teams/api'
import type { Vault, CreateVaultRequest, UpdateVaultRequest, NotePermission } from '@/shared/types'

export const useVaultStore = defineStore('vaults', () => {
  const vaults = ref<Vault[]>([])
  const activeVault = ref<Vault | null>(null)
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

  async function update(vaultId: string, body: UpdateVaultRequest) {
    const updated = await updateVault(vaultId, body)
    const idx = vaults.value.findIndex(v => v.id === vaultId)
    if (idx !== -1) vaults.value[idx] = updated
    if (activeVault.value?.id === vaultId) activeVault.value = updated
    return updated
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

  function $reset() {
    vaults.value = []
    activeVault.value = null
    loading.value = false
    error.value = null
  }

  return {
    vaults,
    activeVault,
    loading,
    error,
    load,
    create,
    update,
    remove,
    setActive,
    patchPermission,
    addTeamVault,
    removeTeamVault,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)

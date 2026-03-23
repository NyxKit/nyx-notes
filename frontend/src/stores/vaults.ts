import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { fetchVaults, createVault, deleteVault, patchVaultPermission } from '@/api/vaults'
import { createTeamVault, deleteTeamVault } from '@/api/teams'
import type { Vault, CreateVaultRequest, NotePermission } from '@/types'

export const useVaultStore = defineStore('vaults', () => {
  const vaults = ref<Vault[]>([])
  const activeVault = ref<Vault | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

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
    remove,
    setActive,
    patchPermission,
    addTeamVault,
    removeTeamVault,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)

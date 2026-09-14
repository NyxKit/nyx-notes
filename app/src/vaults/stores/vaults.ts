import { ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import type { Vault, CreateVaultRequest, UpdateVaultRequest } from '@/shared/types'

export const useVaultStore = defineStore('vaults', () => {
  const vaults = ref<Vault[]>([])
  const activeVault = ref<Vault | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    loading.value = false
  }

  async function create(body: CreateVaultRequest) {
    void body
    return null as unknown as Vault
  }

  async function remove(vaultId: string) {
    void vaultId
  }

  function setActive(vault: Vault | null) {
    activeVault.value = vault
  }

  async function update(vaultId: string, body: UpdateVaultRequest) {
    void vaultId
    void body
    return null as unknown as Vault
  }

  async function addServerVault(body: CreateVaultRequest) {
    void body
    return null as unknown as Vault
  }

  async function removeServerVault(vaultId: string) {
    void vaultId
  }

  function $reset() {
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
    addServerVault,
    removeServerVault,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useVaultStore, import.meta.hot)

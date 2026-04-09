import { api } from '@/shared/api'
import type { CreateVaultRequest, UpdateVaultRequest } from '@/shared/types'
import { Vault } from '@/shared/types'

export async function fetchVaults() {
  const vaults = await api<unknown[]>('/api/vaults')
  return vaults.map(vault => new Vault(vault))
}

export async function fetchPersonalVaults() {
  const vaults = await api<unknown[]>('/api/vaults/personal')
  return vaults.map(vault => new Vault(vault))
}

export async function fetchServerVaults() {
  const vaults = await api<unknown[]>('/api/server/vaults')
  return vaults.map(vault => new Vault(vault))
}

export async function createVault(body: CreateVaultRequest) {
  return new Vault(await api<unknown>('/api/vaults', { method: 'POST', body }))
}

export function deleteVault(vaultId: string) {
  return api(`/api/vaults/${vaultId}`, { method: 'DELETE' })
}

export async function updateVault(vaultId: string, body: UpdateVaultRequest) {
  return new Vault(await api<unknown>(`/api/vaults/${vaultId}`, { method: 'PATCH', body }))
}

export async function createServerVault(body: CreateVaultRequest) {
  return new Vault(await api<unknown>('/api/server/vaults', { method: 'POST', body }))
}

export function deleteServerVault(vaultId: string) {
  return api(`/api/server/vaults/${vaultId}`, { method: 'DELETE' })
}

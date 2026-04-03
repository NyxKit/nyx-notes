import { api } from '@/shared/api'
import type { Vault, CreateVaultRequest, UpdateVaultRequest } from '@/shared/types'

export function fetchVaults() {
  return api<Vault[]>('/api/vaults')
}

export function fetchPersonalVaults() {
  return api<Vault[]>('/api/vaults/personal')
}

export function fetchServerVaults() {
  return api<Vault[]>('/api/server/vaults')
}

export function createVault(body: CreateVaultRequest) {
  return api<Vault>('/api/vaults', { method: 'POST', body })
}

export function deleteVault(vaultId: string) {
  return api(`/api/vaults/${vaultId}`, { method: 'DELETE' })
}

export function updateVault(vaultId: string, body: UpdateVaultRequest) {
  return api<Vault>(`/api/vaults/${vaultId}`, { method: 'PATCH', body })
}

export function createServerVault(body: CreateVaultRequest) {
  return api<Vault>('/api/server/vaults', { method: 'POST', body })
}

export function deleteServerVault(vaultId: string) {
  return api(`/api/server/vaults/${vaultId}`, { method: 'DELETE' })
}

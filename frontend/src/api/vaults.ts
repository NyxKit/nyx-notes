import { api } from './client'
import type { Vault, CreateVaultRequest, NotePermission } from '@/types'

export function fetchVaults() {
  return api<Vault[]>('/api/vaults')
}

export function createVault(body: CreateVaultRequest) {
  return api<Vault>('/api/vaults', { method: 'POST', body })
}

export function deleteVault(vaultId: string) {
  return api(`/api/vaults/${vaultId}`, { method: 'DELETE' })
}

export function patchVaultPermission(teamId: string, vaultId: string, permission: NotePermission) {
  return api(`/api/teams/${teamId}/vaults/${vaultId}/permission`, {
    method: 'PATCH',
    body: { permission },
  })
}

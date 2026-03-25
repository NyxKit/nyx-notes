import { api } from '@/shared/api/client'
import type { Vault, CreateVaultRequest, UpdateVaultRequest, NotePermission } from '@/shared/types'

export function fetchVaults() {
  return api<Vault[]>('/api/vaults')
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

export function patchVaultPermission(teamId: string, vaultId: string, permission: NotePermission) {
  return api(`/api/teams/${teamId}/vaults/${vaultId}/permission`, {
    method: 'PATCH',
    body: { permission },
  })
}

import { NyxLoader } from 'nyx-kit/classes'
import { NotePermission } from '@/shared/types/enums'
import type { VaultOwner } from '@/shared/types'

const NOTE_PERMISSIONS = Object.values(NotePermission)

export default class Vault {
  id: string
  slug: string
  name: string
  description?: string
  owner: VaultOwner
  permission: NotePermission = NotePermission.Restricted
  icon?: string

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.id = NyxLoader.loadString(data, 'id')
    this.slug = NyxLoader.loadString(data, 'slug')
    this.name = NyxLoader.loadString(data, 'name')
    this.description = NyxLoader.loadStringOrNull(data, 'description', null) ?? undefined
    this.owner = NyxLoader.loadObject(data, 'owner', { type: 'local' }) as VaultOwner
    this.permission = NyxLoader.loadEnum(data, 'permission', NotePermission.Restricted, NOTE_PERMISSIONS)
    this.icon = NyxLoader.loadStringOrNull(data, 'icon', null) ?? undefined
  }
}

import { NyxLoader } from 'nyx-kit/classes'
import { NotePermission } from '@/shared/types/enums'

const NOTE_PERMISSION_VALUES = Object.values(NotePermission)

export default class NoteMeta {
  id: string
  vault_id: string
  title: string
  description?: string
  author_id: string
  images: string[] = []
  tags: string[] = []
  category: string | null = null
  created_at: string
  updated_at: string
  is_encrypted: boolean = false
  permission: NotePermission = NotePermission.Restricted
  feedback_type?: string | null
  app_location?: string | null
  storage_path?: string | null
  console_output?: string | null
  interaction_trail?: string | null

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.id = NyxLoader.loadString(data, 'id')
    this.vault_id = NyxLoader.loadString(data, 'vault_id')
    this.title = NyxLoader.loadString(data, 'title')
    this.description = NyxLoader.loadStringOrNull(data, 'description', null) ?? undefined
    this.author_id = NyxLoader.loadString(data, 'author_id')
    this.images = NyxLoader.loadArray<string>(data, 'images', [])
    this.tags = NyxLoader.loadArray<string>(data, 'tags', [])
    this.category = NyxLoader.loadStringOrNull(data, 'category', null)
    this.created_at = NyxLoader.loadString(data, 'created_at')
    this.updated_at = NyxLoader.loadString(data, 'updated_at')
    this.is_encrypted = NyxLoader.loadBoolean(data, 'is_encrypted', false)
    this.permission = NyxLoader.loadEnum(data, 'permission', NotePermission.Restricted, NOTE_PERMISSION_VALUES)
    this.feedback_type = NyxLoader.loadStringOrNull(data, 'feedback_type', null) ?? undefined
    this.app_location = NyxLoader.loadStringOrNull(data, 'app_location', null) ?? undefined
    this.storage_path = NyxLoader.loadStringOrNull(data, 'storage_path', null) ?? undefined
    this.console_output = NyxLoader.loadStringOrNull(data, 'console_output', null) ?? undefined
    this.interaction_trail = NyxLoader.loadStringOrNull(data, 'interaction_trail', null) ?? undefined
  }
}

import { NyxLoader } from 'nyx-kit/classes'
import { CommentVisibility } from '@/shared/types/enums'
import type { CommentReply } from '@/shared/types'
import CommentAnchor from './CommentAnchor'

const COMMENT_VISIBILITIES = Object.values(CommentVisibility)

export default class Comment {
  id: string
  note_id: string
  author_id: string
  author_name: string
  body: string
  anchor: CommentAnchor
  resolved: boolean = false
  visibility: CommentVisibility = CommentVisibility.Visible
  created_at: string
  updated_at: string
  replies: CommentReply[] = []

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.id = NyxLoader.loadString(data, 'id')
    this.note_id = NyxLoader.loadString(data, 'note_id')
    this.author_id = NyxLoader.loadString(data, 'author_id')
    this.author_name = NyxLoader.loadString(data, 'author_name')
    this.body = NyxLoader.loadString(data, 'body')
    this.anchor = new CommentAnchor(NyxLoader.loadObject(data, 'anchor', {}))
    this.resolved = NyxLoader.loadBoolean(data, 'resolved', false)
    this.visibility = NyxLoader.loadEnum(data, 'visibility', CommentVisibility.Visible, COMMENT_VISIBILITIES)
    this.created_at = NyxLoader.loadString(data, 'created_at')
    this.updated_at = NyxLoader.loadString(data, 'updated_at')
    this.replies = NyxLoader.loadArray<Record<string, unknown>>(data, 'replies', []).map(reply => ({
      id: NyxLoader.loadString(reply, 'id'),
      author_id: NyxLoader.loadString(reply, 'author_id'),
      author_name: NyxLoader.loadString(reply, 'author_name'),
      body: NyxLoader.loadString(reply, 'body'),
      created_at: NyxLoader.loadString(reply, 'created_at'),
    }))
  }
}

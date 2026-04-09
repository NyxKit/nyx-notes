import { NyxLoader } from 'nyx-kit/classes'
import { CommentAttachment } from '@/shared/types/enums'

const COMMENT_ATTACHMENTS = Object.values(CommentAttachment)

export default class CommentAnchor {
  text!: string
  prefix!: string
  suffix!: string
  range_from!: number
  range_to!: number
  attachment: CommentAttachment = CommentAttachment.Attached
  line_preview: string = ''
  last_matched_at?: string

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.text = NyxLoader.loadString(data, 'text')
    this.prefix = NyxLoader.loadString(data, 'prefix')
    this.suffix = NyxLoader.loadString(data, 'suffix')
    this.range_from = NyxLoader.loadNumber(data, 'range_from')
    this.range_to = NyxLoader.loadNumber(data, 'range_to')
    this.attachment = NyxLoader.loadEnum(data, 'attachment', CommentAttachment.Attached, COMMENT_ATTACHMENTS)
    this.line_preview = NyxLoader.loadString(data, 'line_preview', this.line_preview)
    this.last_matched_at = NyxLoader.loadStringOrNull(data, 'last_matched_at', null) ?? undefined
  }
}

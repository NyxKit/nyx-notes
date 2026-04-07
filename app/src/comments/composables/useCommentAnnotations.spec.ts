import { describe, expect, it } from 'vitest'
import { NyxAnnotationAttachment, NyxAnnotationStatus } from 'nyx-kit/types'
import { sortCommentsByAnchor, toCommentAnchor, toNyxAnnotations } from './useCommentAnnotations'
import type { Comment } from '@/shared/types'

function makeComment(overrides: Partial<Comment> = {}): Comment {
  return {
    id: 'comment-1',
    note_id: 'note-1',
    author_id: 'user-1',
    author_name: 'User One',
    body: 'Please clarify this line.',
    anchor: {
      text: 'selected phrase',
      prefix: 'Text before ',
      suffix: ' text after',
      range_from: 10,
      range_to: 25,
      attachment: 'attached',
      line_preview: 'A selected line of note text',
    },
    resolved: false,
    visibility: 'visible',
    created_at: '2026-03-28T10:00:00Z',
    updated_at: '2026-03-28T10:00:00Z',
    replies: [],
    ...overrides,
  }
}

describe('useCommentAnnotations', () => {
  it('converts a nyx annotation anchor into a comment anchor payload', () => {
    const anchor = toCommentAnchor({
      text: 'selected phrase',
      context: {
        prefix: 'Text before ',
        suffix: ' text after',
      },
      range: {
        from: 10,
        to: 25,
      },
    }, 'A selected line of note text')

    expect(anchor).toEqual({
      text: 'selected phrase',
      prefix: 'Text before ',
      suffix: ' text after',
      range_from: 10,
      range_to: 25,
      line_preview: 'A selected line of note text',
    })
  })

  it('maps visible comments into nyx annotations and omits hidden legacy comments', () => {
    const annotations = toNyxAnnotations([
      makeComment(),
      makeComment({
        id: 'comment-hidden',
        visibility: 'hidden_legacy',
        anchor: {
          text: 'legacy quote',
          prefix: '',
          suffix: '',
          range_from: 0,
          range_to: 0,
          attachment: 'detached',
          line_preview: 'legacy quote',
        },
      }),
    ], 'comment-1')

    expect(annotations).toHaveLength(1)
    expect(annotations[0].id).toBe('comment-1')
    expect(annotations[0].status).toBe(NyxAnnotationStatus.Unresolved)
    expect(annotations[0].attachment).toBe(NyxAnnotationAttachment.Attached)
    expect(annotations[0].anchor.text).toBe('selected phrase')
  })

  it('omits resolved annotations unless explicitly requested', () => {
    const resolved = makeComment({
      id: 'comment-resolved',
      resolved: true,
    })

    expect(toNyxAnnotations([resolved])).toHaveLength(0)
    expect(toNyxAnnotations([resolved], undefined, 'resolved')).toHaveLength(1)
    expect(toNyxAnnotations([resolved], undefined, 'resolved')[0].status).toBe(NyxAnnotationStatus.Resolved)
  })

  it('hides unresolved annotations while resolved mode is active', () => {
    const unresolved = makeComment({ id: 'comment-open', resolved: false })

    expect(toNyxAnnotations([unresolved], undefined, 'resolved')).toHaveLength(0)
  })

  it('orders attached comments before detached comments', () => {
    const sorted = sortCommentsByAnchor([
      makeComment({
        id: 'detached',
        anchor: {
          text: 'detached phrase',
          prefix: '',
          suffix: '',
          range_from: 0,
          range_to: 0,
          attachment: 'detached',
          line_preview: 'Detached line',
        },
      }),
      makeComment({
        id: 'attached',
        anchor: {
          text: 'attached phrase',
          prefix: '',
          suffix: '',
          range_from: 4,
          range_to: 18,
          attachment: 'attached',
          line_preview: 'Attached line',
        },
      }),
    ])

    expect(sorted.map(comment => comment.id)).toEqual(['attached', 'detached'])
  })
})

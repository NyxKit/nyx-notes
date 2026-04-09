import {
  NyxAnnotationAttachment,
  NyxAnnotationInteraction,
  NyxAnnotationStatus,
  type NyxAnnotation,
  type NyxAnnotationAnchor,
} from 'nyx-kit/types'
import { CommentAttachment, CommentVisibility } from '@/shared/types'
import type { Comment, CommentAnchor } from '@/shared/types'

export function toCommentAnchor(anchor: NyxAnnotationAnchor, linePreview?: string): Omit<CommentAnchor, 'attachment' | 'last_matched_at'> {
  return {
    text: anchor.text,
    prefix: anchor.context.prefix,
    suffix: anchor.context.suffix,
    range_from: anchor.range.from,
    range_to: anchor.range.to,
    line_preview: linePreview ?? anchor.text,
  }
}

export function toNyxAnnotation(comment: Comment, activeId?: string): NyxAnnotation | null {
  if (comment.visibility === CommentVisibility.HiddenLegacy) {
    return null
  }

  return {
    id: comment.id,
    anchor: {
      text: comment.anchor.text,
      context: {
        prefix: comment.anchor.prefix,
        suffix: comment.anchor.suffix,
      },
      range: {
        from: comment.anchor.range_from,
        to: comment.anchor.range_to,
      },
    },
    interaction: activeId === comment.id ? NyxAnnotationInteraction.Focus : NyxAnnotationInteraction.Default,
    status: comment.resolved ? NyxAnnotationStatus.Resolved : NyxAnnotationStatus.Unresolved,
    attachment: comment.anchor.attachment === CommentAttachment.Detached
      ? NyxAnnotationAttachment.Detached
      : NyxAnnotationAttachment.Attached,
  }
}

export function toNyxAnnotations(
  comments: Comment[],
  activeId?: string,
  resolvedMode: 'open' | 'resolved' = 'open'
): NyxAnnotation[] {
  return comments
    .filter(comment => resolvedMode === 'resolved' ? comment.resolved : !comment.resolved)
    .map(comment => toNyxAnnotation(comment, activeId))
    .filter((annotation): annotation is NyxAnnotation => annotation !== null)
}

export function sortCommentsByAnchor(comments: Comment[]): Comment[] {
  return [...comments].sort((left, right) => {
    const leftDetached = left.anchor.attachment === CommentAttachment.Detached
    const rightDetached = right.anchor.attachment === CommentAttachment.Detached

    if (leftDetached !== rightDetached) {
      return leftDetached ? 1 : -1
    }

    if (left.anchor.range_from !== right.anchor.range_from) {
      return left.anchor.range_from - right.anchor.range_from
    }

    return 0
  })
}

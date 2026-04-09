import { api } from '@/shared/api'
import type { CreateFeedbackRequest, UpdateFeedbackRequest } from '@/shared/types'
import { FeedbackNote, NoteMeta } from '@/shared/types'

export async function fetchFeedbackList() {
  const items = await api<unknown[]>('/api/feedback')
  return items.map(item => new NoteMeta(item))
}

export async function fetchFeedback(id: string) {
  return new FeedbackNote(await api<unknown>(`/api/feedback/${id}`))
}

export async function createFeedback(body: CreateFeedbackRequest) {
  return new NoteMeta(await api<unknown>('/api/feedback', {
    method: 'POST',
    body,
  }))
}

export async function updateFeedback(id: string, body: UpdateFeedbackRequest) {
  return new NoteMeta(await api<unknown>(`/api/feedback/${id}`, {
    method: 'PUT',
    body,
  }))
}

export function deleteFeedback(id: string) {
  return api(`/api/feedback/${id}`, { method: 'DELETE' })
}

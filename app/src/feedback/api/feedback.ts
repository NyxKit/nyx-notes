import { api } from '@/shared/api'
import type { Note, NoteMeta, CreateFeedbackRequest, UpdateFeedbackRequest } from '@/shared/types'

export function fetchFeedbackList() {
  return api<NoteMeta[]>('/api/feedback')
}

export function fetchFeedback(id: string) {
  return api<Note>(`/api/feedback/${id}`)
}

export function createFeedback(body: CreateFeedbackRequest) {
  return api<NoteMeta>('/api/feedback', {
    method: 'POST',
    body,
  })
}

export function updateFeedback(id: string, body: UpdateFeedbackRequest) {
  return api<NoteMeta>(`/api/feedback/${id}`, {
    method: 'PUT',
    body,
  })
}

export function deleteFeedback(id: string) {
  return api(`/api/feedback/${id}`, { method: 'DELETE' })
}

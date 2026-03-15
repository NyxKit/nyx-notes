import { api } from './client'
import type { Team, CreateTeamRequest, AddMemberRequest, PatchMemberRequest } from '@/types'

export function fetchTeams() {
  return api<Team[]>('/api/teams')
}

export function fetchTeam(teamId: string) {
  return api<Team>(`/api/teams/${teamId}`)
}

export function createTeam(body: CreateTeamRequest) {
  return api<Team>('/api/teams', { method: 'POST', body })
}

export function deleteTeam(teamId: string) {
  return api(`/api/teams/${teamId}`, { method: 'DELETE' })
}

export function addMember(teamId: string, body: AddMemberRequest) {
  return api(`/api/teams/${teamId}/members`, { method: 'POST', body })
}

export function patchMember(teamId: string, userId: string, body: PatchMemberRequest) {
  return api(`/api/teams/${teamId}/members/${userId}`, { method: 'PATCH', body })
}

export function removeMember(teamId: string, userId: string) {
  return api(`/api/teams/${teamId}/members/${userId}`, { method: 'DELETE' })
}

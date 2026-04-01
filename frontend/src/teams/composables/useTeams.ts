import { ref } from 'vue'
import { getApiRequestEpoch } from '@/shared/api'
import {
  fetchTeams,
  fetchTeam,
  createTeam,
  deleteTeam,
  addMember,
  patchMember,
  removeMember,
} from '@/teams/api'
import type { Team, TeamRole } from '@/shared/types'

const teams = ref<Team[]>([])

export function useTeams() {
  async function load() {
    const requestEpoch = getApiRequestEpoch()
    const nextTeams = await fetchTeams()
    if (requestEpoch !== getApiRequestEpoch()) return
    teams.value = nextTeams
  }

  async function loadOne(teamId: string) {
    const requestEpoch = getApiRequestEpoch()
    const team = await fetchTeam(teamId)
    if (requestEpoch !== getApiRequestEpoch()) return team
    const idx = teams.value.findIndex(t => t.id === teamId)
    if (idx !== -1) teams.value[idx] = team
    else teams.value.push(team)
    return team
  }

  function teamName(teamId: string): string {
    return teams.value.find(t => t.id === teamId)?.name ?? teamId
  }

  async function create(name: string) {
    const team = await createTeam({ name })
    teams.value.push(team)
    return team
  }

  async function remove(teamId: string) {
    await deleteTeam(teamId)
    teams.value = teams.value.filter(t => t.id !== teamId)
  }

  async function addTeamMember(teamId: string, userId: string, role: Exclude<TeamRole, 'owner'>) {
    await addMember(teamId, { user_id: userId, role })
    const team = teams.value.find(t => t.id === teamId)
    if (team) team.members.push({ user_id: userId, role })
  }

  async function updateMember(teamId: string, userId: string, role: Exclude<TeamRole, 'owner'>) {
    await patchMember(teamId, userId, { role })
    const team = teams.value.find(t => t.id === teamId)
    if (team) {
      const m = team.members.find(m => m.user_id === userId)
      if (m) m.role = role
    }
  }

  async function kickMember(teamId: string, userId: string) {
    await removeMember(teamId, userId)
    const team = teams.value.find(t => t.id === teamId)
    if (team) team.members = team.members.filter(m => m.user_id !== userId)
  }

  function clear() {
    teams.value = []
  }

  return {
    teams,
    load,
    loadOne,
    teamName,
    create,
    remove,
    addTeamMember,
    updateMember,
    kickMember,
    clear,
  }
}

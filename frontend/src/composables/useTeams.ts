import { ref } from 'vue'
import { fetchTeams } from '@/api/teams'
import type { Team } from '@/types'

const teams = ref<Team[]>([])

export function useTeams() {
  async function load() {
    teams.value = await fetchTeams()
  }

  function teamName(teamId: string): string {
    return teams.value.find(t => t.id === teamId)?.name ?? teamId
  }

  return { teams, load, teamName }
}

import { readonly, ref } from 'vue'

const teams = ref([])

export function useTeams() {
  function unsupported(): never {
    throw new Error('Team management is not available in the MVP frontend')
  }

  return {
    teams: readonly(teams),
    load: async () => {},
    loadOne: async () => unsupported(),
    teamName: (teamId: string) => teamId,
    create: async () => unsupported(),
    remove: async () => unsupported(),
    addTeamMember: async () => unsupported(),
    updateMember: async () => unsupported(),
    kickMember: async () => unsupported(),
    clear: () => {
      teams.value = []
    },
  }
}

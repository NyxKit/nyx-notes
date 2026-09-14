import { computed } from 'vue'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { WorkspaceProfileType } from '@/shared/types'
import type { AnyWorkspaceProfile, FavoriteNoteRef } from '@/shared/types'

function emptyResult() {
  return { results: [], excluded_profiles_count: 0 }
}

export function getServerLabel(profile: AnyWorkspaceProfile) {
  if (profile.type === WorkspaceProfileType.Remote) {
    return profile.server_label || profile.display_name || profile.server_url
  }

  return profile.display_name || 'Main Server'
}

export function useGlobalNoteBrowsing() {
  const { profiles } = useWorkspaceProfiles()
  const { sessions } = useAuth()

  const eligibleProfiles = computed(() =>
    profiles.value.filter(profile => {
      const session = sessions.value[profile.id]
      const sessionToken = session?.state === 'signed_in' ? session.token : undefined
      if (profile.type === WorkspaceProfileType.Local) return true
      return Boolean(sessionToken)
    }),
  )

  const configuredProfileCount = computed(() => profiles.value.length)

  async function loadSearchResults(_query: string, _favoriteRefs: FavoriteNoteRef[]) {
    void _query
    void _favoriteRefs
    return emptyResult()
  }

  async function loadFavoriteResults(_favoriteRefs: FavoriteNoteRef[]) {
    void _favoriteRefs
    return emptyResult()
  }

  async function loadRecentResults(_favoriteRefs: FavoriteNoteRef[]) {
    void _favoriteRefs
    return emptyResult()
  }

  return {
    eligibleProfiles,
    configuredProfileCount,
    loadRecentResults,
    loadSearchResults,
    loadFavoriteResults,
  }
}

import { computed } from 'vue'
import { ofetch } from 'ofetch'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import type {
  AnyWorkspaceProfile,
  BrowseNoteCardModel,
  FavoriteNoteRef,
  Note,
  NoteMeta,
  Vault,
} from '@/shared/types'

interface ProfileBrowseClient {
  fetchVaults: () => Promise<Vault[]>
  fetchNotes: (vaultId: string) => Promise<NoteMeta[]>
  fetchNote: (vaultId: string, noteId: string) => Promise<Note>
}

interface EligibleProfileContext {
  profile: AnyWorkspaceProfile
  client: ProfileBrowseClient
}

interface SearchLoadResult {
  results: BrowseNoteCardModel[]
  excluded_profiles_count: number
}

interface FavoritesLoadResult {
  results: BrowseNoteCardModel[]
  excluded_profiles_count: number
}

interface RecentLoadResult {
  results: BrowseNoteCardModel[]
  excluded_profiles_count: number
}

function formatUpdatedLabel(iso: string) {
  const updatedAt = new Date(iso)
  const diff = Date.now() - updatedAt.getTime()
  const minutes = Math.floor(diff / 60_000)

  if (minutes < 1) return 'just now'
  if (minutes < 60) return `${minutes}m ago`

  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}h ago`

  const days = Math.floor(hours / 24)
  if (days < 7) return `${days}d ago`

  return updatedAt.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
}

function getServerLabel(profile: AnyWorkspaceProfile) {
  if (profile.type === 'remote') {
    return profile.server_label || profile.display_name || profile.server_url
  }

  return profile.display_name || 'Local'
}

function makeProfileClient(profile: AnyWorkspaceProfile, token?: string): ProfileBrowseClient {
  const client = ofetch.create({
    baseURL: profile.type === 'remote' ? profile.server_url : import.meta.env.VITE_API_BASE_URL ?? '/',
    headers: token ? { Authorization: `Bearer ${token}` } : undefined,
  })

  return {
    fetchVaults: () => client<Vault[]>('/api/vaults'),
    fetchNotes: (vaultId: string) => client<NoteMeta[]>(`/api/vaults/${vaultId}/notes`),
    fetchNote: (vaultId: string, noteId: string) => client<Note>(`/api/vaults/${vaultId}/notes/${noteId}`),
  }
}

function makeHref(profileId: string, vaultId: string, noteId: string) {
  return `/vaults/${vaultId}/notes/${noteId}?profile=${encodeURIComponent(profileId)}`
}

function computeMatchScore(query: string, note: NoteMeta, content?: string) {
  const normalized = query.trim().toLowerCase()
  if (!normalized) return 0

  let score = 0
  const title = note.title.toLowerCase()
  const tags = note.tags.map(tag => tag.toLowerCase())
  const body = content?.toLowerCase() ?? ''

  if (title.includes(normalized)) score += 100
  if (tags.some(tag => tag.includes(normalized))) score += 50
  if (body.includes(normalized)) score += 20

  return score
}

function buildBrowseNote(
  note: NoteMeta,
  profile: AnyWorkspaceProfile,
  vault: Vault,
  isFavorite: boolean,
  matchScore?: number,
): BrowseNoteCardModel {
  return {
    note_id: note.id,
    vault_id: note.vault_id,
    profile_id: profile.id,
    title: note.title || 'Untitled',
    description: note.description,
    tags: note.tags,
    updated_at: note.updated_at,
    updated_label: formatUpdatedLabel(note.updated_at),
    href: makeHref(profile.id, note.vault_id, note.id),
    server_label: getServerLabel(profile),
    server_id: profile.type === 'remote' ? profile.server_id : undefined,
    vault_name: vault.name,
    vault_slug: vault.slug,
    is_favorite: isFavorite,
    match_score: matchScore,
  }
}

export function useGlobalNoteBrowsing() {
  const { profiles } = useWorkspaceProfiles()
  const { sessions } = useAuth()

  const eligibleProfiles = computed((): EligibleProfileContext[] => {
    const contexts: EligibleProfileContext[] = []

    for (const profile of profiles.value) {
      if (profile.type === 'local') {
        contexts.push({ profile, client: makeProfileClient(profile) })
        continue
      }

      const session = sessions.value[profile.id]
      if (session?.state !== 'signed_in' || !session.token) {
        continue
      }

      contexts.push({ profile, client: makeProfileClient(profile, session.token) })
    }

    return contexts
  })

  const configuredProfileCount = computed(() => profiles.value.length)

  async function loadSearchResults(query: string, favoriteRefs: FavoriteNoteRef[]): Promise<SearchLoadResult> {
    const trimmedQuery = query.trim()
    const favoriteKeySet = new Set(favoriteRefs.map(ref => `${ref.profile_id}:${ref.vault_id}:${ref.note_id}`))

    if (!trimmedQuery) {
      return {
        results: [],
        excluded_profiles_count: configuredProfileCount.value - eligibleProfiles.value.length,
      }
    }

    const results: BrowseNoteCardModel[] = []
    let excludedProfilesCount = configuredProfileCount.value - eligibleProfiles.value.length

    for (const { profile, client } of eligibleProfiles.value) {
      try {
        const vaults = await client.fetchVaults()
        for (const vault of vaults) {
          const notes = await client.fetchNotes(vault.id)
          const noteCards = await Promise.all(notes.map(async note => {
            let matchScore = computeMatchScore(trimmedQuery, note)
            if (matchScore === 0) {
              const fullNote = await client.fetchNote(vault.id, note.id)
              matchScore = computeMatchScore(trimmedQuery, note, fullNote.content)
            }

            if (matchScore === 0) return null

            return buildBrowseNote(
              note,
              profile,
              vault,
              favoriteKeySet.has(`${profile.id}:${note.vault_id}:${note.id}`),
              matchScore,
            )
          }))

          results.push(...noteCards.filter((note): note is BrowseNoteCardModel => note !== null))
        }
      } catch {
        excludedProfilesCount += 1
      }
    }

    return {
      results,
      excluded_profiles_count: excludedProfilesCount,
    }
  }

  async function loadFavoriteResults(favoriteRefs: FavoriteNoteRef[]): Promise<FavoritesLoadResult> {
    const results: BrowseNoteCardModel[] = []
    let excludedProfilesCount = configuredProfileCount.value - eligibleProfiles.value.length
    const groupedRefs = new Map<string, FavoriteNoteRef[]>()

    for (const ref of favoriteRefs) {
      const key = ref.profile_id || '__legacy__'
      groupedRefs.set(key, [...(groupedRefs.get(key) ?? []), ref])
    }

    for (const { profile, client } of eligibleProfiles.value) {
      const refs = groupedRefs.get(profile.id) ?? []
      const legacyRefs = groupedRefs.get('__legacy__') ?? []

      try {
        const vaults = await client.fetchVaults()
        const vaultMap = new Map(vaults.map(vault => [vault.id, vault]))

        for (const ref of refs) {
          const vault = vaultMap.get(ref.vault_id)
          if (!vault) continue
          const note = await client.fetchNote(ref.vault_id, ref.note_id)
          results.push(buildBrowseNote(note.meta, profile, vault, true))
        }

        if (legacyRefs.length > 0) {
          for (const vault of vaults) {
            const notes = await client.fetchNotes(vault.id)
            for (const note of notes) {
              if (!legacyRefs.some(ref => ref.note_id === note.id)) continue
              results.push(buildBrowseNote(note, profile, vault, true))
            }
          }
        }
      } catch {
        excludedProfilesCount += 1
      }
    }

    return {
      results,
      excluded_profiles_count: excludedProfilesCount,
    }
  }

  async function loadRecentResults(favoriteRefs: FavoriteNoteRef[]): Promise<RecentLoadResult> {
    const results: BrowseNoteCardModel[] = []
    let excludedProfilesCount = configuredProfileCount.value - eligibleProfiles.value.length
    const favoriteKeySet = new Set(favoriteRefs.map(ref => `${ref.profile_id}:${ref.vault_id}:${ref.note_id}`))

    for (const { profile, client } of eligibleProfiles.value) {
      try {
        const vaults = await client.fetchVaults()
        for (const vault of vaults) {
          const notes = await client.fetchNotes(vault.id)
          results.push(...notes.map(note => buildBrowseNote(
            note,
            profile,
            vault,
            favoriteKeySet.has(`${profile.id}:${note.vault_id}:${note.id}`),
          )))
        }
      } catch {
        excludedProfilesCount += 1
      }
    }

    return {
      results,
      excluded_profiles_count: excludedProfilesCount,
    }
  }

  return {
    eligibleProfiles,
    configuredProfileCount,
    loadRecentResults,
    loadSearchResults,
    loadFavoriteResults,
  }
}

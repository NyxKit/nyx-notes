import type { AnyWorkspaceProfile, StoredWorkspaceProfiles } from '@/shared/types'
import { WorkspaceProfileType } from '@/shared/types'

const STORAGE_KEY = 'nyx_workspace_profiles'
const LOCAL_PROFILE_ID = 'local'

function defaultProfiles(): StoredWorkspaceProfiles {
  return {
    active_profile_id: null,
    profiles: [],
  }
}

export function normalizeServerUrl(serverUrl: string) {
  const trimmed = serverUrl.trim()
  if (!trimmed) return ''

  try {
    const url = new URL(trimmed.includes('://') ? trimmed : `https://${trimmed}`)
    url.hash = ''
    url.search = ''
    url.pathname = url.pathname.replace(/\/+$/, '') || '/'
    return url.toString().replace(/\/$/, '')
  } catch {
    return trimmed
  }
}

export function readStoredProfiles(): StoredWorkspaceProfiles {
  const raw = localStorage.getItem(STORAGE_KEY)
  if (!raw) return defaultProfiles()

  try {
    const parsed = JSON.parse(raw) as StoredWorkspaceProfiles
    if (!Array.isArray(parsed.profiles) || parsed.profiles.length === 0) {
      return defaultProfiles()
    }

    if (!parsed.active_profile_id) {
      parsed.active_profile_id = parsed.profiles[0]?.id ?? null
    }

    return parsed
  } catch {
    return defaultProfiles()
  }
}

export function writeStoredProfiles(state: StoredWorkspaceProfiles) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(state))
}

export function createProfileId() {
  if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
    return crypto.randomUUID()
  }

  return `profile-${Date.now()}-${Math.random().toString(16).slice(2)}`
}

export function remoteProfileKey(serverUrl: string, username: string) {
  return `${normalizeServerUrl(serverUrl)}::${username.trim().toLowerCase()}`
}

export function sortProfiles(profiles: AnyWorkspaceProfile[]) {
  return [...profiles].sort((left, right) => {
    if (left.type !== right.type) return left.type === WorkspaceProfileType.Local ? -1 : 1
    return left.display_name.localeCompare(right.display_name)
  })
}

export { LOCAL_PROFILE_ID }

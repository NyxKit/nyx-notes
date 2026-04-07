import { computed, ref } from 'vue'
import type {
  AnyWorkspaceProfile,
  ProfileActivationResult,
  RemoteProfileDraft,
  RemoteWorkspaceProfile,
} from '@/shared/types'
import {
  LOCAL_PROFILE_ID,
  createProfileId,
  normalizeServerUrl,
  readStoredProfiles,
  remoteProfileKey,
  sortProfiles,
  writeStoredProfiles,
} from '@/shared/utils'
import { clearProfilePassword, storeProfilePassword } from '@/shared/utils'

const stored = readStoredProfiles()

// Auto-activate local profile if none exists (MVP: single local server)
let initialProfiles = sortProfiles(stored.profiles)
let initialActiveId = stored.active_profile_id

if (!initialActiveId || !initialProfiles.some(p => p.id === initialActiveId)) {
  const existingLocal = initialProfiles.find(p => p.id === LOCAL_PROFILE_ID)
  if (!existingLocal) {
    initialProfiles = sortProfiles([
      ...initialProfiles,
      { id: LOCAL_PROFILE_ID, type: 'local', display_name: 'Main Server' },
    ])
  }
  initialActiveId = LOCAL_PROFILE_ID
  writeStoredProfiles({
    active_profile_id: initialActiveId,
    profiles: initialProfiles,
  })
}

const profiles = ref<AnyWorkspaceProfile[]>(initialProfiles)
const activeProfileId = ref<string | null>(initialActiveId)

function persist() {
  writeStoredProfiles({
    active_profile_id: activeProfileId.value,
    profiles: profiles.value,
  })
}

function touchProfile(profileId: string, updates: Partial<AnyWorkspaceProfile>) {
  const index = profiles.value.findIndex(profile => profile.id === profileId)
  if (index === -1) return

  profiles.value[index] = {
    ...profiles.value[index],
    ...updates,
  } as AnyWorkspaceProfile
  profiles.value = sortProfiles(profiles.value)
  persist()
}

export function useWorkspaceProfiles() {
  const activeProfile = computed(() =>
    profiles.value.find(profile => profile.id === activeProfileId.value) ?? null
  )

  const remoteProfiles = computed(() =>
    profiles.value.filter((profile): profile is RemoteWorkspaceProfile => profile.type === 'remote')
  )

  function setActiveProfile(profileId: string): ProfileActivationResult | null {
    const profile = profiles.value.find(item => item.id === profileId)
    if (!profile) return null

    activeProfileId.value = profileId
    touchProfile(profileId, { last_used_at: new Date().toISOString() })

    return {
      profile,
      requires_login: profile.type === 'remote',
    }
  }

  function createLocalProfile() {
    const existingLocal = profiles.value.find(profile => profile.id === LOCAL_PROFILE_ID)
    if (!existingLocal) {
      profiles.value = sortProfiles([
        ...profiles.value,
        { id: LOCAL_PROFILE_ID, type: 'local', display_name: 'Main Server' },
      ])
    }

    activeProfileId.value = LOCAL_PROFILE_ID
    persist()
    return profiles.value.find(profile => profile.id === LOCAL_PROFILE_ID) ?? null
  }

  function addRemoteProfile(draft: RemoteProfileDraft) {
    const normalizedUrl = normalizeServerUrl(draft.server_url)
    const key = remoteProfileKey(normalizedUrl, draft.username)
    const duplicate = remoteProfiles.value.find(profile =>
      remoteProfileKey(profile.server_url, profile.username) === key
    )

    if (duplicate) {
      throw new Error('A profile for this server and username already exists')
    }

    const profile: RemoteWorkspaceProfile = {
      id: createProfileId(),
      type: 'remote',
      display_name: draft.display_name.trim() || draft.username.trim() || normalizedUrl,
      server_url: normalizedUrl,
      username: draft.username.trim(),
      connection_status: 'unknown',
    }

    profiles.value = sortProfiles([...profiles.value, profile])
    activeProfileId.value = profile.id
    storeProfilePassword(profile.id, draft.password)
    persist()
    return profile
  }

  function updateRemoteProfile(profileId: string, draft: RemoteProfileDraft) {
    const current = remoteProfiles.value.find(profile => profile.id === profileId)
    if (!current) throw new Error('Remote profile not found')

    const normalizedUrl = normalizeServerUrl(draft.server_url)
    const key = remoteProfileKey(normalizedUrl, draft.username)
    const duplicate = remoteProfiles.value.find(profile =>
      profile.id !== profileId && remoteProfileKey(profile.server_url, profile.username) === key
    )

    if (duplicate) {
      throw new Error('A profile for this server and username already exists')
    }

    touchProfile(profileId, {
      display_name: draft.display_name.trim() || draft.username.trim() || normalizedUrl,
      server_url: normalizedUrl,
      username: draft.username.trim(),
      connection_status: 'unknown',
      last_error: undefined,
    })
    storeProfilePassword(profileId, draft.password)
  }

  function removeProfile(profileId: string) {
    if (profileId === LOCAL_PROFILE_ID) {
      throw new Error('The local profile cannot be removed')
    }

    profiles.value = profiles.value.filter(profile => profile.id !== profileId)
    clearProfilePassword(profileId)

    if (activeProfileId.value === profileId) {
      activeProfileId.value = profiles.value[0]?.id ?? null
    }

    persist()
  }

  function updateProfileRoute(profileId: string, route: string) {
    touchProfile(profileId, { last_route: route })
  }

  function updateRemoteProfileStatus(
    profileId: string,
    updates: Partial<Pick<RemoteWorkspaceProfile, 'auth_mode' | 'connection_status' | 'server_id' | 'server_label' | 'api_version' | 'last_error'>>
  ) {
    touchProfile(profileId, updates as Partial<AnyWorkspaceProfile>)
  }

  function updateProfileDisplayName(profileId: string, displayName: string) {
    touchProfile(profileId, { display_name: displayName })
  }

  return {
    profiles,
    remoteProfiles,
    activeProfileId,
    activeProfile,
    createLocalProfile,
    addRemoteProfile,
    updateRemoteProfile,
    updateRemoteProfileStatus,
    updateProfileDisplayName,
    setActiveProfile,
    removeProfile,
    updateProfileRoute,
  }
}

const PROFILE_PASSWORD_PREFIX = 'nyx_workspace_profile_password:'

function passwordKey(profileId: string) {
  return `${PROFILE_PASSWORD_PREFIX}${profileId}`
}

export function storeProfilePassword(profileId: string, password: string) {
  localStorage.setItem(passwordKey(profileId), password)
}

export function readProfilePassword(profileId: string) {
  return localStorage.getItem(passwordKey(profileId))
}

export function clearProfilePassword(profileId: string) {
  localStorage.removeItem(passwordKey(profileId))
}

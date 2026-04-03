import type { Vault } from '@/shared/types'

type PathRoute = { path: string }

export function vaultRoute(vault: Pick<Vault, 'slug' | 'owner'>): PathRoute {
  if (vault.owner.type === 'home') {
    return {
      path: `/${vault.owner.server_slug}/homes/${vault.owner.home_slug}/vaults/${vault.slug}`,
    }
  }

  if (vault.owner.type === 'server') {
    return {
      path: `/${vault.owner.server_slug}/vaults/${vault.slug}`,
    }
  }

  return { path: `/vaults/${vault.slug}` }
}

export function noteRoute(vault: Pick<Vault, 'slug' | 'owner'>, noteId: string): PathRoute {
  const base = vaultRoute(vault)
  return {
    path: `${base.path}/notes/${noteId}`,
  }
}

export function vaultSettingsRoute(vault: Pick<Vault, 'slug' | 'owner'>): PathRoute {
  const base = vaultRoute(vault)
  return {
    path: `${base.path}/settings`,
  }
}

export function vaultCrumbRouteFromParams(
  serverSlug: string | undefined,
  homeSlug: string | undefined,
  vaultSlug: string,
): PathRoute {
  if (serverSlug && homeSlug) {
    return { path: `/${serverSlug}/homes/${homeSlug}/vaults/${vaultSlug}` }
  }

  if (serverSlug) {
    return { path: `/${serverSlug}/vaults/${vaultSlug}` }
  }

  return { path: `/vaults/${vaultSlug}` }
}

export function noteCrumbRouteFromParams(
  serverSlug: string | undefined,
  homeSlug: string | undefined,
  vaultSlug: string,
  noteId: string,
): PathRoute {
  return {
    path: `${vaultCrumbRouteFromParams(serverSlug, homeSlug, vaultSlug).path}/notes/${noteId}`,
  }
}

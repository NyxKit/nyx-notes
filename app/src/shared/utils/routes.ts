import { RouteName } from '@/shared/types'
import type { Vault } from '@/shared/types'
import type { RouteLocationRaw } from 'vue-router'

export function vaultRoute(vault: Pick<Vault, 'slug' | 'owner'>): RouteLocationRaw {
  if (vault.owner.type === 'home') {
    return {
      name: RouteName.UserVault,
      params: {
        server_slug: vault.owner.server_slug,
        home_slug: vault.owner.home_slug,
        vault_id: vault.slug,
      },
    }
  }

  if (vault.owner.type === 'server') {
    return {
      name: RouteName.ServerVault,
      params: {
        server_slug: vault.owner.server_slug,
        vault_id: vault.slug,
      },
    }
  }

  return { name: RouteName.Home }
}

export function noteRoute(vault: Pick<Vault, 'slug' | 'owner'>, noteId?: string): RouteLocationRaw {
  if (vault.owner.type === 'home') {
    return {
      name: RouteName.UserNote,
      params: {
        server_slug: vault.owner.server_slug,
        home_slug: vault.owner.home_slug,
        vault_id: vault.slug,
        ...(noteId ? { id: noteId } : {}),
      },
    }
  }

  if (vault.owner.type === 'server') {
    return {
      name: RouteName.ServerNote,
      params: {
        server_slug: vault.owner.server_slug,
        vault_id: vault.slug,
        ...(noteId ? { id: noteId } : {}),
      },
    }
  }

  return { name: RouteName.Home }
}

export function vaultSettingsRoute(vault: Pick<Vault, 'slug' | 'owner'>): RouteLocationRaw {
  if (vault.owner.type === 'home') {
    return {
      name: RouteName.UserVaultSettings,
      params: {
        server_slug: vault.owner.server_slug,
        home_slug: vault.owner.home_slug,
        vault_id: vault.slug,
      },
    }
  }

  if (vault.owner.type === 'server') {
    return {
      name: RouteName.ServerVaultSettings,
      params: {
        server_slug: vault.owner.server_slug,
        vault_id: vault.slug,
      },
    }
  }

  return { name: RouteName.Settings }
}

export function vaultCrumbRouteFromParams(
  serverSlug: string | undefined,
  homeSlug: string | undefined,
  vaultSlug: string,
): RouteLocationRaw {
  if (serverSlug && homeSlug) {
    return {
      name: RouteName.UserVault,
      params: { server_slug: serverSlug, home_slug: homeSlug, vault_id: vaultSlug },
    }
  }

  if (serverSlug) {
    return {
      name: RouteName.ServerVault,
      params: { server_slug: serverSlug, vault_id: vaultSlug },
    }
  }

  return { name: RouteName.Home }
}

export function noteCrumbRouteFromParams(
  serverSlug: string | undefined,
  homeSlug: string | undefined,
  vaultSlug: string,
  noteId: string,
): RouteLocationRaw {
  if (serverSlug && homeSlug) {
    return {
      name: RouteName.UserNote,
      params: { server_slug: serverSlug, home_slug: homeSlug, vault_id: vaultSlug, id: noteId },
    }
  }

  if (serverSlug) {
    return {
      name: RouteName.ServerNote,
      params: { server_slug: serverSlug, vault_id: vaultSlug, id: noteId },
    }
  }

  return { name: RouteName.Home }
}

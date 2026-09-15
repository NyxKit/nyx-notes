import { watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useNotesStore } from '@/notes/stores'
import { useVaultStore } from '@/vaults/stores'
import { RouteName } from '@/shared/types'

const pageLabels: Partial<Record<RouteName, string>> = {
  [RouteName.UserRoot]: 'Personal',
  [RouteName.ServerRoot]: 'Shared Vaults',
  [RouteName.UserVaultSettings]: 'Vault Settings',
  [RouteName.ServerVaultSettings]: 'Vault Settings',
  [RouteName.Search]: 'Search',
  [RouteName.Favorites]: 'Favorites',
  [RouteName.Users]: 'Users',
  [RouteName.Settings]: 'Settings',
  [RouteName.Login]: 'Sign In',
  [RouteName.Setup]: 'Setup',
}

export function usePageTitle() {
  const route = useRoute()
  const notes = useNotesStore()
  const vaults = useVaultStore()

  watchEffect(() => {
    const name = route.name as RouteName
    let title = pageLabels[name]
    const isNote = name === RouteName.UserNote || name === RouteName.ServerNote
    const isVault = name === RouteName.UserVault || name === RouteName.ServerVault

    if (isNote || isVault) {
      const vault = vaults.vaults.find(vault => {
        if (vault.slug !== route.params.vault_id && vault.id !== route.params.vault_id) return false
        if (vault.owner.type === 'home') {
          return vault.owner.server_slug === route.params.server_slug
            && vault.owner.home_slug === route.params.home_slug
        }
        return vault.owner.type === 'server'
          && vault.owner.server_slug === route.params.server_slug
          && !route.params.home_slug
      })

      if (isNote && route.params.id) {
        const note = notes.activeNote
        if (vault && note?.meta.id === route.params.id
          && (note.meta.vault_id === vault.id || note.meta.vault_id === vault.slug)) {
          title = note.meta.title.trim() || 'Untitled'
        }
      } else {
        title = vault?.name.trim() || undefined
      }
    }

    document.title = title ? `${title} | Nyx Notes` : 'Nyx Notes'
  })
}

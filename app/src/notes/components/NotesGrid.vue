<script setup lang="ts">
import { computed } from 'vue'
import { NyxGrid } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import { useNotes } from '@/notes/composables'
import { noteRoute } from '@/shared/utils'
import { VaultOwnerType } from '@/shared/types'
import type { BrowseNoteCardModel, NoteMeta, Vault } from '@/shared/types'
import NoteCard from './NoteCard.vue'

const props = withDefaults(defineProps<{
  title: string
  notes?: Array<BrowseNoteCardModel | NoteMeta>
  vault?: Vault
  vaultId?: string
  serverSlug?: string
  serverLabel?: string
  mode?: NyxGridMode
  columns?: number
}>(), {
  mode: NyxGridMode.Masonry,
  columns: 5,
})

const notesScope = useNotes(
  () => props.vaultId,
  () => props.serverSlug,
)

const normalizedNotes = computed<BrowseNoteCardModel[]>(() => {
  const notes = props.notes ?? notesScope.notes.value

  return notes.map((note) => {
    if ('note_id' in note) return note

    const vault = props.vault
    return {
      note_id: note.id,
      vault_id: note.vault_id,
      profile_id: props.serverSlug ?? 'local',
      title: note.title || 'Untitled',
      description: note.description,
      tags: note.tags,
      images: note.images ?? [],
      updated_at: note.updated_at,
      updated_label: formatUpdatedLabel(note.updated_at),
      href: noteRoute(vault ?? {
        slug: note.vault_id,
        owner: { type: VaultOwnerType.Server, server_slug: props.serverSlug ?? 'main-server' },
      }, note.id),
      server_label: props.serverLabel ?? props.serverSlug ?? 'Main Server',
      server_id: vault?.owner.type === VaultOwnerType.Server ? vault.owner.server_slug : undefined,
      vault_name: vault?.name ?? note.vault_id,
      vault_slug: note.vault_id,
      is_favorite: false,
    }
  })
})

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

const loading = computed(() => false)
</script>

<template>
  <div class="notes-grid">
    <slot name="loading" v-if="loading" />

    <NyxGrid v-else-if="normalizedNotes.length > 0" :title="title" :mode="mode" :columns="columns">
      <template v-for="note in normalizedNotes" :key="note.note_id">
        <slot name="card" :note="note">
          <NoteCard :note="note" :image="note.images?.[0]" />
        </slot>
      </template>
    </NyxGrid>

    <slot v-else name="empty" />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxSelect } from 'nyx-kit/components'
import { NyxSize } from 'nyx-kit/types'
import type { NyxSelectOptionGroup } from 'nyx-kit/types'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { useTeams } from '@/teams/composables'
import { RouteName } from '@/shared/types'
import type { Vault } from '@/shared/types'

const props = withDefaults(defineProps<{ dest?: 'notes' | 'vault' }>(), { dest: 'notes' })

const router = useRouter()
const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { setActive } = vaultStore
const { load: loadTeams, teamName } = useTeams()
const { notesFor } = useNotesStore()

onMounted(loadTeams)

const personalVaults = computed(() =>
  vaults.value.filter(v => v.owner.type === 'user')
)

const teamGroups = computed(() => {
  const groups = new Map<string, Vault[]>()
  for (const v of vaults.value.filter(v => v.owner.type === 'team')) {
    const tid = v.owner.id
    if (!groups.has(tid)) groups.set(tid, [])
    groups.get(tid)!.push(v)
  }
  return groups
})

const vaultSelectOptions = computed((): NyxSelectOptionGroup[] => {
  const groups: NyxSelectOptionGroup[] = []
  if (personalVaults.value.length) {
    groups.push({
      label: 'Personal',
      options: personalVaults.value.map(v => ({ label: v.name, value: v.id }))
    })
  }
  for (const [tid, tvaults] of teamGroups.value.entries()) {
    groups.push({
      label: teamName(tid),
      options: tvaults.map(v => ({ label: v.name, value: v.id }))
    })
  }
  return groups
})

const selectedVaultId = computed({
  get: () => activeVault.value?.id ?? '',
  set: (id: string) => {
    const vault = vaults.value.find(v => v.id === id)
    if (vault) select(vault)
  }
})

const noteCountLabel = computed(() => {
  const n = activeVault.value ? notesFor(activeVault.value.id).length : 0
  return `${n} ${n === 1 ? 'note' : 'notes'}`
})

function select(vault: Vault) {
  setActive(vault)
  const target = props.dest === 'vault'
    ? { name: RouteName.Vault, params: { vault_id: vault.id } }
    : { name: RouteName.Note, params: { vault_id: vault.id } }
  router.push(target)
}
</script>

<template>
  <div class="vault-switcher">
    <div class="vault-switcher__card">
      <div class="vault-switcher__card-row">
        <NyxSelect
          v-model="selectedVaultId"
          :options="vaultSelectOptions"
          :size="NyxSize.Small"
          class="vault-switcher__select"
        />
      </div>
      <span class="vault-switcher__count">{{ noteCountLabel }}</span>
    </div>

    <RouterLink
      v-if="activeVault?.owner.type === 'team'"
      :to="{ name: RouteName.TeamSettings, params: { team_id: activeVault.owner.id } }"
      class="vault-switcher__team-link"
    >
      Team settings ›
    </RouterLink>
  </div>
</template>

<style scoped>
.vault-switcher {
  padding: 1rem;
  flex-shrink: 0;
}

.vault-switcher__card {
  background: rgba(37, 37, 43, 0.6);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-radius: var(--nyx-radius-lg);
  padding: 0.875rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  transition: background 0.3s;
}

.vault-switcher__card:hover {
  background: rgba(43, 44, 50, 0.8);
}

.vault-switcher__card-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.vault-switcher__select {
  flex: 1;
  min-width: 0;
  --nyx-border-size-select: 0;
}


.vault-switcher__count {
  font-size: 0.6875rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--nyx-c-text-3);
}

.vault-switcher__team-link {
  display: block;
  margin-top: 0.5rem;
  font-size: 0.75rem;
  color: var(--nyx-c-primary);
  text-decoration: none;
  padding-left: 0.125rem;
}

.vault-switcher__team-link:hover {
  text-decoration: underline;
}
</style>

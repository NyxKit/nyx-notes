<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NyxSelect } from 'nyx-kit/components'
import { NyxSize } from 'nyx-kit/types'
import type { NyxSelectOptionGroup } from 'nyx-kit/types'
import { useVaults } from '@/composables/useVaults'
import { useTeams } from '@/composables/useTeams'
import { useNotes } from '@/composables/useNotes'
import type { Vault } from '@/types'

const router = useRouter()
const { vaults, activeVault, setActive } = useVaults()
const { load: loadTeams, teamName } = useTeams()
const { notes } = useNotes()

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
  const n = notes.value.length
  return `${n} ${n === 1 ? 'note' : 'notes'}`
})

function select(vault: Vault) {
  setActive(vault)
  router.push(`/vaults/${vault.id}/notes`)
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
        <RouterLink
          v-if="activeVault"
          :to="`/vaults/${activeVault.id}/settings`"
          class="vault-switcher__settings"
          title="Vault settings"
        >
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
            <circle cx="7" cy="7" r="2.5" stroke="currentColor" stroke-width="1.25"/>
            <path d="M7 1v1.5M7 11.5V13M1 7h1.5M11.5 7H13M2.93 2.93l1.06 1.06M10.01 10.01l1.06 1.06M2.93 11.07l1.06-1.06M10.01 3.99l1.06-1.06" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
          </svg>
        </RouterLink>
      </div>
      <span class="vault-switcher__count">{{ noteCountLabel }}</span>
    </div>

    <RouterLink
      v-if="activeVault?.owner.type === 'team'"
      :to="`/teams/${activeVault.owner.id}/settings`"
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

.vault-switcher__settings {
  color: var(--nyx-c-text-3);
  text-decoration: none;
  display: flex;
  align-items: center;
  flex-shrink: 0;
  transition: color 0.2s;
  line-height: 0;
}

.vault-switcher__settings:hover {
  color: var(--nyx-c-text-2);
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

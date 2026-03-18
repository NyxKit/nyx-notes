<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NyxSelect } from 'nyx-kit/components'
import { NyxSize } from 'nyx-kit/types'
import type { NyxSelectOptionGroup } from 'nyx-kit/types'
import { useVaults } from '@/composables/useVaults'
import { useTeams } from '@/composables/useTeams'
import type { Vault } from '@/types'

const router = useRouter()
const { vaults, activeVault, setActive } = useVaults()
const { load: loadTeams, teamName } = useTeams()

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

function select(vault: Vault) {
  setActive(vault)
  router.push(`/vaults/${vault.id}/notes`)
}
</script>

<template>
  <div class="vault-switcher">
    <div class="vault-switcher__row">
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
      >⚙</RouterLink>
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
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.vault-switcher__row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.vault-switcher__select {
  flex: 1;
  min-width: 0;
}

.vault-switcher__settings {
  color: var(--nyx-color-muted, #718096);
  text-decoration: none;
  font-size: 0.875rem;
  flex-shrink: 0;
  line-height: 1;
}

.vault-switcher__settings:hover {
  color: inherit;
}

.vault-switcher__team-link {
  font-size: 0.75rem;
  color: var(--nyx-color-accent, #6366f1);
  text-decoration: none;
  padding-left: 0.125rem;
}

.vault-switcher__team-link:hover {
  text-decoration: underline;
}
</style>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
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

function select(vault: Vault) {
  setActive(vault)
  router.push(`/vaults/${vault.id}/notes`)
}
</script>

<template>
  <div class="vault-switcher">
    <div class="vault-switcher__row">
      <select
        :value="activeVault?.id ?? ''"
        class="vault-switcher__select"
        @change="e => {
          const vault = vaults.find(v => v.id === (e.target as HTMLSelectElement).value)
          if (vault) select(vault)
        }"
      >
        <optgroup v-if="personalVaults.length" label="Personal">
          <option v-for="v in personalVaults" :key="v.id" :value="v.id">
            {{ v.name }}
          </option>
        </optgroup>
        <optgroup
          v-for="[tid, tvaults] in teamGroups"
          :key="tid"
          :label="teamName(tid)"
        >
          <option v-for="v in tvaults" :key="v.id" :value="v.id">
            {{ v.name }}
          </option>
        </optgroup>
      </select>

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
  font-size: 0.875rem;
  font-weight: 600;
  background: transparent;
  border: none;
  cursor: pointer;
  outline: none;
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

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
    <select
      :value="activeVault?.id ?? ''"
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
  </div>
</template>

<style scoped>
.vault-switcher {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.vault-switcher select {
  width: 100%;
  font-size: 0.875rem;
  font-weight: 600;
  background: transparent;
  border: none;
  cursor: pointer;
  outline: none;
}
</style>

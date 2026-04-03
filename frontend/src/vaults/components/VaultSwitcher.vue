<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxSelect } from 'nyx-kit/components'
import { NyxSize } from 'nyx-kit/types'
import type { NyxSelectOptionGroup } from 'nyx-kit/types'
import { noteRoute, vaultRoute } from '@/shared/utils'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import type { Vault } from '@/shared/types'

const props = withDefaults(defineProps<{ dest?: 'notes' | 'vault' }>(), { dest: 'vault' })

const route = useRoute()
const router = useRouter()
const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { setActive } = vaultStore
const { notesFor } = useNotesStore()

const personalVaults = computed(() =>
  vaults.value.filter(v => v.owner.type === 'home')
)

const serverVaults = computed(() => vaults.value.filter(v => v.owner.type === 'server'))

const vaultSelectOptions = computed((): NyxSelectOptionGroup[] => {
  const groups: NyxSelectOptionGroup[] = []
  if (personalVaults.value.length) {
    groups.push({
      label: 'Personal',
      options: personalVaults.value.map(v => ({ label: v.name, value: v.slug }))
    })
  }
  if (serverVaults.value.length) {
    groups.push({
      label: 'Shared Server',
      options: serverVaults.value.map(v => ({ label: v.name, value: v.slug }))
    })
  }
  return groups
})

const selectedVaultId = computed({
  get: () => String(route.params.vault_id ?? ''),
  set: (slug: string) => {
    const vault = vaults.value.find(v => v.slug === slug)
    if (vault) select(vault)
  }
})

const noteCountLabel = computed(() => {
  const n = activeVault.value ? notesFor(activeVault.value.slug).length : 0
  return `${n} ${n === 1 ? 'note' : 'notes'}`
})

function select(vault: Vault) {
  setActive(vault)
  const target = props.dest === 'vault'
    ? vaultRoute(vault)
    : noteRoute(vault, '')
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

</style>

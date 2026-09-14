<script setup lang="ts">
import { computed } from 'vue'
import { NyxGrid } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import { useWorkspaceProfiles } from '@/shared/composables'
import { VaultCard } from '@/vaults/components'
import { useVaultStore } from '@/vaults/stores'

const props = withDefaults(defineProps<{ scope?: 'personal' | 'server' }>(), {
  scope: 'personal',
})

const { activeProfile } = useWorkspaceProfiles()
const vaultStore = useVaultStore()

const visibleVaults = computed(() => vaultStore.vaults)

const gridTitle = computed(() =>
  props.scope === 'personal' ? 'Personal' : (activeProfile.value?.display_name ?? 'Main Server')
)

</script>

<template>
  <div class="home-view">
    <main class="home-view__body">
      <div class="home-view__canvas home-view__canvas--overview">
        <NyxGrid :title="gridTitle" :mode="NyxGridMode.Grid" :columns="5">
          <VaultCard
            v-for="vault in visibleVaults"
            :key="vault.id"
            :model-value="vault"
          />
        </NyxGrid>
      </div>
    </main>

    <footer class="home-view__footer">
      <span class="home-view__footer-text">Nyx Notes — Silent Atelier</span>
    </footer>
  </div>
</template>

<style scoped>
.home-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.home-view__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.home-view__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.home-view__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.home-view__canvas--overview {
  display: flex;
  flex-direction: column;
}

.home-view__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.home-view__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

</style>

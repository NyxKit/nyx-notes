<script setup lang="ts">
import { computed, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useRoute, useRouter } from 'vue-router'
import { NyxButton, NyxIcon } from 'nyx-kit/components'
import { noteRoute } from '@/shared/utils'
import { NotesGrid } from '@/notes/components'
import { useVaultStore } from '@/vaults/stores'

const route = useRoute()
const router = useRouter()
const vaultId = computed(() => route.params.vault_id as string)

const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { setActive } = vaultStore

const vault = computed(() => vaults.value.find(v => v.slug === vaultId.value) ?? null)

watch(vault, currentVault => setActive(currentVault), { immediate: true })

async function createFirst() {
  if (activeVault.value) router.push(noteRoute(activeVault.value))
}
</script>

<template>
  <div class="vault-view">
    <Teleport to="#layout-header-actions" defer>
      <NyxButton
        v-if="activeVault"
        :gradient="true"
        @click="createFirst"
      >
        New Note
      </NyxButton>
    </Teleport>

    <main class="vault-view__body">
      <NotesGrid
        class="vault-view__canvas vault-view__canvas--overview"
        title="Notes"
        :vault="activeVault ?? undefined"
        :vault-id="vaultId"
      >
        <template #loading>
          <div class="vault-view__canvas--center">
            <div class="vault-view__skeleton-grid">
              <div v-for="n in 6" :key="n" class="vault-view__skeleton-card" />
            </div>
          </div>
        </template>

        <template #empty>
          <div class="vault-view__canvas--center">
            <div class="vault-view__welcome-card">
              <div class="vault-view__welcome-icon">
                <NyxIcon name="file-text" :size="32" />
              </div>
              <h1 class="vault-view__heading">This vault is empty.</h1>
              <p class="vault-view__desc">Start writing your first note. It will appear here once saved.</p>
              <NyxButton :gradient="true" @click="createFirst">New Note</NyxButton>
            </div>
          </div>
        </template>
      </NotesGrid>
    </main>

    <footer class="vault-view__footer">
      <span class="vault-view__footer-text">{{ activeVault?.name ?? 'Vault' }}</span>
    </footer>
  </div>
</template>

<style scoped>
.vault-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.vault-view__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.vault-view__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.vault-view__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.vault-view__canvas--overview {
  display: flex;
  flex-direction: column;
}

.vault-view__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.vault-view__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

.vault-view__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.vault-view__skeleton-card {
  height: 120px;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: vault-view-pulse 1.4s ease-in-out infinite;
}

@keyframes vault-view-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}

.vault-view__welcome-card {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 2.5rem 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: flex-start;
  max-width: 400px;
  width: 100%;
}

.vault-view__welcome-icon {
  color: var(--nyx-c-primary);
  opacity: 0.7;
  line-height: 0;
}

.vault-view__heading {
  font-family: 'Manrope', sans-serif;
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1.25;
  color: var(--nyx-c-text-1);
  margin: 0;
}

.vault-view__desc {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--nyx-c-text-2);
  margin: 0;
}
</style>

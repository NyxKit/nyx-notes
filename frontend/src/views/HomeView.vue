<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'
import { NyxButton, NyxInput, NyxForm, NyxFormField } from 'nyx-kit/components'
import { NyxVariant } from 'nyx-kit/types'
import VaultIcon from '@/components/VaultIcon.vue'
import VaultIconPicker from '@/components/VaultIconPicker.vue'

const router = useRouter()
const vaultStore = useVaultStore()
const { vaults, loading } = storeToRefs(vaultStore)
const { load: loadVaults, create: createVault, setActive } = vaultStore

const showCreateForm = ref(false)
const newSlug = ref('')
const newName = ref('')
const newIcon = ref<string | undefined>(undefined)
const creating = ref(false)

onMounted(async () => {
  setActive(null)
  await loadVaults()
  // Only auto-redirect on initial page load (no back history entry means this is the entry point)
  const isInitialLoad = !window.history.state?.back
  if (isInitialLoad && vaults.value.length === 1) {
    router.replace(`/vaults/${vaults.value[0].id}`)
  }
})

function openVault(vaultId: string) {
  router.push(`/vaults/${vaultId}`)
}

async function submitCreate() {
  if (!newSlug.value.trim() || !newName.value.trim()) return
  creating.value = true
  try {
    const vault = await createVault({ slug: newSlug.value.trim(), name: newName.value.trim(), icon: newIcon.value })
    router.push(`/vaults/${vault.id}`)
  } finally {
    creating.value = false
  }
}

function cancelCreate() {
  showCreateForm.value = false
  newSlug.value = ''
  newName.value = ''
  newIcon.value = undefined
}
</script>

<template>
  <div class="app-shell__main">

      <!-- Header -->
      <header class="app-shell__header">
        <div class="app-shell__header-left">
          <span class="app-shell__title">Vaults</span>
        </div>
        <div class="app-shell__header-right">
          <NyxButton :gradient="true" @click="showCreateForm = true">New Vault</NyxButton>
        </div>
      </header>

      <!-- Body -->
      <main class="app-shell__body">

        <!-- Loading -->
        <div v-if="loading" class="app-shell__canvas app-shell__canvas--center">
          <div class="home__skeleton-grid">
            <div v-for="n in 4" :key="n" class="home__skeleton-card" />
          </div>
        </div>

        <!-- Vault masonry -->
        <div v-else class="app-shell__canvas app-shell__canvas--masonry">
          <div class="home__masonry-header">
            <h2 class="home__masonry-title">Your Vaults</h2>
          </div>
          <div class="home__masonry">

            <NyxButton
              v-for="vault in vaults"
              :key="vault.id"
              class="home__vault-card"
              @click="openVault(vault.id)"
            >
              <div class="home__vault-text">
                <span class="home__vault-name">{{ vault.name }}</span>
                <span class="home__vault-slug">{{ vault.slug }}</span>
              </div>
              <VaultIcon
                :slug="vault.icon || 'folder'"
                class="home__vault-bg-icon"
                aria-hidden="true"
              />
            </NyxButton>

            <!-- Inline create form card -->
            <NyxForm v-if="showCreateForm" class="home__vault-card home__vault-card--form" @submit="submitCreate">
              <NyxFormField label="Vault name">
                <template #default="{ id }">
                  <NyxInput :id="id" v-model="newName" placeholder="Vault name" autofocus />
                </template>
              </NyxFormField>
              <NyxFormField label="Slug">
                <template #default="{ id }">
                  <NyxInput :id="id" v-model="newSlug" placeholder="slug (e.g. work)" />
                </template>
              </NyxFormField>
              <NyxFormField label="Icon">
                <template #default>
                  <VaultIconPicker v-model="newIcon" />
                </template>
              </NyxFormField>
              <div class="home__form-actions">
                <NyxButton :gradient="true" type="submit" :disabled="creating">
                  {{ creating ? 'Creating…' : 'Create' }}
                </NyxButton>
                <NyxButton @click="cancelCreate">Cancel</NyxButton>
              </div>
            </NyxForm>

          </div>
        </div>

      </main>

      <!-- Footer -->
      <footer class="app-shell__footer">
        <span class="app-shell__footer-text">Nyx Notes — Silent Atelier</span>
      </footer>

  </div>
</template>

<style scoped>
.app-shell__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

/* ── Header ─────────────────────────────────────────────────── */
.app-shell__header {
  height: 64px;
  flex-shrink: 0;
  background: var(--nyx-c-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 1px 0 0 var(--nyx-c-divider);
}

.app-shell__header-left,
.app-shell__header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.app-shell__title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
}

/* ── Body ────────────────────────────────────────────────────── */
.app-shell__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.app-shell__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.app-shell__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-shell__canvas--masonry {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  align-items: flex-start;
}

/* ── Footer ─────────────────────────────────────────────────── */
.app-shell__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.app-shell__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

/* ── Masonry ─────────────────────────────────────────────────── */
.home__masonry-header {
  display: flex;
  align-items: center;
  width: 100%;
}

.home__masonry-title {
  font-family: 'Manrope', sans-serif;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--nyx-c-text-3);
  margin: 0;
}

.home__masonry {
  columns: 5 160px;
  column-gap: 1rem;
  width: 100%;
}

/* ── Vault card ─────────────────────────────────────────────── */
.home__vault-card {
  position: relative;
  overflow: hidden;
  aspect-ratio: 1 / 1;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  align-items: flex-start;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 1.25rem;
  margin-bottom: 1rem;
  break-inside: avoid;
  cursor: pointer;
  text-align: left;
  border: 1px solid transparent;
  width: 100%;
  transition: border-color 0.15s, background 0.15s;
}

.home__vault-card:hover {
  border-color: var(--nyx-c-divider);
  background: var(--nyx-c-bg-mute);
}

.home__vault-card--form {
  aspect-ratio: unset;
  justify-content: flex-start;
  cursor: default;
  gap: 0.75rem;
}

.home__vault-card--form:hover {
  background: var(--nyx-c-bg-soft);
}

.home__vault-text {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  position: relative;
  z-index: 1;
}

.home__vault-name {
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--nyx-c-text-1);
  line-height: 1.4;
}

.home__vault-slug {
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
  font-family: 'Inter', monospace;
}

.home__vault-bg-icon {
  position: absolute;
  right: -8%;
  top: 50%;
  transform: translateY(-50%);
  width: 70%;
  height: auto;
  opacity: 0.12;
  pointer-events: none;
  color: var(--nyx-c-text-1);
}

/* ── Inline create form ─────────────────────────────────────── */
.home__form-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-top: 0.25rem;
}

/* ── Skeleton ────────────────────────────────────────────────── */
.home__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.home__skeleton-card {
  aspect-ratio: 1 / 1;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: home-pulse 1.4s ease-in-out infinite;
}

@keyframes home-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}
</style>

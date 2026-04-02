<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { NyxButton, NyxCard, NyxForm, NyxFormField, NyxGrid, NyxInput, NyxTextarea } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import { RouteName } from '@/shared/types'
import { VaultCard, VaultIconPicker } from '@/vaults/components'
import { useVaultStore } from '@/vaults/stores'

const router = useRouter()
const vaultStore = useVaultStore()
const { vaults, loading } = storeToRefs(vaultStore)
const { load: loadVaults, create: createVault, setActive } = vaultStore

const showCreateForm = ref(false)
const newSlug = ref('')
const newName = ref('')
const newDescription = ref('')
const newIcon = ref<string | undefined>(undefined)
const creating = ref(false)

onMounted(async () => {
  setActive(null)
  await loadVaults()
  const isInitialLoad = !window.history.state?.back
  if (isInitialLoad && vaults.value.length === 1) {
    router.replace({ name: RouteName.Vault, params: { vault_id: vaults.value[0].id } })
  }
})

async function submitCreate() {
  if (!newSlug.value.trim() || !newName.value.trim()) return

  creating.value = true
  try {
    const vault = await createVault({
      slug: newSlug.value.trim(),
      name: newName.value.trim(),
      description: newDescription.value.trim() || undefined,
      icon: newIcon.value,
    })
    router.push({ name: RouteName.Vault, params: { vault_id: vault.id } })
  } finally {
    creating.value = false
  }
}

function cancelCreate() {
  showCreateForm.value = false
  newSlug.value = ''
  newName.value = ''
  newDescription.value = ''
  newIcon.value = undefined
}
</script>

<template>
  <div class="home-view">
    <Teleport to="#layout-header-actions" defer>
      <NyxButton :gradient="true" @click="showCreateForm = true">New Vault</NyxButton>
    </Teleport>

    <main class="home-view__body">
      <div v-if="loading" class="home-view__canvas home-view__canvas--center">
        <div class="home-view__skeleton-grid">
          <div v-for="n in 4" :key="n" class="home-view__skeleton-card" />
        </div>
      </div>

      <div v-else class="home-view__canvas home-view__canvas--overview">
        <NyxGrid title="Your Vaults" :mode="NyxGridMode.Grid" :columns="5">
          <VaultCard
            v-for="vault in vaults"
            :key="vault.id"
            :model-value="vault"
          />

          <NyxCard v-if="showCreateForm" class="home-view__create-card">
            <div class="home-view__create-copy">
              <h3 class="home-view__create-title">New vault</h3>
              <p class="home-view__create-supporting">Choose a name, slug, description, and icon.</p>
            </div>

            <NyxForm class="home-view__create-form" @submit="submitCreate">
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

              <NyxFormField label="Description">
                <template #default="{ id }">
                  <NyxTextarea :id="id" v-model="newDescription" placeholder="Optional description" />
                </template>
              </NyxFormField>

              <NyxFormField label="Icon">
                <template #default>
                  <VaultIconPicker v-model="newIcon" />
                </template>
              </NyxFormField>

              <div class="home-view__form-actions">
                <NyxButton :gradient="true" type="submit" :disabled="creating">
                  {{ creating ? 'Creating…' : 'Create' }}
                </NyxButton>
                <NyxButton type="button" @click="cancelCreate">Cancel</NyxButton>
              </div>
            </NyxForm>
          </NyxCard>
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

.home-view__create-copy {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  margin-bottom: 1rem;
}

.home-view__create-title {
  margin: 0;
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--nyx-browse-card-text);
}

.home-view__create-supporting {
  margin: 0;
  font-family: 'Inter', sans-serif;
  font-size: 0.75rem;
  line-height: 1.5;
  color: var(--nyx-c-text-2);
}

.home-view__create-form {
  display: flex;
  flex-direction: column;
  gap: 0.875rem;
}

.home-view__form-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
  margin-top: 0.25rem;
}

.home-view__create-card :deep(.nyx-card__body) {
  display: flex;
  flex-direction: column;
  padding: 1.25rem;
}

.home-view__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.home-view__skeleton-card {
  aspect-ratio: 1 / 1;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: home-view-pulse 1.4s ease-in-out infinite;
}

@keyframes home-view-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}
</style>

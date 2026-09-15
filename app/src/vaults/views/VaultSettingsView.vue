<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useAuth } from '@/auth/composables'
import { NyxButton, NyxInput, NyxSelect, NyxTextarea } from 'nyx-kit/components'
import { NyxTheme } from 'nyx-kit/types'
import type { NyxSelectOption } from 'nyx-kit/types'
import { NyxKit } from 'nyx-kit'
import { VaultIconPicker } from '@/vaults/components'
import { useVaultStore } from '@/vaults/stores'

const route = useRoute()
const router = useRouter()
const auth = useAuth()
const vaultStore = useVaultStore()
const { vaults } = storeToRefs(vaultStore)
const { load, remove, update, removeServerVault } = vaultStore

const vaultId = computed(() => route.params.vault_id as string)
const vault = computed(() => vaults.value.find(v => v.slug === vaultId.value) ?? null)

const deleteError = ref<string | null>(null)
const iconError = ref<string | null>(null)
const detailsError = ref<string | null>(null)
const detailsSaved = ref(false)
const localName = ref('')
const localDescription = ref('')

// Load vaults if not yet populated
watch(vaultId, async () => {
  if (!vaults.value.length) await load()
}, { immediate: true })

watch(vault, (currentVault) => {
  localName.value = currentVault?.name ?? ''
  localDescription.value = currentVault?.description ?? ''
  detailsSaved.value = false
}, { immediate: true })

async function onDetailsSave() {
  if (!vault.value) return
  detailsError.value = null
  detailsSaved.value = false

  try {
    await update(vaultId.value, {
      name: localName.value.trim() || vault.value.name,
      description: localDescription.value.trim() || null,
    })
    detailsSaved.value = true
  } catch (e) {
    detailsError.value = String(e)
  }
}

async function onIconChange(slug: string | undefined) {
  if (!vault.value) return
  iconError.value = null
  try {
    await update(vaultId.value, { icon: slug ?? null })
  } catch (e) {
    iconError.value = String(e)
  }
}

async function onDelete() {
  if (!vault.value) return
  deleteError.value = null
  const result = await NyxKit.confirm({
    title: 'Delete Vault',
    message: `Are you sure you want to delete "${vault.value.name}"? This cannot be undone.`,
    confirmText: 'Yes, Delete Vault',
    cancelText: 'Cancel',
    theme: NyxTheme.Danger,
  })
  if (result.isFailure) return
  try {
    if (vault.value?.owner.type === 'server') {
      await removeServerVault(vaultId.value)
    } else {
      await remove(vaultId.value)
    }
    router.push(auth.personalOverviewRoute.value)
  } catch (e) {
    deleteError.value = String(e)
  }
}

const ownerLabel = computed(() => {
  if (!vault.value) return ''
  if (vault.value.owner.type === 'home') return 'Personal'
  if (vault.value.owner.type === 'server') return `Server: ${vault.value.owner.server_slug}`
  return 'Built-in'
})

const permissionOptions: NyxSelectOption[] = [
  { label: 'Restricted', value: 'restricted' },
  { label: 'Comment', value: 'comment' },
  { label: 'Edit', value: 'edit' },
]
</script>

<template>
  <div class="settings-page">
    <div class="settings-page__inner">
      <header class="settings-page__header">
        <NyxButton @click="router.back()">← Back</NyxButton>
        <h1 class="settings-page__title">Vault Settings</h1>
      </header>

      <div v-if="!vault" class="settings-page__loading">Loading…</div>

      <template v-else>
        <!-- Info -->
        <section class="settings-section">
          <h2 class="settings-section__heading">Details</h2>
          <div class="settings-field">
            <label class="settings-field__label" for="vault-name">Name</label>
            <NyxInput id="vault-name" v-model="localName" placeholder="Vault name" />
          </div>
          <div class="settings-field">
            <label class="settings-field__label" for="vault-description">Description</label>
            <NyxTextarea
              id="vault-description"
              v-model="localDescription"
              placeholder="Optional description"
            />
          </div>
          <div class="settings-row">
            <span class="settings-row__label">Slug</span>
            <span class="settings-row__value settings-row__value--mono">{{ vault.slug }}</span>
          </div>
            <div class="settings-row">
              <span class="settings-row__label">Owner</span>
              <span class="settings-row__value">{{ ownerLabel }}</span>
            </div>
          <div v-if="vault.owner.type === 'server'" class="settings-row">
            <span class="settings-row__label">Role</span>
            <span class="settings-row__value">Shared server vault</span>
          </div>
          <div class="settings-actions">
            <NyxButton :gradient="true" @click="onDetailsSave">Save details</NyxButton>
            <span v-if="detailsSaved" class="settings-success">Saved</span>
          </div>
          <p v-if="detailsError" class="settings-error">{{ detailsError }}</p>
        </section>

        <!-- Icon -->
        <section class="settings-section">
          <h2 class="settings-section__heading">Icon</h2>
          <p class="settings-section__description">
            Choose a decorative icon for this vault. Click the active icon to remove it.
          </p>
          <VaultIconPicker
            :model-value="vault.icon"
            @update:model-value="onIconChange"
          />
          <p v-if="iconError" class="settings-error">{{ iconError }}</p>
        </section>

        <!-- Permission (shared server vaults are currently read-only in the frontend) -->
        <section v-if="vault.owner.type === 'server'" class="settings-section">
          <h2 class="settings-section__heading">Default Permission</h2>
          <p class="settings-section__description">
            Shared server vaults currently inherit their default note permission from the backend.
            Permission editing is not yet exposed in the MVP frontend.
          </p>
          <NyxSelect
            :model-value="vault.permission"
            :options="permissionOptions"
            disabled
          />
        </section>

        <!-- Danger zone -->
        <section class="settings-section settings-section--danger">
          <h2 class="settings-section__heading settings-section__heading--danger">Danger Zone</h2>
          <p class="settings-section__description">
            Deleting a vault is permanent. The vault must be empty (no notes) before it can be deleted.
          </p>
          <NyxButton :theme="NyxTheme.Danger" @click="onDelete">
            Delete vault
          </NyxButton>
          <p v-if="deleteError" class="settings-error">{{ deleteError }}</p>
        </section>
      </template>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  padding: 2rem 1rem;
}

.settings-page__inner {
  max-width: 600px;
  margin: 0 auto;
}

.settings-page__header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 2rem;
}

.settings-page__title {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
}

.settings-page__loading {
  color: var(--nyx-color-muted, #718096);
  text-align: center;
  padding: 3rem;
}

.settings-section {
  margin-bottom: 2rem;
  padding: 1.25rem;
  border: 1px solid var(--nyx-c-divider, #e2e8f0);
  border-radius: 0.5rem;
}

.settings-section--danger {
  border-color: #fed7d7;
}

.settings-section__heading {
  font-size: 0.9375rem;
  font-weight: 600;
  margin: 0 0 1rem;
}

.settings-section__heading--danger {
  color: #c53030;
}

.settings-section__description {
  font-size: 0.875rem;
  color: var(--nyx-color-muted, #718096);
  margin: 0 0 1rem;
  line-height: 1.5;
}

.settings-row {
  display: flex;
  align-items: baseline;
  gap: 1rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--nyx-c-divider, #e2e8f0);
  font-size: 0.875rem;
}

.settings-row:last-child {
  border-bottom: none;
}

.settings-row__label {
  width: 80px;
  flex-shrink: 0;
  color: var(--nyx-color-muted, #718096);
}

.settings-row__value--mono {
  font-family: monospace;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.settings-field__label {
  font-size: 0.875rem;
  color: var(--nyx-color-muted, #718096);
}

.settings-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1rem;
}

.settings-success {
  font-size: 0.8125rem;
  color: var(--nyx-c-primary);
}

.settings-error {
  font-size: 0.8125rem;
  color: #c53030;
  margin: 0.5rem 0 0;
}
</style>

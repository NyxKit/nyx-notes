<script setup lang="ts">
import { computed, watch, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NyxSelect } from 'nyx-kit/components'
import type { NyxSelectOption } from 'nyx-kit/types'
import { useVaults } from '@/composables/useVaults'
import type { NotePermission } from '@/types'

const route = useRoute()
const router = useRouter()
const { vaults, load, remove, patchPermission } = useVaults()

const vaultId = computed(() => route.params.vault_id as string)
const vault = computed(() => vaults.value.find(v => v.id === vaultId.value) ?? null)

const permissionError = ref<string | null>(null)
const deleteError = ref<string | null>(null)
const confirmDelete = ref(false)

// Load vaults if not yet populated
watch(vaultId, async () => {
  if (!vaults.value.length) await load()
}, { immediate: true })

async function onPermissionChange(permission: NotePermission) {
  if (!vault.value || vault.value.owner.type !== 'team') return
  permissionError.value = null
  try {
    await patchPermission(vault.value.owner.id, vaultId.value, permission)
  } catch (e) {
    permissionError.value = String(e)
  }
}

async function onDelete() {
  deleteError.value = null
  try {
    await remove(vaultId.value)
    router.push('/')
  } catch (e) {
    deleteError.value = String(e)
    confirmDelete.value = false
  }
}

const permissionOptions: NyxSelectOption[] = [
  { label: 'Restricted — only team members with explicit access', value: 'restricted' },
  { label: 'Comment — all team members can comment', value: 'comment' },
  { label: 'Edit — all team members can edit', value: 'edit' },
]

const permissionModel = computed({
  get: () => vault.value?.permission ?? 'restricted',
  set: (v: string) => onPermissionChange(v as NotePermission)
})
</script>

<template>
  <div class="settings-page">
    <div class="settings-page__inner">
      <header class="settings-page__header">
        <button class="settings-page__back" @click="router.back()">← Back</button>
        <h1 class="settings-page__title">Vault Settings</h1>
      </header>

      <div v-if="!vault" class="settings-page__loading">Loading…</div>

      <template v-else>
        <!-- Info -->
        <section class="settings-section">
          <h2 class="settings-section__heading">Details</h2>
          <div class="settings-row">
            <span class="settings-row__label">Name</span>
            <span class="settings-row__value">{{ vault.name }}</span>
          </div>
          <div class="settings-row">
            <span class="settings-row__label">Slug</span>
            <span class="settings-row__value settings-row__value--mono">{{ vault.slug }}</span>
          </div>
          <div class="settings-row">
            <span class="settings-row__label">Owner</span>
            <span class="settings-row__value">
              {{ vault.owner.type === 'user' ? 'Personal' : `Team: ${vault.owner.id}` }}
            </span>
          </div>
        </section>

        <!-- Permission (team vaults only) -->
        <section v-if="vault.owner.type === 'team'" class="settings-section">
          <h2 class="settings-section__heading">Default Permission</h2>
          <p class="settings-section__description">
            Controls what team members can do with notes in this vault by default.
            Note authors can override this per note.
          </p>
          <NyxSelect
            v-model="permissionModel"
            :options="permissionOptions"
          />
          <p v-if="permissionError" class="settings-error">{{ permissionError }}</p>
        </section>

        <!-- Danger zone -->
        <section class="settings-section settings-section--danger">
          <h2 class="settings-section__heading settings-section__heading--danger">Danger Zone</h2>
          <p class="settings-section__description">
            Deleting a vault is permanent. The vault must be empty (no notes) before it can be deleted.
          </p>
          <div v-if="!confirmDelete">
            <button class="settings-btn settings-btn--danger" @click="confirmDelete = true">
              Delete vault
            </button>
          </div>
          <div v-else class="settings-confirm">
            <p class="settings-confirm__warning">
              Are you sure you want to delete <strong>{{ vault.name }}</strong>? This cannot be undone.
            </p>
            <div class="settings-confirm__actions">
              <button class="settings-btn" @click="confirmDelete = false">Cancel</button>
              <button class="settings-btn settings-btn--danger" @click="onDelete">
                Yes, delete vault
              </button>
            </div>
            <p v-if="deleteError" class="settings-error">{{ deleteError }}</p>
          </div>
        </section>
      </template>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  min-height: 100vh;
  padding: 2rem 1rem;
  overflow-y: auto;
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

.settings-page__back {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--nyx-color-accent, #6366f1);
  font-size: 0.875rem;
  font-family: inherit;
  padding: 0;
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
  border: 1px solid var(--nyx-color-border, #e2e8f0);
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
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
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


.settings-btn {
  font-size: 0.875rem;
  padding: 0.4375rem 1rem;
  border-radius: 0.375rem;
  cursor: pointer;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  background: transparent;
  color: inherit;
  font-family: inherit;
}

.settings-btn--danger {
  background: #c53030;
  color: #fff;
  border-color: #c53030;
}

.settings-confirm {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.settings-confirm__warning {
  font-size: 0.875rem;
  margin: 0;
}

.settings-confirm__actions {
  display: flex;
  gap: 0.5rem;
}

.settings-error {
  font-size: 0.8125rem;
  color: #c53030;
  margin: 0.5rem 0 0;
}
</style>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NyxSelect } from 'nyx-kit/components'
import { NyxSize } from 'nyx-kit/types'
import type { NyxSelectOption } from 'nyx-kit/types'
import { useAuth } from '@/composables/useAuth'
import { useTeams } from '@/composables/useTeams'
import { useVaults } from '@/composables/useVaults'
import type { TeamRole, NotePermission } from '@/types'

const route = useRoute()
const router = useRouter()
const { currentUser, authMode } = useAuth()
const { teams, loadOne, remove: removeTeam, addTeamMember, updateMember, kickMember } = useTeams()
const { vaults, load: loadVaults, patchPermission, addTeamVault, removeTeamVault } = useVaults()

const teamId = computed(() => route.params.team_id as string)
const team = computed(() => teams.value.find(t => t.id === teamId.value) ?? null)

const teamVaults = computed(() =>
  vaults.value.filter(v => v.owner.type === 'team' && v.owner.id === teamId.value)
)

const currentUserId = computed(() =>
  authMode.value === 'local' ? (team.value?.members[0]?.user_id ?? '') : (currentUser.value?.id ?? '')
)

const isOwner = computed(() =>
  team.value?.members.some(m => m.user_id === currentUserId.value && m.role === 'owner') ?? false
)

// Load team + vaults
watch(teamId, async () => {
  await Promise.all([loadOne(teamId.value), loadVaults()])
}, { immediate: true })

// — Add member form —
const newMemberId = ref('')
const newMemberRole = ref<Exclude<TeamRole, 'owner'>>('member')
const addMemberError = ref<string | null>(null)
const addMemberLoading = ref(false)

async function onAddMember() {
  if (!newMemberId.value.trim()) return
  addMemberError.value = null
  addMemberLoading.value = true
  try {
    await addTeamMember(teamId.value, newMemberId.value.trim(), newMemberRole.value)
    newMemberId.value = ''
  } catch (e) {
    addMemberError.value = String(e)
  } finally {
    addMemberLoading.value = false
  }
}

async function onRoleChange(userId: string, role: string) {
  await updateMember(teamId.value, userId, role as Exclude<TeamRole, 'owner'>)
}

async function onKickMember(userId: string) {
  await kickMember(teamId.value, userId)
}

// — Add vault form —
const showAddVault = ref(false)
const newVaultName = ref('')
const newVaultSlug = ref('')
const addVaultError = ref<string | null>(null)
const addVaultLoading = ref(false)

watch(newVaultName, (name) => {
  newVaultSlug.value = name.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')
})

async function onAddVault() {
  if (!newVaultName.value.trim() || !newVaultSlug.value.trim()) return
  addVaultError.value = null
  addVaultLoading.value = true
  try {
    await addTeamVault(teamId.value, { name: newVaultName.value.trim(), slug: newVaultSlug.value.trim() })
    newVaultName.value = ''
    newVaultSlug.value = ''
    showAddVault.value = false
  } catch (e) {
    addVaultError.value = String(e)
  } finally {
    addVaultLoading.value = false
  }
}

async function onVaultPermissionChange(vaultId: string, permission: NotePermission) {
  await patchPermission(teamId.value, vaultId, permission)
}

async function onDeleteVault(vaultId: string) {
  await removeTeamVault(teamId.value, vaultId)
}

// — Delete team —
const confirmDeleteTeam = ref(false)
const deleteTeamError = ref<string | null>(null)

async function onDeleteTeam() {
  deleteTeamError.value = null
  try {
    await removeTeam(teamId.value)
    router.push('/')
  } catch (e) {
    deleteTeamError.value = String(e)
    confirmDeleteTeam.value = false
  }
}

const roleOptions: NyxSelectOption[] = [
  { label: 'Admin', value: 'admin' },
  { label: 'Member', value: 'member' },
]

const permissionOptions: NyxSelectOption[] = [
  { label: 'Restricted', value: 'restricted' },
  { label: 'Comment', value: 'comment' },
  { label: 'Edit', value: 'edit' },
]

const newMemberRoleModel = computed({
  get: () => newMemberRole.value as string,
  set: (v: string) => { newMemberRole.value = v as Exclude<TeamRole, 'owner'> }
})
</script>

<template>
  <div class="settings-page">
    <div class="settings-page__inner">
      <header class="settings-page__header">
        <button class="settings-page__back" @click="router.back()">← Back</button>
        <h1 class="settings-page__title">Team Settings</h1>
      </header>

      <div v-if="!team" class="settings-page__loading">Loading…</div>

      <template v-else>
        <!-- Info -->
        <section class="settings-section">
          <h2 class="settings-section__heading">Details</h2>
          <div class="settings-row">
            <span class="settings-row__label">Name</span>
            <span class="settings-row__value">{{ team.name }}</span>
          </div>
          <div class="settings-row">
            <span class="settings-row__label">ID</span>
            <span class="settings-row__value settings-row__value--mono">{{ team.id }}</span>
          </div>
        </section>

        <!-- Members -->
        <section class="settings-section">
          <h2 class="settings-section__heading">Members</h2>

          <table class="settings-table">
            <thead>
              <tr>
                <th>User ID</th>
                <th>Role</th>
                <th v-if="isOwner" />
              </tr>
            </thead>
            <tbody>
              <tr v-for="member in team.members" :key="member.user_id">
                <td class="settings-table__mono">{{ member.user_id }}</td>
                <td>
                  <NyxSelect
                    v-if="isOwner && member.role !== 'owner'"
                    :model-value="member.role"
                    :options="roleOptions"
                    :size="NyxSize.Small"
                    @update:model-value="onRoleChange(member.user_id, $event as string)"
                  />
                  <span v-else class="settings-badge">{{ member.role }}</span>
                </td>
                <td v-if="isOwner">
                  <button
                    v-if="member.role !== 'owner'"
                    class="settings-link settings-link--danger"
                    @click="onKickMember(member.user_id)"
                  >
                    Remove
                  </button>
                </td>
              </tr>
            </tbody>
          </table>

          <!-- Add member form -->
          <div v-if="isOwner" class="settings-add-form">
            <input
              v-model="newMemberId"
              class="settings-input"
              placeholder="User ID"
              @keydown.enter="onAddMember"
            />
            <NyxSelect
              v-model="newMemberRoleModel"
              :options="roleOptions"
              :size="NyxSize.Small"
            />
            <button
              class="settings-btn settings-btn--primary"
              :disabled="!newMemberId.trim() || addMemberLoading"
              @click="onAddMember"
            >
              {{ addMemberLoading ? 'Adding…' : 'Add member' }}
            </button>
            <p v-if="addMemberError" class="settings-error">{{ addMemberError }}</p>
          </div>
        </section>

        <!-- Vaults -->
        <section class="settings-section">
          <div class="settings-section__row-header">
            <h2 class="settings-section__heading">Vaults</h2>
            <button
              v-if="isOwner"
              class="settings-link"
              @click="showAddVault = !showAddVault"
            >
              + Add vault
            </button>
          </div>

          <!-- Add vault form -->
          <div v-if="showAddVault" class="settings-add-form settings-add-form--block">
            <div class="settings-field">
              <label class="settings-field__label">Name</label>
              <input v-model="newVaultName" class="settings-input" placeholder="e.g. Design" />
            </div>
            <div class="settings-field">
              <label class="settings-field__label">Slug</label>
              <input v-model="newVaultSlug" class="settings-input" placeholder="e.g. design" />
            </div>
            <div class="settings-add-form__actions">
              <button class="settings-btn" @click="showAddVault = false">Cancel</button>
              <button
                class="settings-btn settings-btn--primary"
                :disabled="!newVaultName.trim() || !newVaultSlug.trim() || addVaultLoading"
                @click="onAddVault"
              >
                {{ addVaultLoading ? 'Creating…' : 'Create vault' }}
              </button>
            </div>
            <p v-if="addVaultError" class="settings-error">{{ addVaultError }}</p>
          </div>

          <div v-if="teamVaults.length" class="settings-vault-list">
            <div
              v-for="vault in teamVaults"
              :key="vault.id"
              class="settings-vault-row"
            >
              <span class="settings-vault-row__name">{{ vault.name }}</span>
              <NyxSelect
                :model-value="vault.permission"
                :options="permissionOptions"
                :size="NyxSize.Small"
                @update:model-value="onVaultPermissionChange(vault.id, $event as NotePermission)"
              />
              <button
                v-if="isOwner"
                class="settings-link settings-link--danger"
                @click="onDeleteVault(vault.id)"
              >
                Delete
              </button>
            </div>
          </div>
          <p v-else-if="!showAddVault" class="settings-empty">No vaults yet.</p>
        </section>

        <!-- Danger zone (owner only) -->
        <section v-if="isOwner" class="settings-section settings-section--danger">
          <h2 class="settings-section__heading settings-section__heading--danger">Danger Zone</h2>
          <p class="settings-section__description">
            Deleting the team removes all its vaults and notes permanently.
          </p>
          <div v-if="!confirmDeleteTeam">
            <button class="settings-btn settings-btn--danger" @click="confirmDeleteTeam = true">
              Delete team
            </button>
          </div>
          <div v-else class="settings-confirm">
            <p class="settings-confirm__warning">
              Are you sure you want to delete <strong>{{ team.name }}</strong>? This cannot be undone.
            </p>
            <div class="settings-confirm__actions">
              <button class="settings-btn" @click="confirmDeleteTeam = false">Cancel</button>
              <button class="settings-btn settings-btn--danger" @click="onDeleteTeam">
                Yes, delete team
              </button>
            </div>
            <p v-if="deleteTeamError" class="settings-error">{{ deleteTeamError }}</p>
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
  max-width: 640px;
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
  margin-bottom: 1.5rem;
  padding: 1.25rem;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.5rem;
}

.settings-section--danger {
  border-color: #fed7d7;
}

.settings-section__row-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.settings-section__row-header .settings-section__heading {
  margin-bottom: 0;
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
  font-size: 0.8125rem;
}

.settings-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
  margin-bottom: 1rem;
}

.settings-table th {
  text-align: left;
  font-weight: 500;
  color: var(--nyx-color-muted, #718096);
  padding: 0.375rem 0.5rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.settings-table td {
  padding: 0.5rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.settings-table__mono {
  font-family: monospace;
  font-size: 0.8125rem;
}

.settings-badge {
  font-size: 0.75rem;
  padding: 0.125rem 0.5rem;
  border-radius: 9999px;
  background: var(--nyx-color-surface-raised, #f7fafc);
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  text-transform: capitalize;
}


.settings-add-form {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
  margin-top: 0.75rem;
}

.settings-add-form--block {
  flex-direction: column;
  align-items: stretch;
  padding: 1rem;
  background: var(--nyx-color-surface-raised, #f7fafc);
  border-radius: 0.375rem;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  margin-top: 0.75rem;
}

.settings-add-form__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.settings-field__label {
  font-size: 0.8125rem;
  color: var(--nyx-color-muted, #718096);
}

.settings-input {
  padding: 0.4375rem 0.625rem;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.375rem;
  font-family: inherit;
  font-size: 0.875rem;
  background: transparent;
  color: inherit;
  flex: 1;
  min-width: 150px;
}

.settings-input:focus {
  outline: none;
  border-color: var(--nyx-color-accent, #6366f1);
}

.settings-vault-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.settings-vault-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
  font-size: 0.875rem;
}

.settings-vault-row:last-child {
  border-bottom: none;
}

.settings-vault-row__name {
  flex: 1;
  font-weight: 500;
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
  white-space: nowrap;
}

.settings-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.settings-btn--primary {
  background: var(--nyx-color-accent, #6366f1);
  color: #fff;
  border-color: var(--nyx-color-accent, #6366f1);
}

.settings-btn--danger {
  background: #c53030;
  color: #fff;
  border-color: #c53030;
}

.settings-link {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.8125rem;
  color: var(--nyx-color-accent, #6366f1);
  font-family: inherit;
  padding: 0;
  white-space: nowrap;
}

.settings-link--danger {
  color: #c53030;
}

.settings-empty {
  font-size: 0.875rem;
  color: var(--nyx-color-muted, #718096);
  margin: 0.5rem 0 0;
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

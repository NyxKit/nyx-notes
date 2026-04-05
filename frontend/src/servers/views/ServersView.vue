<script setup lang="ts">
import { ref } from 'vue'
import { NyxFormField, NyxInput } from 'nyx-kit/components'
import { fetchAuthMode } from '@/auth/api'
import { useAuth } from '@/auth/composables'
import { api } from '@/shared/api'
import type { ServerMetadata } from '@/shared/types'

const auth = useAuth()

const serverMetadata = ref<ServerMetadata | null>(null)
const authModeLabel = ref<string>('local')

void (async () => {
  try {
    const [server, authMode] = await Promise.all([
      api<ServerMetadata>('/api/server'),
      fetchAuthMode(),
    ])
    serverMetadata.value = server
    authModeLabel.value = authMode.mode
  } catch {
    serverMetadata.value = null
  }
})()

function signOut() {
  auth.logout()
  window.location.reload()
}
</script>

<template>
  <div class="servers-page">
    <div class="servers-page__inner">
      <header class="servers-page__header">
        <h1 class="servers-page__title">Settings</h1>
      </header>

      <div class="servers-page__content">
        <section class="servers-section">
          <div class="servers-section__header">
            <h2>Server</h2>
          </div>

          <div class="servers-section__settings">
            <NyxFormField label="Server Name">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.name ?? 'Main Server'" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Root Path">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.root_path ?? ''" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Server Slug">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.slug ?? ''" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Auth Mode">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="authModeLabel" disabled />
              </template>
            </NyxFormField>
          </div>
        </section>

        <section v-if="auth.isAuthenticated.value" class="servers-section">
          <div class="servers-section__header">
            <h2>Session</h2>
          </div>
          <div class="servers-section__actions">
            <NyxButton @click="signOut">Sign Out</NyxButton>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>


<style scoped>
.servers-page {
  min-height: 100vh;
  padding: 2rem 1rem;
  overflow-y: auto;
}

.servers-page__inner {
  max-width: 600px;
  margin: 0 auto;
}

.servers-page__header {
  margin-bottom: 2rem;
}

.servers-page__title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.servers-page__content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.servers-page__separator {
  height: 1px;
  background: var(--nyx-c-border, rgba(255, 255, 255, 0.08));
}

.servers-section {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-lg);
  padding: 1.25rem;
}

.servers-section__settings {
  display: grid;
  gap: 1rem;
}

.servers-section__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.servers-section__header h2 {
  font-size: 0.875rem;
  font-weight: 600;
  margin: 0;
}

.servers-section__actions {
  display: flex;
  gap: 0.5rem;
}

.servers-section__empty {
  text-align: center;
  padding: 2rem;
  color: var(--nyx-c-text-3);
}

.servers-section__list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.servers-section__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem;
  border-radius: var(--nyx-radius-md);
  cursor: pointer;
  transition: background 0.2s;
}

.servers-section__item:hover {
  background: var(--nyx-c-bg);
}

.servers-section__item--active {
  background: var(--nyx-c-primary);
  background: rgba(139, 92, 246, 0.15);
}

.servers-section__item--active:hover {
  background: rgba(139, 92, 246, 0.2);
}

.profile-card {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex: 1;
}

.profile-card__icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--nyx-c-bg);
  border-radius: var(--nyx-radius-md);
  color: var(--nyx-c-text-2);
  flex-shrink: 0;
}

.profile-card__info {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
  min-width: 0;
  flex: 1;
}

.profile-card__name {
  font-weight: 500;
  font-size: 0.875rem;
}

.profile-card__detail {
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.profile-card__status {
  flex-shrink: 0;
}

.profile-card__badge {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  padding: 0.25rem 0.5rem;
  border-radius: var(--nyx-radius-sm);
  font-weight: 500;
}

.profile-card__badge--success {
  background: rgba(72, 187, 120, 0.15);
  color: #48bb78;
}

.profile-card__badge--warning {
  background: rgba(237, 137, 54, 0.15);
  color: #ed8936;
}

.profile-card__indicator {
  color: var(--nyx-c-primary);
}

.profile-card__actions {
  display: flex;
  gap: 0.5rem;
  flex-shrink: 0;
}
</style>

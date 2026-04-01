<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { NyxButton, NyxIcon } from 'nyx-kit/components'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'

const router = useRouter()
const auth = useAuth()
const profilesStore = useWorkspaceProfiles()

const profiles = computed(() => profilesStore.profiles.value)
const activeProfile = computed(() => profilesStore.activeProfile.value)
const canAddLocal = computed(() => !profiles.value.some(profile => profile.type === 'local'))

async function activateProfile(profileId: string) {
  const result = profilesStore.setActiveProfile(profileId)
  if (!result) return
  await auth.bootstrapActiveProfile()
  await router.push(result.profile.last_route || '/')
}

async function signOutActiveProfile() {
  auth.logout()
}

async function addLocalProfile() {
  profilesStore.createLocalProfile()
  await auth.bootstrapActiveProfile()
  await router.push('/')
}

function goToAddServer() {
  router.push('/login?add=remote')
}

function goToManageServer() {
  router.push('/login?manage=active')
}
</script>

<template>
  <div class="servers-page">
    <div class="servers-page__inner">
      <header class="servers-page__header">
        <h1 class="servers-page__title">Servers</h1>
      </header>

      <div class="servers-page__content">
        <section class="servers-section">
          <div class="servers-section__header">
            <h2>Active Profile</h2>
          </div>
          <div v-if="activeProfile" class="servers-section__card">
            <div class="profile-card">
              <div class="profile-card__icon">
                <NyxIcon v-if="activeProfile.type === 'local'" name="layout-grid" :size="24" />
                <NyxIcon v-else name="server" :size="24" />
              </div>
              <div class="profile-card__info">
                <span class="profile-card__name">{{ activeProfile.display_name }}</span>
                <span v-if="activeProfile.type === 'remote'" class="profile-card__detail">{{ activeProfile.server_url }}</span>
                <span v-if="activeProfile.type === 'local'" class="profile-card__detail">Local workspace</span>
              </div>
              <div class="profile-card__status">
                <span v-if="auth.isAuthenticated.value" class="profile-card__badge profile-card__badge--success">Signed in</span>
                <span v-else class="profile-card__badge profile-card__badge--warning">Signed out</span>
              </div>
            </div>
            <div class="profile-card__actions">
              <NyxButton v-if="activeProfile.type === 'remote' && !auth.isAuthenticated.value" @click="router.push('/login')">
                Sign In
              </NyxButton>
              <NyxButton v-if="activeProfile.type === 'remote' && auth.isAuthenticated.value" @click="signOutActiveProfile">
                Sign Out
              </NyxButton>
            </div>
          </div>
        </section>

        <section class="servers-section">
          <div class="servers-section__header">
            <h2>All Profiles</h2>
            <div class="servers-section__actions">
              <NyxButton v-if="canAddLocal" @click="addLocalProfile">Add Local</NyxButton>
              <NyxButton @click="goToAddServer">Add Server</NyxButton>
            </div>
          </div>

          <div v-if="profiles.length === 0" class="servers-section__empty">
            <p>No profiles yet. Add a local workspace or connect to a server.</p>
          </div>

          <div v-else class="servers-section__list">
            <div
              v-for="profile in profiles"
              :key="profile.id"
              class="servers-section__item"
              :class="{ 'servers-section__item--active': profile.id === activeProfile?.id }"
            >
              <div class="profile-card" @click="activateProfile(profile.id)">
                <div class="profile-card__icon">
                  <LayoutGrid v-if="profile.type === 'local'" :size="20" />
                  <Server v-else :size="20" />
                </div>
                <div class="profile-card__info">
                  <span class="profile-card__name">{{ profile.display_name }}</span>
                  <span v-if="profile.type === 'remote'" class="profile-card__detail">{{ profile.server_url }}</span>
                  <span v-else class="profile-card__detail">Local workspace</span>
                </div>
                <div v-if="profile.id === activeProfile?.id" class="profile-card__indicator">
                  <Check :size="16" />
                </div>
              </div>
              <div v-if="profile.type === 'remote'" class="profile-card__actions">
                <NyxButton @click.stop="goToManageServer">Edit</NyxButton>
              </div>
            </div>
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

.servers-section {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-lg);
  padding: 1.25rem;
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

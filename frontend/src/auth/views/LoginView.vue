<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NyxButton, NyxCard } from 'nyx-kit/components'
import { InstallationModeStep, RemoteProfileForm, useAuth } from '@/auth'
import { useWorkspaceProfiles } from '@/shared/composables'
import type { InstallationMode } from '@/auth/types/profileSetup'
import type { ServerSetupChoice } from '@/auth/types/profileSetup'
import type { RemoteProfileDraft } from '@/shared/types'

const router = useRouter()
const route = useRoute()
const auth = useAuth()
const workspaceProfiles = useWorkspaceProfiles()

const selectedMode = ref<InstallationMode | null>(null)
const serverChoice = ref<ServerSetupChoice | null>(null)
const error = ref<string | null>(null)
const loading = ref(false)

const hasProfiles = computed(() => workspaceProfiles.profiles.value.length > 0)
const addingRemoteProfile = computed(() => route.query.add === 'remote')
const managingActiveRemote = computed(() => route.query.manage === 'active')
const activeRemoteProfile = computed(() =>
  workspaceProfiles.activeProfile.value?.type === 'remote'
    ? workspaceProfiles.activeProfile.value
    : null
)

async function completeLocalSetup() {
  error.value = null
  workspaceProfiles.createLocalProfile()
  await auth.bootstrapActiveProfile()
  await router.push(auth.personalOverviewRoute.value)
}

function chooseServerMode(choice: ServerSetupChoice) {
  selectedMode.value = 'server'
  serverChoice.value = choice
  error.value = null
}

function chooseMode(mode: InstallationMode) {
  selectedMode.value = mode

  if (mode === 'local') {
    void completeLocalSetup()
    return
  }

  serverChoice.value = null
}

async function connectExistingServer(draft: RemoteProfileDraft) {
  loading.value = true
  error.value = null

  try {
    workspaceProfiles.addRemoteProfile(draft)
    await auth.bootstrapActiveProfile()

    if (auth.isAuthenticated.value) {
      await router.push((route.query.redirect as string) ?? '/')
      return
    }

    if (auth.authMode.value === 'oidc') {
      error.value = 'This remote server uses OIDC, which is not supported in the multi-profile flow yet.'
    }
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : 'Unable to connect to this server profile.'
  } finally {
    loading.value = false
  }
}

async function signInRemoteProfile(draft: RemoteProfileDraft) {
  loading.value = true
  error.value = null

  try {
    if (activeRemoteProfile.value) {
      workspaceProfiles.updateRemoteProfile(activeRemoteProfile.value.id, draft)
      await auth.login(draft.username, draft.password)
      await router.push((route.query.redirect as string) ?? '/')
    }
  } catch {
    error.value = 'Invalid credentials'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login">
    <NyxCard class="login__card">
      <h1>Nyx Notes</h1>

      <template v-if="!hasProfiles">
        <InstallationModeStep
          :selected-mode="selectedMode"
          :server-choice="serverChoice"
          @select-mode="chooseMode"
          @select-server-choice="chooseServerMode"
        />

        <NyxCard v-if="serverChoice === 'setup_new_server'" class="login__subcard">
          <h2>Set Up a New Server</h2>
          <p>Use Docker or a server binary to start Nyx Notes with `AUTH_MODE=secret_key` on your NAS or home server, then come back here and connect to it.</p>
          <NyxButton @click="serverChoice = 'connect_existing_server'">Continue to Connection</NyxButton>
        </NyxCard>

        <RemoteProfileForm
          v-if="serverChoice === 'connect_existing_server'"
          submit-label="Connect Server"
          :loading="loading"
          @submit="connectExistingServer"
        />
      </template>

      <template v-else-if="addingRemoteProfile">
        <p>Add another remote server profile.</p>
        <RemoteProfileForm
          submit-label="Add Server"
          :loading="loading"
          @submit="connectExistingServer"
        />
      </template>

      <template v-else-if="managingActiveRemote && activeRemoteProfile">
        <p>Update the active remote profile and re-authenticate it.</p>
        <RemoteProfileForm
          :initial-value="{
            display_name: activeRemoteProfile.display_name,
            server_url: activeRemoteProfile.server_url,
            username: activeRemoteProfile.username,
          }"
          submit-label="Update Server"
          :loading="loading"
          @submit="signInRemoteProfile"
        />
      </template>

      <template v-else-if="activeRemoteProfile && !auth.isAuthenticated.value">
        <p>Sign in to <strong>{{ activeRemoteProfile.display_name }}</strong> to continue.</p>
        <RemoteProfileForm
          :initial-value="{
            display_name: activeRemoteProfile.display_name,
            server_url: activeRemoteProfile.server_url,
            username: activeRemoteProfile.username,
          }"
          submit-label="Sign In"
          :loading="loading || auth.bootstrapping.value"
          @submit="signInRemoteProfile"
        />
      </template>

      <template v-else>
        <p>Loading workspace…</p>
      </template>

      <p v-if="error" class="login__error">{{ error }}</p>
    </NyxCard>
  </div>
</template>

<style scoped>
.login {
  display: flex;
  min-height: 100vh;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.login__card {
  width: min(32rem, 100%);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.login__subcard {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.login__error {
  color: var(--nyx-color-danger, #e53e3e);
  font-size: 0.875rem;
}
</style>

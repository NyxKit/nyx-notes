<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NyxCard } from 'nyx-kit/components'
import { InitialSetupForm, InstallationModeStep, RemoteProfileForm, SignInForm, useAuth } from '@/auth'
import { useWorkspaceProfiles } from '@/shared/composables'
import { InstallationMode, ServerSetupChoice } from '@/auth/types/profileSetup'
import { AuthMode, RoutePath, RouteQueryKey, RouteQueryValue, WorkspaceProfileType } from '@/shared/types'
import type { RemoteProfileDraft } from '@/shared/types'
import { RouteName } from '@/shared/types/router'

const router = useRouter()
const route = useRoute()
const auth = useAuth()
const workspaceProfiles = useWorkspaceProfiles()

const selectedMode = ref<InstallationMode | null>(null)
const serverChoice = ref<ServerSetupChoice | null>(null)
const error = ref<string | null>(null)
const loading = ref(false)
const initialized = ref<boolean | null>(null)

const hasProfiles = computed(() => workspaceProfiles.profiles.value.length > 0)
const addingRemoteProfile = computed(() => route.query[RouteQueryKey.Add] === RouteQueryValue.Remote)
const managingActiveRemote = computed(() => route.query[RouteQueryKey.Manage] === RouteQueryValue.Active)
const activeRemoteProfile = computed(() =>
  workspaceProfiles.activeProfile.value?.type === WorkspaceProfileType.Remote
    ? workspaceProfiles.activeProfile.value
    : null
)

const needsSetup = computed(() => 
  hasProfiles.value && 
  initialized.value === false
)

onMounted(async () => {
  if (route.path === RoutePath.Setup) {
    return
  }
  if (hasProfiles.value) {
    initialized.value = await auth.checkInitialized()
  }
})

watch(() => route.path, async (path) => {
  if (path === RoutePath.Setup) {
    return
  }
  if (hasProfiles.value && initialized.value === null) {
    initialized.value = await auth.checkInitialized()
  }
})

async function completeSetup() {
  initialized.value = true
  await auth.bootstrapActiveProfile()
  if (auth.isAuthenticated.value) {
    await router.push(auth.personalOverviewRoute.value)
  } else {
    await router.push({ name: RouteName.Login })
  }
}

async function completeLocalSetup() {
  error.value = null
  workspaceProfiles.createLocalProfile()
  await auth.bootstrapActiveProfile()
  initialized.value = await auth.checkInitialized()
  if (initialized.value === false) {
    await router.push({ name: RouteName.Setup })
  } else {
    await router.push(auth.personalOverviewRoute.value)
  }
}

function chooseServerMode(choice: ServerSetupChoice) {
  selectedMode.value = InstallationMode.Server
  serverChoice.value = choice
  error.value = null
}

function chooseMode(mode: InstallationMode) {
  selectedMode.value = mode

  if (mode === InstallationMode.Local) {
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
      await router.push((route.query[RouteQueryKey.Redirect] as string) ?? RoutePath.Home)
      return
    }

    if (auth.authMode.value === AuthMode.Oidc) {
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
    const serverUrl = draft.server_url
    await auth.login(draft.username, draft.password, serverUrl)
    await router.push((route.query[RouteQueryKey.Redirect] as string) ?? RoutePath.Home)
  } catch {
    error.value = 'Invalid credentials'
  } finally {
    loading.value = false
  }
}

async function signIn(data: { username: string; password: string }) {
  loading.value = true
  error.value = null

  try {
    await auth.login(data.username, data.password)
    await router.push(auth.personalOverviewRoute.value)
  } catch {
    error.value = 'Invalid credentials'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login">
    <NyxCard class="login__card" title="Nyx Notes">
      <template v-if="!hasProfiles">
        <InstallationModeStep
          :selected-mode="selectedMode"
          :server-choice="serverChoice"
          @select-mode="chooseMode"
          @select-server-choice="chooseServerMode"
        />

        <NyxCard v-if="serverChoice === ServerSetupChoice.SetupNewServer" class="login__subcard">
          <h2>Set Up a New Server</h2>
          <p>Use Docker or a server binary to start Nyx Notes with `AUTH_MODE=secret_key` on your NAS or home server, then come back here and connect to it.</p>
          <NyxButton @click="serverChoice = ServerSetupChoice.ConnectExistingServer">Continue to Connection</NyxButton>
        </NyxCard>

        <RemoteProfileForm
          v-if="serverChoice === ServerSetupChoice.ConnectExistingServer"
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

      <template v-else-if="auth.authMode.value === AuthMode.SecretKey && !auth.isAuthenticated.value">
        <SignInForm
          :loading="loading || auth.bootstrapping.value"
          @submit="signIn"
        />
      </template>

      <template v-else-if="needsSetup">
        <InitialSetupForm :loading="loading" @complete="completeSetup" />
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

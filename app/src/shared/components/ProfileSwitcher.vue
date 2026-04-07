<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { NyxButton, NyxCard } from 'nyx-kit/components'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { RouteName } from '@/shared/types'

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

async function removeActiveProfile() {
  const profile = activeProfile.value
  if (!profile || profile.type === 'local') return

  auth.clearProfileSession(profile.id)
  profilesStore.removeProfile(profile.id)
  await auth.bootstrapActiveProfile()

  if (!profilesStore.activeProfile.value) {
    await router.push({ name: RouteName.Login })
    return
  }

  await router.push(profilesStore.activeProfile.value.last_route || '/')
}

async function signOutActiveProfile() {
  auth.logout()
  await router.push({ name: RouteName.Login })
}

async function addLocalProfile() {
  profilesStore.createLocalProfile()
  await auth.bootstrapActiveProfile()
  await router.push(auth.personalOverviewRoute.value)
}
</script>

<template>
  <NyxCard class="profile-switcher">
    <div class="profile-switcher__header">
      <h2>Profiles</h2>
      <p>{{ activeProfile?.display_name ?? 'No active profile' }}</p>
    </div>

    <div class="profile-switcher__list">
      <NyxButton
        v-for="profile in profiles"
        :key="profile.id"
        @click="activateProfile(profile.id)"
      >
        {{ profile.display_name }}
      </NyxButton>
    </div>

    <div class="profile-switcher__actions">
      <NyxButton @click="router.push({ name: RouteName.Login, query: { add: 'remote' } })">Add Server</NyxButton>
      <NyxButton v-if="canAddLocal" @click="addLocalProfile">Add This Server</NyxButton>
      <NyxButton v-if="activeProfile?.type === 'remote'" @click="router.push({ name: RouteName.Login, query: { manage: 'active' } })">Edit Active Server</NyxButton>
      <NyxButton v-if="activeProfile?.type === 'remote'" @click="signOutActiveProfile">Sign Out</NyxButton>
      <NyxButton v-if="activeProfile?.type === 'remote'" @click="removeActiveProfile">Remove Active Server</NyxButton>
    </div>
  </NyxCard>
</template>

<style scoped>
.profile-switcher {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin: 0.75rem;
}

.profile-switcher__header {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.profile-switcher__list,
.profile-switcher__actions {
  display: grid;
  gap: 0.5rem;
}
</style>

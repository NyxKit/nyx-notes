<script setup lang="ts">
import { useRouter } from 'vue-router'
import { NyxCard } from 'nyx-kit/components'
import { InitialSetupForm, useAuth } from '@/auth'

const router = useRouter()
const auth = useAuth()

async function completeSetup() {
  await auth.bootstrapActiveProfile()
  await router.push(auth.personalOverviewRoute.value)
}
</script>

<template>
  <div class="setup">
    <NyxCard class="setup__card" title="Welcome to Nyx Notes">
      <p class="setup__card-description">Set up your first admin account to get started.</p>

      <InitialSetupForm @complete="completeSetup" />
    </NyxCard>
  </div>
</template>

<style scoped>
.setup {
  display: flex;
  min-height: 100vh;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.setup__card {
  width: min(32rem, 100%);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.setup__card-description {
  margin-bottom: 1rem;
}
</style>

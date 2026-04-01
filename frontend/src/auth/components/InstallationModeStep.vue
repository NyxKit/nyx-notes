<script setup lang="ts">
import { NyxButton, NyxCard } from 'nyx-kit/components'
import type { InstallationMode, ServerSetupChoice } from '@/auth/types/profileSetup'

defineProps<{
  selectedMode?: InstallationMode | null
  serverChoice?: ServerSetupChoice | null
}>()

const emit = defineEmits<{
  selectMode: [mode: InstallationMode]
  selectServerChoice: [choice: ServerSetupChoice]
}>()
</script>

<template>
  <div class="setup-step">
    <NyxCard class="setup-step__card">
      <h2>How should Nyx Notes start?</h2>
      <p>Pick a local workspace or connect this client to a self-hosted server.</p>

      <div class="setup-step__actions">
        <NyxButton @click="emit('selectMode', 'local')">Use Local Workspace</NyxButton>
        <NyxButton @click="emit('selectMode', 'server')">Use Server</NyxButton>
      </div>

      <div v-if="selectedMode === 'server'" class="setup-step__server-actions">
        <NyxButton @click="emit('selectServerChoice', 'setup_new_server')">Set Up a New Server</NyxButton>
        <NyxButton @click="emit('selectServerChoice', 'connect_existing_server')">Connect to Existing Server</NyxButton>
      </div>
    </NyxCard>
  </div>
</template>

<style scoped>
.setup-step {
  width: 100%;
}

.setup-step__card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.setup-step__actions,
.setup-step__server-actions {
  display: grid;
  gap: 0.75rem;
}
</style>

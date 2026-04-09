<script setup lang="ts">
import { NyxButton, NyxCard } from 'nyx-kit/components'
import { InstallationMode, ServerSetupChoice } from '@/auth/types/profileSetup'

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
      <p>Use this server directly or connect this client to another self-hosted server.</p>

      <div class="setup-step__actions">
        <NyxButton @click="emit('selectMode', InstallationMode.Local)">Use This Server</NyxButton>
        <NyxButton @click="emit('selectMode', InstallationMode.Server)">Use Server</NyxButton>
      </div>

      <div v-if="selectedMode === InstallationMode.Server" class="setup-step__server-actions">
        <NyxButton @click="emit('selectServerChoice', ServerSetupChoice.SetupNewServer)">Set Up a New Server</NyxButton>
        <NyxButton @click="emit('selectServerChoice', ServerSetupChoice.ConnectExistingServer)">Connect to Existing Server</NyxButton>
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

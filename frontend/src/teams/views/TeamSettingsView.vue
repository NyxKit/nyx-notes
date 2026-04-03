<script setup lang="ts">
import { useRouter } from 'vue-router'
import { NyxButton } from 'nyx-kit/components'

const router = useRouter()
</script>

<template>
  <div class="team-settings-placeholder">
    <NyxButton @click="router.back()">← Back</NyxButton>
    <h1>Team Settings Unavailable</h1>
    <p>Team management is not part of the active MVP frontend.</p>
  </div>
</template>

<style scoped>
.team-settings-placeholder {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 2rem;
}
</style>

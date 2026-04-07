<script setup lang="ts">
import { computed } from 'vue'
import { useWorkspaceProfiles } from '@/shared/composables'

const { activeProfileId } = useWorkspaceProfiles()
const routerViewKey = computed(() => activeProfileId.value ?? 'no-profile')
</script>

<template>
  <RouterView :key="routerViewKey" />
</template>

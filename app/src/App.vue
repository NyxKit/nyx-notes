<script setup lang="ts">
import { computed } from 'vue'
import { usePageTitle, useWorkspaceProfiles } from '@/shared/composables'

usePageTitle()

const { activeProfileId } = useWorkspaceProfiles()
const routerViewKey = computed(() => activeProfileId.value ?? 'no-profile')
</script>

<template>
  <RouterView :key="routerViewKey" />
</template>

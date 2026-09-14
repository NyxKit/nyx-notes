<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const routerViewKey = computed(() => route.fullPath)
</script>

<template>
  <RouterView :key="routerViewKey" />
</template>

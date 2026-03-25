<script setup lang="ts">
import { computed } from 'vue'

import {
  homeRaw,
  bookRaw,
  starRaw,
  briefcaseRaw,
  codeRaw,
  penRaw,
  heartRaw,
  globeRaw,
  lockRaw,
  rocketRaw,
  lightbulbRaw,
  musicRaw,
  cameraRaw,
  folderRaw,
  compassRaw,
  flaskRaw,
  graduationCapRaw,
  chartRaw,
  leafRaw,
  diamondRaw,
} from '@/vaults/assets/icons'

const icons: Record<string, string> = {
  'home': homeRaw,
  'book': bookRaw,
  'star': starRaw,
  'briefcase': briefcaseRaw,
  'code': codeRaw,
  'pen': penRaw,
  'heart': heartRaw,
  'globe': globeRaw,
  'lock': lockRaw,
  'rocket': rocketRaw,
  'lightbulb': lightbulbRaw,
  'music': musicRaw,
  'camera': cameraRaw,
  'folder': folderRaw,
  'compass': compassRaw,
  'flask': flaskRaw,
  'graduation-cap': graduationCapRaw,
  'chart': chartRaw,
  'leaf': leafRaw,
  'diamond': diamondRaw,
}

const props = defineProps<{
  slug?: string
  size?: number
}>()

const svg = computed(() => icons[props.slug ?? ''] ?? folderRaw)
</script>

<template>
  <span
    class="vault-icon"
    :style="props.size !== undefined ? { width: `${props.size}px`, height: `${props.size}px` } : {}"
    v-html="svg"
    aria-hidden="true"
  />
</template>

<style scoped>
.vault-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.vault-icon :deep(svg) {
  width: 100%;
  height: 100%;
}
</style>

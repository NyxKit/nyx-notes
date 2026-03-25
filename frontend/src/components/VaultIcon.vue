<script setup lang="ts">
import { computed } from 'vue'

import homeRaw from '@/assets/icons/vaults/home.svg?raw'
import bookRaw from '@/assets/icons/vaults/book.svg?raw'
import starRaw from '@/assets/icons/vaults/star.svg?raw'
import briefcaseRaw from '@/assets/icons/vaults/briefcase.svg?raw'
import codeRaw from '@/assets/icons/vaults/code.svg?raw'
import penRaw from '@/assets/icons/vaults/pen.svg?raw'
import heartRaw from '@/assets/icons/vaults/heart.svg?raw'
import globeRaw from '@/assets/icons/vaults/globe.svg?raw'
import lockRaw from '@/assets/icons/vaults/lock.svg?raw'
import rocketRaw from '@/assets/icons/vaults/rocket.svg?raw'
import lightbulbRaw from '@/assets/icons/vaults/lightbulb.svg?raw'
import musicRaw from '@/assets/icons/vaults/music.svg?raw'
import cameraRaw from '@/assets/icons/vaults/camera.svg?raw'
import folderRaw from '@/assets/icons/vaults/folder.svg?raw'
import compassRaw from '@/assets/icons/vaults/compass.svg?raw'
import flaskRaw from '@/assets/icons/vaults/flask.svg?raw'
import graduationCapRaw from '@/assets/icons/vaults/graduation-cap.svg?raw'
import chartRaw from '@/assets/icons/vaults/chart.svg?raw'
import leafRaw from '@/assets/icons/vaults/leaf.svg?raw'
import diamondRaw from '@/assets/icons/vaults/diamond.svg?raw'

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

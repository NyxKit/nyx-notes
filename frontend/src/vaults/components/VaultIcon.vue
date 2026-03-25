<script setup lang="ts">
import { computed } from 'vue'

import homeRaw from '@/vaults/assets/icons/home.svg?raw'
import bookRaw from '@/vaults/assets/icons/book.svg?raw'
import starRaw from '@/vaults/assets/icons/star.svg?raw'
import briefcaseRaw from '@/vaults/assets/icons/briefcase.svg?raw'
import codeRaw from '@/vaults/assets/icons/code.svg?raw'
import penRaw from '@/vaults/assets/icons/pen.svg?raw'
import heartRaw from '@/vaults/assets/icons/heart.svg?raw'
import globeRaw from '@/vaults/assets/icons/globe.svg?raw'
import lockRaw from '@/vaults/assets/icons/lock.svg?raw'
import rocketRaw from '@/vaults/assets/icons/rocket.svg?raw'
import lightbulbRaw from '@/vaults/assets/icons/lightbulb.svg?raw'
import musicRaw from '@/vaults/assets/icons/music.svg?raw'
import cameraRaw from '@/vaults/assets/icons/camera.svg?raw'
import folderRaw from '@/vaults/assets/icons/folder.svg?raw'
import compassRaw from '@/vaults/assets/icons/compass.svg?raw'
import flaskRaw from '@/vaults/assets/icons/flask.svg?raw'
import graduationCapRaw from '@/vaults/assets/icons/graduation-cap.svg?raw'
import chartRaw from '@/vaults/assets/icons/chart.svg?raw'
import leafRaw from '@/vaults/assets/icons/leaf.svg?raw'
import diamondRaw from '@/vaults/assets/icons/diamond.svg?raw'

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

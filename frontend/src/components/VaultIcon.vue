<script setup lang="ts">
import { computed } from 'vue'

import homeRaw from '@/assets/icons/home.svg?raw'
import bookRaw from '@/assets/icons/book.svg?raw'
import starRaw from '@/assets/icons/star.svg?raw'
import briefcaseRaw from '@/assets/icons/briefcase.svg?raw'
import codeRaw from '@/assets/icons/code.svg?raw'
import penRaw from '@/assets/icons/pen.svg?raw'
import heartRaw from '@/assets/icons/heart.svg?raw'
import globeRaw from '@/assets/icons/globe.svg?raw'
import lockRaw from '@/assets/icons/lock.svg?raw'
import rocketRaw from '@/assets/icons/rocket.svg?raw'
import lightbulbRaw from '@/assets/icons/lightbulb.svg?raw'
import musicRaw from '@/assets/icons/music.svg?raw'
import cameraRaw from '@/assets/icons/camera.svg?raw'
import folderRaw from '@/assets/icons/folder.svg?raw'
import compassRaw from '@/assets/icons/compass.svg?raw'
import flaskRaw from '@/assets/icons/flask.svg?raw'
import graduationCapRaw from '@/assets/icons/graduation-cap.svg?raw'
import chartRaw from '@/assets/icons/chart.svg?raw'
import leafRaw from '@/assets/icons/leaf.svg?raw'
import diamondRaw from '@/assets/icons/diamond.svg?raw'

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

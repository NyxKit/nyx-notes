<script setup lang="ts">
import { NyxCard } from 'nyx-kit/components'

defineProps<{
  images: string[]
  title?: string
}>()
</script>

<template>
  <section v-if="images.length" class="image-shelf">
    <h3 v-if="title" class="image-shelf__title">{{ title }}</h3>
    <div class="image-shelf__grid">
      <NyxCard
        v-for="image in images"
        :key="image"
        class="image-shelf__card"
        :src="image"
      />
    </div>
  </section>
</template>

<style scoped>
.image-shelf {
  display: grid;
  gap: 0.75rem;
}

.image-shelf__title {
  margin: 0;
  font-size: 0.8125rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--nyx-c-text-3);
}

.image-shelf__grid {
  display: grid;
  gap: 0.75rem;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
}

.image-shelf__card {
  width: 100%;
}
</style>

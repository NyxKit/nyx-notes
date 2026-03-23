<script setup lang="ts">
import VaultIcon from './VaultIcon.vue'

const ICONS = [
  'home', 'book', 'star', 'briefcase', 'code',
  'pen', 'heart', 'globe', 'lock', 'rocket',
  'lightbulb', 'music', 'camera', 'folder', 'compass',
  'flask', 'graduation-cap', 'chart', 'leaf', 'diamond',
]

const props = defineProps<{
  modelValue?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [slug: string | undefined]
}>()

function select(slug: string) {
  emit('update:modelValue', props.modelValue === slug ? undefined : slug)
}
</script>

<template>
  <div class="icon-picker" role="group" aria-label="Choose an icon">
    <button
      v-for="slug in ICONS"
      :key="slug"
      type="button"
      class="icon-picker__btn"
      :class="{ 'icon-picker__btn--active': modelValue === slug }"
      :title="slug"
      :aria-pressed="modelValue === slug"
      @click="select(slug)"
    >
      <VaultIcon :slug="slug" :size="20" />
    </button>
  </div>
</template>

<style scoped>
.icon-picker {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 0.375rem;
}

.icon-picker__btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: var(--nyx-radius-md, 0.5rem);
  border: none;
  background: transparent;
  color: var(--nyx-c-text-2);
  cursor: pointer;
  transition: background 0.12s, color 0.12s;
}

.icon-picker__btn:hover {
  background: var(--nyx-c-bg-mute);
  color: var(--nyx-c-text-1);
}

.icon-picker__btn--active {
  background: var(--nyx-c-primary-container, #49435f);
  color: var(--nyx-c-on-primary-container, #d5cbee);
}
</style>

<script setup lang="ts">
import { NyxIcon } from 'nyx-kit/components'
import type { RouteLocationRaw } from 'vue-router'

defineProps<{
  to?: RouteLocationRaw
  icon: string
  active?: boolean
}>()
</script>

<template>
  <RouterLink
    v-if="to"
    :to="to"
    class="sidebar-nav-item"
    :class="{ 'sidebar-nav-item--active': active }"
  >
    <NyxIcon :name="icon" :size="16" />
    <slot />
  </RouterLink>

  <button
    v-else
    type="button"
    class="sidebar-nav-item sidebar-nav-item--button"
    :class="{ 'sidebar-nav-item--active': active }"
  >
    <NyxIcon :name="icon" :size="16" />
    <slot />
  </button>
</template>

<style scoped>
.sidebar-nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  border-radius: var(--nyx-radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
  text-decoration: none;
  transition: background 0.2s, color 0.2s;
}

.sidebar-nav-item--button {
  width: 100%;
  background: transparent;
  border: 0;
  text-align: left;
  font: inherit;
  cursor: pointer;
}

.sidebar-nav-item:hover {
  background: #25252b;
  color: var(--nyx-c-text-1);
}

.sidebar-nav-item--active {
  background: #49435f;
  color: var(--nyx-c-text-1);
}
</style>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { NyxCard } from 'nyx-kit/components'
import { VaultIcon } from '@/vaults/components'
import type { Vault } from '@/shared/types'

const model = defineModel<Vault>({ required: true })
</script>

<template>
  <RouterLink
    class="vault-item"
    :to="`/vaults/${model.id}`"
    :aria-label="`Open ${model.name}`"
  >
    <NyxCard class="vault-item__shell">
      <div class="vault-item__content">
        <div class="vault-item__text">
          <h3 class="vault-item__name">{{ model.name }}</h3>
          <p class="vault-item__slug">{{ model.slug }}</p>
          <p v-if="model.description" class="vault-item__description">{{ model.description }}</p>
        </div>
      </div>

      <VaultIcon
        :slug="model.icon || 'folder'"
        class="vault-item__bg-icon"
        aria-hidden="true"
      />
    </NyxCard>
  </RouterLink>
</template>

<style scoped>
.vault-item {
  display: block;
  margin-bottom: 1rem;
  text-decoration: none;
  color: inherit;
  break-inside: avoid;
}

.vault-item__shell {
  position: relative;
  aspect-ratio: 1 / 1;
  overflow: hidden;
  background: var(--nyx-browse-card-bg) !important;
  border: 0 !important;
  box-shadow: var(--nyx-browse-card-shadow) !important;
  border-radius: var(--nyx-radius-xl) !important;
  max-width: none !important;
  width: 100% !important;
  transition: background-color var(--nyx-speed-regular), transform var(--nyx-speed-regular);
}

.vault-item:hover .vault-item__shell,
.vault-item:focus-visible .vault-item__shell {
  background: var(--nyx-browse-card-hover) !important;
}

.vault-item:focus-visible .vault-item__shell {
  outline: 1px solid var(--nyx-browse-card-outline);
  outline-offset: 0;
}

.vault-item__content {
  position: relative;
  z-index: 1;
  height: 100%;
}

.vault-item__text {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.35rem;
}

.vault-item__name {
  margin: 0;
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--nyx-browse-card-text);
}

.vault-item__slug {
  margin: 0;
  font-family: 'Inter', sans-serif;
  font-size: 0.6875rem;
  line-height: 1.4;
  color: var(--nyx-browse-card-muted);
  overflow-wrap: anywhere;
}

.vault-item__description {
  margin: 0;
  font-family: 'Inter', sans-serif;
  font-size: 0.75rem;
  line-height: 1.55;
  color: var(--nyx-c-text-2);
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 4;
  line-clamp: 4;
}

.vault-item__bg-icon {
  position: absolute;
  right: 0;
  bottom: 0;
  width: 100%;
  height: auto;
  opacity: 0.12;
  pointer-events: none;
  fill: var(--nyx-c-text-1);
  transform: scale(1) translate(15%, 15%);
}

.vault-item :deep(.nyx-card__body) {
  display: flex;
  min-height: 100%;
  padding: 1.25rem;
}

.vault-item :deep(.nyx-card__body:last-child) {
  padding-bottom: 1.25rem;
}
</style>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { NyxCard, NyxBadge } from 'nyx-kit/components'
import VaultIcon from './VaultIcon.vue'
import { vaultRoute } from '@/shared/utils'
import type { Vault } from '@/shared/types'

const model = defineModel<Vault>({ required: true })
</script>

<template>
  <RouterLink
    class="vault-item"
    :to="vaultRoute(model)"
    :aria-label="`Open ${model.name}`"
  >
    <NyxCard class="vault-item__shell">
      <div class="vault-item__content">
        <h3 class="vault-item__name">{{ model.name }}</h3>
        <NyxBadge class="vault-item__slug">{{ model.slug }}</NyxBadge>
        <p v-if="model.description" class="vault-item__description">{{ model.description }}</p>
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
  line-height: 1.5;
}

.vault-item__shell {
  position: relative;
  aspect-ratio: 1 / 1;
  overflow: hidden;
  width: 100%;
  max-width: none;
}

.vault-item:focus-visible .vault-item__shell {
  outline: 1px solid var(--nyx-browse-card-outline);
  outline-offset: 0;
}

.vault-item__content {
  position: relative;
  z-index: 1;
  height: 100%;
  width: 100%;
}

.vault-item__name {
  font-family: 'Manrope', sans-serif;
  font-size: calc(var(--nyx-font-size-xl) * 1.5);
  font-weight: 600;
  color: var(--nyx-browse-card-text);
}

.vault-item__slug {
  position: absolute;
  bottom: 0;
  right: 0;
}

.vault-item__description {
  color: var(--nyx-c-text-3);
  max-width: 85%;
  margin-top: 0.5rem;
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 5;
  line-clamp: 5;
}

.vault-item__bg-icon {
  position: absolute;
  right: 0;
  bottom: 0;
  width: 100%;
  height: auto;
  opacity: 0.06;
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

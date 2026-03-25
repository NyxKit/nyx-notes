<script setup lang="ts">
import { NyxButton } from 'nyx-kit/components'
import { VaultIcon } from '@/vaults/components'
import type { Vault } from '@/shared/types'

const model = defineModel<Vault>({ required: true })

defineEmits<{
  click: []
}>()
</script>

<template>
  <NyxButton class="vault-item" @click="$emit('click')">
    <div class="vault-item__text">
      <span class="vault-item__name">{{ model.name }}</span>
      <span class="vault-item__slug">{{ model.slug }}</span>
    </div>
    <VaultIcon
      :slug="model.icon || 'folder'"
      class="vault-item__bg-icon"
      aria-hidden="true"
    />
  </NyxButton>
</template>

<style scoped>
.vault-item {
  position: relative;
  overflow: hidden;
  aspect-ratio: 1 / 1;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  align-items: flex-start;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 1.25rem;
  margin-bottom: 1rem;
  break-inside: avoid;
  cursor: pointer;
  text-align: left;
  border: 1px solid transparent;
  width: 100%;
  transition: border-color 0.15s, background 0.15s;
}

.vault-item:hover {
  border-color: var(--nyx-c-divider);
  background: var(--nyx-c-bg-mute);
}

.vault-item__text {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  position: relative;
  z-index: 1;
}

.vault-item__name {
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--nyx-c-text-1);
  line-height: 1.4;
}

.vault-item__slug {
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
  font-family: 'Inter', monospace;
}

.vault-item__bg-icon {
  position: absolute;
  right: -8%;
  top: 50%;
  transform: translateY(-50%);
  width: 70%;
  height: auto;
  opacity: 0.12;
  pointer-events: none;
  color: var(--nyx-c-text-1);
}
</style>

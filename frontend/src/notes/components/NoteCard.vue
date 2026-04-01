<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { NyxBadge, NyxCard } from 'nyx-kit/components'
import { NyxTheme, NyxVariant } from 'nyx-kit/types'
import type { BrowseNoteCardModel } from '@/shared/types'

defineProps<{
  note: BrowseNoteCardModel
}>()
</script>

<template>
  <RouterLink
    class="note-card"
    :to="note.href"
    :aria-label="`Open ${note.title || 'Untitled'} note`"
  >
    <NyxCard class="note-card__shell">
      <div class="note-card__content">
        <div class="note-card__text">
          <h3 class="note-card__title">{{ note.title || 'Untitled' }}</h3>
          <p v-if="note.description" class="note-card__description">{{ note.description }}</p>
        </div>

        <div v-if="note.tags.length" class="note-card__tags">
          <NyxBadge
            v-for="tag in note.tags.slice(0, 3)"
            :key="tag"
            :theme="NyxTheme.Primary"
            :variant="NyxVariant.Soft"
          >
            {{ tag }}
          </NyxBadge>
        </div>


        <footer class="note-card__meta">
          <span class="note-card__meta-origin">
            <span class="note-card__meta-origin-label">{{ note.server_label }}</span>
            <span class="note-card__meta-origin-separator">/</span>
            <span class="note-card__meta-origin-label">{{ note.vault_name }}</span>
          </span>
          <span class="note-card__meta-time">{{ note.updated_label }}</span>
        </footer>
      </div>
    </NyxCard>
  </RouterLink>
</template>

<style scoped>
.note-card {
  display: block;
  margin-bottom: 1rem;
  text-decoration: none;
  color: inherit;
  break-inside: avoid;
}

.note-card:hover .note-card__shell,
.note-card:focus-visible .note-card__shell {
  background: var(--nyx-browse-card-hover) !important;
}

.note-card:focus-visible .note-card__shell {
  outline: 1px solid var(--nyx-browse-card-outline);
  outline-offset: 0;
}

.note-card__shell {
  width: 100%;
  max-width: none;
}

.note-card__content {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
  min-height: 100%;
  width: 100%;
}

.note-card__meta {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 0.35rem;
  font-size: 0.6875rem;
  line-height: 1.4;
  color: var(--nyx-browse-card-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.note-card__meta-origin {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.note-card__origin-separator {
  opacity: 0.7;
}

.note-card__text {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.note-card__title {
  margin: 0;
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  line-height: 1.4;
  color: var(--nyx-browse-card-text);
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  line-clamp: 3;
}

.note-card__description {
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

.note-card__tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.note-card__date {
  margin: auto 0 0;
  font-size: 0.6875rem;
  line-height: 1.4;
  color: var(--nyx-browse-card-muted);
}

.note-card :deep(.nyx-card__body) {
  display: flex;
  min-height: 100%;
  padding: 1.25rem 1.25rem 1rem;
}

.note-card :deep(.nyx-card__body:last-child) {
  padding-bottom: 1rem;
}
</style>

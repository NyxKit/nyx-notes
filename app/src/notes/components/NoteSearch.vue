<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NyxInput } from 'nyx-kit/components'
import { NyxInputType } from 'nyx-kit/types'
import { RouteName } from '@/shared/types'

const router = useRouter()
const route = useRoute()

const search = ref(String(route.query.q ?? ''))
let searchTimeout: ReturnType<typeof setTimeout> | null = null
let syncingFromRoute = false

watch(() => [route.name, String(route.query.q ?? '')] as const, ([name, value]) => {
  if (name !== RouteName.Search) return
  syncingFromRoute = true
  search.value = value
})

watch(search, (value) => {
  if (syncingFromRoute) {
    syncingFromRoute = false
    return
  }

  if (searchTimeout) clearTimeout(searchTimeout)

  searchTimeout = setTimeout(() => {
    router.replace({
      name: RouteName.Search,
      query: value ? { q: value } : {},
    })
  }, 200)
})

onBeforeUnmount(() => {
  if (searchTimeout) clearTimeout(searchTimeout)
})
</script>

<template>
  <div class="note-search">
    <NyxInput
      v-model="search"
      :type="NyxInputType.Search"
      placeholder="Search notes..."
    />
  </div>
</template>

<style scoped>
.note-search {
  padding: 0.25rem 1rem 0.5rem;
  flex-shrink: 0;
}
</style>

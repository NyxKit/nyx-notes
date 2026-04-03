<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { RouteName } from '@/shared/types'
import SidebarNavItem from './SidebarNavItem.vue'

const route = useRoute()
const { personalOverviewRoute, serverVaultsRoute } = useAuth()
const { activeProfile } = useWorkspaceProfiles()

const serverLabel = computed(() => activeProfile.value?.display_name ?? 'Main Server')

const section = computed(() => {
  if (route.name === RouteName.Root) return 'personal'
  if (route.name === RouteName.ServerVaults) return 'server'
  if (route.name === RouteName.Favorites) return 'favorites'
  return 'notes'
})
</script>

<template>
  <nav class="sidebar-nav">

    <!-- App section -->
    <span class="sidebar-nav__section-label">Workspace</span>

    <SidebarNavItem
      :to="personalOverviewRoute"
      icon="layout-grid"
      :active="section === 'personal'"
    >
      Personal
    </SidebarNavItem>

    <SidebarNavItem
      :to="serverVaultsRoute"
      icon="server"
      :active="section === 'server'"
    >
      {{ serverLabel }}
    </SidebarNavItem>

    <SidebarNavItem
      :to="{ name: RouteName.Favorites }"
      icon="star"
      :active="section === 'favorites'"
    >
      Favorites
    </SidebarNavItem>

  </nav>
</template>

<style scoped>
.sidebar-nav {
  padding: 0 0.75rem;
  flex-shrink: 0;
}

/* Section label */
.sidebar-nav__section-label {
  padding: 0.5rem 0.5rem 0.25rem;
  font-size: 0.625rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--nyx-c-text-3);
}

</style>

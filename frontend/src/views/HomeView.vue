<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'

const router = useRouter()
const { vaults, load: loadVaults, setActive, create: createVault } = useVaults()
const { loadList, notes, create: createNote } = useNotes()

const LAST_NOTE_KEY = 'nyx_last_note'

onMounted(async () => {
  await loadVaults()

  const last = localStorage.getItem(LAST_NOTE_KEY)
  if (last) {
    try {
      const { vaultId, noteId } = JSON.parse(last)
      if (vaults.value.some(v => v.id === vaultId)) {
        router.replace(`/vaults/${vaultId}/notes/${noteId}`)
        return
      }
    } catch { /* malformed entry */ }
    localStorage.removeItem(LAST_NOTE_KEY)
  }

  for (const vault of vaults.value) {
    await loadList(vault.id)
    if (notes.value.length > 0) {
      setActive(vault)
      const latest = notes.value[0]
      router.replace(`/vaults/${vault.id}/notes/${latest.id}`)
      return
    }
  }
})

async function createFirst() {
  await loadVaults()

  let vault = vaults.value.find(v => v.slug === 'home') ?? vaults.value[0]
  if (!vault) {
    vault = await createVault({ slug: 'home', name: 'Home' })
  }

  setActive(vault)
  const meta = await createNote(vault.id, { title: 'Untitled', content: '' })
  router.push(`/vaults/${vault.id}/notes/${meta.id}`)
}
</script>

<template>
  <div class="home">

    <!-- Header bar -->
    <header class="home__header">
      <div class="home__header-left">
        <button class="home__icon-btn" disabled>
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
            <path d="M2 4.5h14M2 9h14M2 13.5h14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
        <span class="home__header-title">Untitled Note</span>
      </div>
      <div class="home__header-right">
        <button class="home__icon-btn" disabled>
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">
            <path d="M15 9C15 12.31 12.31 15 9 15c-.72 0-1.41-.12-2.05-.33L4 15.5l.61-2.84C4.22 11.79 3 10.52 3 9c0-3.31 2.69-6 6-6s6 2.69 6 6z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- Main content -->
    <main class="home__body">
      <div class="home__grid">

        <!-- Left: onboarding card -->
        <div class="home__welcome-card">
          <div class="home__welcome-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" aria-hidden="true">
              <rect x="6" y="4" width="16" height="20" rx="2" stroke="currentColor" stroke-width="1.5"/>
              <path d="M10 10h8M10 14h8M10 18h5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <path d="M20 20l4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
          </div>
          <h1 class="home__heading">Begin your next chapter.</h1>
          <p class="home__desc">
            The canvas is quiet, waiting for the first word. Organize your thoughts, draft your masterpiece, or simply capture a fleeting moment.
          </p>
          <button class="home__cta" @click="createFirst">New Note</button>
          <div class="home__shortcuts">
            <span class="home__shortcut"><kbd>⌘</kbd><span>N</span> New</span>
            <span class="home__shortcut"><kbd>⌘</kbd><span>O</span> Open</span>
            <span class="home__shortcut"><kbd>⌘</kbd><span>/</span> Shortcuts</span>
          </div>
        </div>

        <!-- Right: info cards -->
        <div class="home__card-stack">
          <div class="home__glass-card">
            <div class="home__glass-icon">
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path d="M10 2C6.13 2 3 5.13 3 9s3.13 7 7 7 7-3.13 7-7-3.13-7-7-7z" stroke="currentColor" stroke-width="1.25"/>
                <path d="M10 6v4l2.5 2.5" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
              </svg>
            </div>
            <span class="home__glass-label">Personal Vault</span>
            <p class="home__glass-sub">Your notes are stored locally and synced across sessions.</p>
          </div>
          <div class="home__glass-card home__glass-card--dim">
            <div class="home__glass-icon">
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path d="M10 3l1.5 4.5H16l-3.75 2.75L13.75 15 10 12.25 6.25 15l1.5-4.75L4 7.5h4.5L10 3z" stroke="currentColor" stroke-width="1.25" stroke-linejoin="round"/>
              </svg>
            </div>
            <span class="home__glass-label">Need inspiration?</span>
            <p class="home__glass-sub">Start with a blank canvas. The words will follow.</p>
          </div>
        </div>

      </div>
    </main>

    <!-- Footer -->
    <footer class="home__footer">
      <span class="home__footer-text">Nyx Notes — Silent Atelier</span>
    </footer>

  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--nyx-c-bg);
  color: var(--nyx-c-text-1);
}

/* Header */
.home__header {
  height: 64px;
  flex-shrink: 0;
  background: var(--nyx-c-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1rem;
  box-shadow: 0 1px 0 0 var(--nyx-c-divider);
}

.home__header-left,
.home__header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.home__header-title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
}

.home__icon-btn {
  background: none;
  border: none;
  color: var(--nyx-c-text-3);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border-radius: var(--nyx-radius-md);
  line-height: 0;
}

/* Body */
.home__body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  overflow: auto;
}

.home__grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
  max-width: 760px;
  width: 100%;
}

/* Welcome card */
.home__welcome-card {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 2.5rem 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: flex-start;
}

.home__welcome-icon {
  color: var(--nyx-c-primary);
  opacity: 0.7;
  line-height: 0;
}

.home__heading {
  font-family: 'Manrope', sans-serif;
  font-size: 1.625rem;
  font-weight: 700;
  line-height: 1.25;
  color: var(--nyx-c-text-1);
  margin: 0;
}

.home__desc {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--nyx-c-text-2);
  margin: 0;
}

.home__cta {
  background: linear-gradient(135deg, #cbc2e4 0%, #49435f 100%);
  color: #1a1821;
  font-family: 'Manrope', sans-serif;
  font-size: 0.875rem;
  font-weight: 700;
  border: none;
  cursor: pointer;
  padding: 0.75rem 1.75rem;
  border-radius: var(--nyx-radius-md);
  transition: opacity 0.2s;
  margin-top: 0.25rem;
}

.home__cta:hover {
  opacity: 0.9;
}

.home__shortcuts {
  margin-top: auto;
  display: flex;
  gap: 1.25rem;
  flex-wrap: wrap;
}

.home__shortcut {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

kbd {
  background: var(--nyx-c-bg-mute);
  border: 1px solid var(--nyx-c-divider);
  border-radius: var(--nyx-radius-xs);
  padding: 0.125rem 0.3125rem;
  font-family: 'Inter', monospace;
  font-size: 0.625rem;
  color: var(--nyx-c-text-2);
}

/* Glass cards */
.home__card-stack {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.home__glass-card {
  background: rgba(37, 37, 43, 0.6);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-radius: var(--nyx-radius-xl);
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.home__glass-card--dim {
  background: rgba(25, 25, 29, 0.3);
}

.home__glass-icon {
  color: var(--nyx-c-primary);
  opacity: 0.6;
  line-height: 0;
  margin-bottom: 0.25rem;
}

.home__glass-label {
  font-family: 'Manrope', sans-serif;
  font-size: 0.875rem;
  font-weight: 700;
  color: var(--nyx-c-text-1);
}

.home__glass-sub {
  font-size: 0.8125rem;
  color: var(--nyx-c-text-3);
  line-height: 1.5;
  margin: 0;
}

/* Footer */
.home__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.home__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}
</style>

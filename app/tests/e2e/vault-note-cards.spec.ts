import { expect, test } from '@playwright/test'

test('note masonry cards reuse the shared browse-card flow', async ({ page }) => {
  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

    if (!url.pathname.startsWith('/api/')) {
      await route.continue()
      return
    }

    if (url.pathname === '/api/auth/mode') {
      await route.fulfill({ json: { mode: 'local' } })
      return
    }

    if (url.pathname === '/api/vaults') {
      await route.fulfill({
        json: [
          { id: 'vault-1', slug: 'writing', name: 'Writing', owner: { type: 'user', id: 'user-1' }, permission: 'edit', icon: 'folder' },
        ],
      })
      return
    }

    if (url.pathname === '/api/vaults/vault-1/notes') {
      await route.fulfill({
        json: [
          {
            id: 'note-1',
            vault_id: 'vault-1',
            title: '',
            description: 'The first actual paragraph becomes the card summary.',
            author_id: 'user-1',
            tags: ['focus', 'draft'],
            category: null,
            created_at: '2026-03-26T10:30:00Z',
            updated_at: '2026-03-26T11:00:00Z',
            is_encrypted: false,
            permission: 'edit',
          },
        ],
      })
      return
    }

    if (url.pathname === '/api/vaults/vault-1/notes/note-1') {
      await route.fulfill({
        json: {
          meta: {
            id: 'note-1',
            vault_id: 'vault-1',
            title: '',
            author_id: 'user-1',
            tags: ['focus', 'draft'],
            category: null,
            created_at: '2026-03-26T10:30:00Z',
            updated_at: '2026-03-26T11:00:00Z',
            is_encrypted: false,
            permission: 'edit',
          },
          content: '# Draft',
        },
      })
      return
    }

    await route.fulfill({ json: [] })
  })

  await page.goto('/vaults/vault-1')

  await expect(page.getByRole('link', { name: 'Open Untitled note' })).toBeVisible()
  await expect(page.getByText('focus')).toBeVisible()

  await page.getByLabel('Open Untitled note').click()

  await expect(page).toHaveURL(/\/vaults\/vault-1\/notes\/note-1$/)
})

import { expect, test } from '@playwright/test'

test('sidebar search updates the global search view live', async ({ page }) => {
  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

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
            title: 'Project Plan',
            description: 'Roadmap summary.',
            author_id: 'user-1',
            tags: ['planning'],
            category: null,
            created_at: '2026-04-01T09:00:00Z',
            updated_at: '2026-04-01T10:00:00Z',
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
            title: 'Project Plan',
            description: 'Roadmap summary.',
            author_id: 'user-1',
            tags: ['planning'],
            category: null,
            created_at: '2026-04-01T09:00:00Z',
            updated_at: '2026-04-01T10:00:00Z',
            is_encrypted: false,
            permission: 'edit',
          },
          content: 'Body keyword content.',
        },
      })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/login')
  await page.getByRole('button', { name: 'Use Local Workspace' }).click()
  await page.getByPlaceholder('Search notes…').fill('keyword')

  await expect(page).toHaveURL(/\/notes\/search\?q=keyword/)
  await expect(page.getByRole('link', { name: 'Open Project Plan note' })).toBeVisible()
  await expect(page.getByText('Local')).toBeVisible()
  await expect(page.getByText('Writing')).toBeVisible()
})

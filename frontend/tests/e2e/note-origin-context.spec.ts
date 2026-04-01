import { expect, test } from '@playwright/test'

test('note cards show origin labels and the sidebar no longer shows a global new-note CTA', async ({ page }) => {
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

    await route.fulfill({ json: {} })
  })

  await page.goto('/login')
  await page.getByRole('button', { name: 'Use Local Workspace' }).click()
  await page.goto('/vaults/vault-1')

  await expect(page.getByText('Local')).toBeVisible()
  await expect(page.getByText('Writing')).toBeVisible()
  await expect(page.locator('.app-shell__sidebar').getByRole('button', { name: 'New Note' })).toHaveCount(0)
})

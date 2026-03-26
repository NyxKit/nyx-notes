import { expect, test } from '@playwright/test'

test('vault dashboard cards use the shared browse-card flow', async ({ page }) => {
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
          { id: 'vault-2', slug: 'archive', name: 'Archive', owner: { type: 'user', id: 'user-1' }, permission: 'edit', icon: 'folder' },
        ],
      })
      return
    }

    if (url.pathname === '/api/vaults/vault-1/notes') {
      await route.fulfill({ json: [] })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/')

  await expect(page.getByText('Your Vaults')).toBeVisible()
  await expect(page.getByLabel('Open Writing')).toBeVisible()

  await page.getByLabel('Open Writing').click()

  await expect(page).toHaveURL(/\/vaults\/vault-1$/)
})

import { expect, test } from '@playwright/test'

test('inline create-vault card keeps form behavior inside the shared card family', async ({ page }) => {
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

    if (url.pathname === '/api/vaults' && route.request().method() === 'GET') {
      await route.fulfill({
        json: [
          { id: 'vault-1', slug: 'writing', name: 'Writing', owner: { type: 'user', id: 'user-1' }, permission: 'edit', icon: 'folder' },
          { id: 'vault-2', slug: 'archive', name: 'Archive', owner: { type: 'user', id: 'user-1' }, permission: 'edit', icon: 'folder' },
        ],
      })
      return
    }

    if (url.pathname === '/api/vaults' && route.request().method() === 'POST') {
      await route.fulfill({
        json: { id: 'vault-3', slug: 'ideas', name: 'Ideas', owner: { type: 'user', id: 'user-1' }, permission: 'edit', icon: 'folder' },
      })
      return
    }

    if (url.pathname === '/api/vaults/vault-3/notes') {
      await route.fulfill({ json: [] })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/')
  await page.getByRole('button', { name: 'New Vault' }).click()

  await expect(page.getByText('Choose a name, slug, description, and icon.')).toBeVisible()

  await page.getByLabel('Vault name').fill('Ideas')
  await page.getByLabel('Slug').fill('ideas')
  await page.getByRole('button', { name: 'Create' }).click()

  await expect(page).toHaveURL(/\/vaults\/vault-3$/)
})

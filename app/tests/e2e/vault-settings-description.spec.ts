import { expect, test } from '@playwright/test'

test('vault settings save and persist description updates', async ({ page }) => {
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
          {
            id: 'vault-1',
            slug: 'writing',
            name: 'Writing',
            description: 'Original description.',
            owner: { type: 'user', id: 'user-1' },
            permission: 'edit',
            icon: 'folder',
          },
        ],
      })
      return
    }

    if (url.pathname === '/api/vaults/vault-1' && route.request().method() === 'PATCH') {
      const body = route.request().postDataJSON()

      await route.fulfill({
        json: {
          id: 'vault-1',
          slug: 'writing',
          name: body.name,
          description: body.description,
          owner: { type: 'user', id: 'user-1' },
          permission: 'edit',
          icon: 'folder',
        },
      })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/vaults/vault-1/settings')

  const description = page.getByLabel('Description')
  await expect(description).toHaveValue('Original description.')

  await description.fill('Updated description.')
  await page.getByRole('button', { name: 'Save details' }).click()

  await expect(description).toHaveValue('Updated description.')
  await expect(page.getByText('Saved')).toBeVisible()
})

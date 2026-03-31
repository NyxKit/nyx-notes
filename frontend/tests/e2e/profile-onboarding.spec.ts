import { expect, test } from '@playwright/test'

test('first-run local setup creates a local workspace profile', async ({ page }) => {
  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

    if (url.pathname === '/api/vaults') {
      await route.fulfill({ json: [] })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/login')
  await page.getByRole('button', { name: 'Use Local Workspace' }).click()

  await expect(page).toHaveURL('/')
})

test('first-run server connection adds and signs into a remote profile', async ({ page }) => {
  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

    if (url.pathname === '/api/auth/mode') {
      await route.fulfill({ json: { mode: 'secret_key' } })
      return
    }

    if (url.pathname === '/api/auth/login') {
      await route.fulfill({ json: { token: 'remote-token', expires_in: 3600 } })
      return
    }

    if (url.pathname === '/api/vaults') {
      await route.fulfill({ json: [] })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/login')
  await page.getByLabel('Profile Name').fill('Work NAS')
  await page.getByLabel('Server URL').fill('https://notes.example.com')
  await page.getByLabel('Username').fill('alice')
  await page.getByLabel('Password').fill('top-secret')
  await page.getByRole('button', { name: 'Connect Server' }).click()

  await expect(page).toHaveURL('/')
})

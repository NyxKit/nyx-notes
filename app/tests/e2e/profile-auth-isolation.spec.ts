import { expect, test } from '@playwright/test'

test('signing out one remote profile keeps another saved remote profile usable', async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem('nyx_workspace_profiles', JSON.stringify({
      active_profile_id: 'remote-a',
      profiles: [
        {
          id: 'remote-a',
          type: 'remote',
          display_name: 'Server A',
          server_url: 'https://server-a.example.com',
          username: 'alice',
          connection_status: 'reachable',
          auth_mode: 'secret_key',
        },
        {
          id: 'remote-b',
          type: 'remote',
          display_name: 'Server B',
          server_url: 'https://server-b.example.com',
          username: 'bob',
          connection_status: 'reachable',
          auth_mode: 'secret_key',
        },
      ],
    }))
    localStorage.setItem('nyx_profile_sessions', JSON.stringify({
      'remote-a': {
        profile_id: 'remote-a',
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: 'token-a',
      },
      'remote-b': {
        profile_id: 'remote-b',
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: 'token-b',
      },
    }))
  })

  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

    if (url.pathname === '/api/auth/mode') {
      await route.fulfill({ json: { mode: 'secret_key' } })
      return
    }

    if (url.pathname === '/api/vaults') {
      if (url.origin === 'https://server-b.example.com') {
        await route.fulfill({ json: [{ id: 'vault-b', slug: 'team-b', name: 'Server B Vault', owner: { type: 'user', id: 'bob' }, permission: 'edit' }] })
        return
      }

      await route.fulfill({ json: [{ id: 'vault-a', slug: 'team-a', name: 'Server A Vault', owner: { type: 'user', id: 'alice' }, permission: 'edit' }] })
      return
    }

    if (url.pathname === '/api/vaults/vault-a/notes' || url.pathname === '/api/vaults/vault-b/notes') {
      await route.fulfill({ json: [] })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/')
  await expect(page.getByText('Server A Vault')).toBeVisible()

  await page.getByRole('button', { name: 'Sign Out' }).click()
  await expect(page).toHaveURL(/\/login$/)

  const remainingSession = await page.evaluate(() => {
    const raw = localStorage.getItem('nyx_profile_sessions')
    return raw ? JSON.parse(raw)['remote-b']?.token : null
  })

  expect(remainingSession).toBe('token-b')

  await page.evaluate(() => {
    const raw = localStorage.getItem('nyx_workspace_profiles')
    if (!raw) return
    const parsed = JSON.parse(raw)
    parsed.active_profile_id = 'remote-b'
    localStorage.setItem('nyx_workspace_profiles', JSON.stringify(parsed))
  })

  await page.goto('/')
  await expect(page.getByText('Server B Vault')).toBeVisible()
})

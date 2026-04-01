import { expect, test } from '@playwright/test'

test('switches between local and remote profiles without mixing workspaces', async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem('nyx_workspace_profiles', JSON.stringify({
      active_profile_id: 'local',
      profiles: [
        { id: 'local', type: 'local', display_name: 'Local' },
        {
          id: 'remote-1',
          type: 'remote',
          display_name: 'Work NAS',
          server_url: 'https://notes.example.com',
          username: 'alice',
          connection_status: 'reachable',
          auth_mode: 'secret_key',
        },
      ],
    }))
    localStorage.setItem('nyx_profile_sessions', JSON.stringify({
      'remote-1': {
        profile_id: 'remote-1',
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: 'remote-token',
      },
    }))
  })

  await page.route('**/api/**', async route => {
    const url = new URL(route.request().url())

    if (url.pathname === '/api/vaults') {
      if (url.origin === 'https://notes.example.com') {
        await route.fulfill({ json: [{ id: 'remote-vault', slug: 'remote', name: 'Remote Vault', owner: { type: 'user', id: 'user-1' }, permission: 'edit' }] })
        return
      }

      await route.fulfill({ json: [{ id: 'local-vault', slug: 'local', name: 'Local Vault', owner: { type: 'user', id: 'local' }, permission: 'edit' }] })
      return
    }

    if (url.pathname === '/api/vaults/local-vault/notes') {
      await route.fulfill({ json: [] })
      return
    }

    if (url.pathname === '/api/vaults/remote-vault/notes') {
      await route.fulfill({ json: [] })
      return
    }

    if (url.pathname === '/api/auth/mode') {
      await route.fulfill({ json: { mode: 'secret_key' } })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/')
  await page.getByRole('button', { name: 'Work NAS' }).click()
  await expect(page.getByText('Remote Vault')).toBeVisible()

  await page.getByRole('button', { name: 'Local' }).click()
  await expect(page.getByText('Local Vault')).toBeVisible()
})

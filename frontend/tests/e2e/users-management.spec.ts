import { expect, test } from '@playwright/test'

test('secret_key admin can manage users and cannot delete self', async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem('nyx_workspace_profiles', JSON.stringify({
      active_profile_id: 'remote-admin',
      profiles: [
        {
          id: 'remote-admin',
          type: 'remote',
          display_name: 'Main Server',
          server_url: 'https://notes.example.com',
          username: 'admin',
          connection_status: 'reachable',
          auth_mode: 'secret_key',
        },
      ],
    }))
    localStorage.setItem('nyx_profile_sessions', JSON.stringify({
      'remote-admin': {
        profile_id: 'remote-admin',
        state: 'signed_in',
        auth_mode: 'secret_key',
        token: 'admin-token',
      },
    }))
  })

  const users = [
    {
      id: 'admin-1',
      username: 'admin',
      email: 'admin@localhost',
      display_name: 'Admin',
      role: 'admin',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      can_edit: true,
      can_delete: false,
    },
    {
      id: 'user-1',
      username: 'alice',
      email: 'alice@example.com',
      display_name: 'Alice',
      role: 'user',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      can_edit: true,
      can_delete: true,
    },
  ]

  await page.route('https://notes.example.com/api/**', async route => {
    const url = new URL(route.request().url())

    if (url.pathname === '/api/auth/mode') {
      await route.fulfill({ json: { mode: 'secret_key', server_name: 'Main Server' } })
      return
    }

    if (url.pathname === '/api/server') {
      await route.fulfill({
        json: {
          id: 'server-main-server',
          slug: 'main-server',
          name: 'Main Server',
          role: 'admin',
          current_user_id: 'admin-1',
        },
      })
      return
    }

    if (url.pathname === '/api/vaults') {
      await route.fulfill({ json: [] })
      return
    }

    if (url.pathname === '/api/users' && route.request().method() === 'GET') {
      await route.fulfill({ json: users })
      return
    }

    if (url.pathname === '/api/users' && route.request().method() === 'POST') {
      const body = route.request().postDataJSON() as Record<string, string>
      const created = {
        id: 'user-2',
        username: body.username,
        email: body.email,
        display_name: body.display_name,
        role: body.role,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        can_edit: true,
        can_delete: true,
      }
      users.push(created)
      await route.fulfill({ status: 201, json: created })
      return
    }

    if (url.pathname === '/api/users/user-1' && route.request().method() === 'PATCH') {
      const body = route.request().postDataJSON() as Record<string, string>
      const existing = users.find(user => user.id === 'user-1')!
      existing.email = body.email
      existing.display_name = body.display_name
      existing.role = body.role
      existing.updated_at = new Date().toISOString()
      await route.fulfill({ json: existing })
      return
    }

    if (url.pathname === '/api/users/admin-1' && route.request().method() === 'DELETE') {
      await route.fulfill({ status: 409, json: { error: 'administrators cannot delete themselves' } })
      return
    }

    if (url.pathname === '/api/users/user-1' && route.request().method() === 'DELETE') {
      const index = users.findIndex(user => user.id === 'user-1')
      users.splice(index, 1)
      await route.fulfill({ status: 204, body: '' })
      return
    }

    await route.fulfill({ json: {} })
  })

  await page.goto('/users')
  await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible()
  await expect(page.getByText('alice@example.com')).toBeVisible()

  await page.getByRole('button', { name: 'Add user' }).click()
  await page.getByLabel('Username').fill('bob')
  await page.getByLabel('Email').fill('bob@example.com')
  await page.getByLabel('Display Name').fill('Bob')
  await page.getByLabel('Password').fill('Correct-password1')
  await page.getByRole('button', { name: 'Create' }).click()

  await expect(page.getByText('bob@example.com')).toBeVisible()

  await page.getByRole('button', { name: 'Edit' }).nth(1).click()
  await page.getByLabel('Email').fill('alice-updated@example.com')
  await page.getByLabel('Display Name').fill('Alice Updated')
  await page.getByRole('button', { name: 'Save' }).click()

  await expect(page.getByText('alice-updated@example.com')).toBeVisible()

  await expect(page.getByRole('button', { name: 'Delete' }).first()).toBeDisabled()

  page.once('dialog', dialog => dialog.accept())
  await page.getByRole('button', { name: 'Delete' }).nth(1).click()
  await expect(page.getByText('alice-updated@example.com')).not.toBeVisible()
})

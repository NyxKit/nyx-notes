import { expect, test } from '@playwright/test'

for (const scenario of [
  { name: 'author of a restricted note', authorId: 'user-123', permission: 'restricted', editable: true },
  { name: 'non-author with comment access', authorId: 'another-user', permission: 'comment', editable: false },
  { name: 'non-author with edit access', authorId: 'another-user', permission: 'edit', editable: true },
]) {
  test(`${scenario.name} gets the correct editor state`, async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.setItem('nyx_workspace_profiles', JSON.stringify({
        active_profile_id: 'local-profile',
        profiles: [{ id: 'local-profile', type: 'local', display_name: 'Nyx Server' }],
      }))
      localStorage.setItem('nyx_profile_sessions', JSON.stringify({
        'local-profile': {
          profile_id: 'local-profile', state: 'signed_in',
          auth_mode: 'secret_key', token: 'test-token', username: 'arnedecant',
        },
      }))
    })

    const note = {
      meta: {
        id: 'note-1', vault_id: 'home', title: 'My note', author_id: scenario.authorId,
        tags: [], category: null, created_at: '2026-09-15T09:00:00Z',
        updated_at: '2026-09-15T09:00:00Z', is_encrypted: false, permission: scenario.permission,
      },
      content: 'Opening paragraph.\n\nAnother paragraph.\n\n## Heading\n\nExisting text',
    }
    const saves: Array<{ content: string }> = []
    let finishFirstSave: (() => void) | undefined
    await page.route('**/api/**', async route => {
      const path = new URL(route.request().url()).pathname
      if (!path.startsWith('/api/')) {
        await route.continue()
      } else if (path === '/api/auth/mode') {
        await route.fulfill({ json: { mode: 'secret_key' } })
      } else if (path === '/api/auth/initialized') {
        await route.fulfill({ json: { initialized: true } })
      } else if (path === '/api/server') {
        await route.fulfill({ json: {
          id: 'server-nyx-server', slug: 'nyx-server', name: 'Nyx Server',
          current_user_id: 'user-123', current_user_username: 'arnedecant', role: 'user',
        } })
      } else if (path === '/api/vaults') {
        await route.fulfill({ json: [{
          id: 'home', slug: 'home', name: 'Home', permission: 'restricted',
          owner: { type: 'home', server_slug: 'nyx-server', home_slug: 'arnedecant' },
        }] })
      } else if (path === '/api/vaults/home/notes') {
        await route.fulfill({ json: [note.meta] })
      } else if (path === '/api/vaults/home/notes/note-1') {
        if (route.request().method() === 'PUT') {
          const update = route.request().postDataJSON()
          saves.push(update)
          note.content = update.content
          note.meta.title = update.title
          if (saves.length === 1) {
            await new Promise<void>(resolve => { finishFirstSave = resolve })
          }
          await route.fulfill({ json: note.meta })
          return
        }
        await route.fulfill({ json: note })
      } else if (path.endsWith('/comments')) {
        await route.fulfill({ json: [] })
      } else {
        await route.fulfill({ status: 404, json: { error: `Unexpected API path: ${path}` } })
      }
    })

    await page.goto('/nyx-server/homes/arnedecant/home/note-1')
    const editor = page.locator('.tiptap')
    await expect(editor).toHaveAttribute('contenteditable', String(scenario.editable))
    await expect(page).toHaveTitle('My note | Nyx Notes')
    if (scenario.editable) {
      await editor.click()
      await page.keyboard.press('ControlOrMeta+End')
      await page.keyboard.type(' Typed text')
      const editorBeforeSave = await editor.elementHandle()
      const positionBeforeSave = await editor.boundingBox()
      const caretBeforeSave = await editor.evaluate(() => window.getSelection()?.anchorOffset)
      // Capture intermediate frames too: stable before/after boxes can miss a flash.
      const frameSamples = await editor.evaluateHandle(el => {
        const range = document.createRange()
        range.selectNodeContents(el.querySelector('p')!)
        const state = { running: true, positions: [] as number[], frame: 0 }
        const sample = () => {
          state.positions.push(range.getBoundingClientRect().y)
          if (state.running) state.frame = requestAnimationFrame(sample)
        }
        sample()
        return state
      })
      await expect.poll(() => saves.at(-1)?.content).toContain('Existing text Typed text')
      try {
        await expect(page.getByText('Saving…', { exact: true })).toBeVisible()
        expect(await editor.boundingBox()).toEqual(positionBeforeSave)
      } finally {
        finishFirstSave?.()
      }
      await expect(page.getByText('Saving…', { exact: true })).toBeHidden()
      expect(await editor.boundingBox()).toEqual(positionBeforeSave)
      expect(await editorBeforeSave!.evaluate(el => el.isConnected)).toBe(true)
      await expect(editor).toBeFocused()
      expect(await editor.evaluate(() => window.getSelection()?.anchorOffset)).toBe(caretBeforeSave)
      // Include two rendered frames after the response updates the store.
      await page.evaluate(() => new Promise<void>(resolve => {
        requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
      }))
      const positions = await frameSamples.evaluate(state => {
        state.running = false
        cancelAnimationFrame(state.frame)
        return state.positions
      })
      expect(positions.length).toBeGreaterThan(2)
      expect(Math.max(...positions) - Math.min(...positions)).toBeLessThan(0.5)
      await frameSamples.dispose()
      await page.reload()
      await expect(page.locator('.tiptap')).toContainText('Existing text Typed text')
      await page.getByPlaceholder('Untitled').fill('Renamed note')
      await expect(page).toHaveTitle('Renamed note | Nyx Notes')
      await page.getByPlaceholder('Untitled').fill('')
      await expect(page).toHaveTitle('Untitled | Nyx Notes')
    } else {
      await editor.click()
      await page.keyboard.type('Must not be saved')
      await expect(editor).toContainText('Existing text')
      await expect(editor).not.toContainText('Must not be saved')
      expect(saves).toHaveLength(0)
    }
    await page.getByRole('link', { name: 'Settings', exact: true }).click()
    await expect(page).toHaveTitle('Settings | Nyx Notes')
  })
}

import { expect, test, type Page } from '@playwright/test'

async function expectPageFits(page: Page) {
  const body = page.locator('.app-shell__body')
  await expect(body).toBeVisible()
  const bounds = await body.boundingBox()
  const viewport = page.viewportSize()!
  expect(bounds!.y + bounds!.height).toBeLessThanOrEqual(viewport.height + 1)
  expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(viewport.width + 1)
  const documentSize = await page.evaluate(() => ({
    width: document.documentElement.scrollWidth,
    height: document.documentElement.scrollHeight,
  }))
  expect(documentSize.width).toBeLessThanOrEqual(viewport.width)
  expect(documentSize.height).toBeLessThanOrEqual(viewport.height)
}

async function expectDangerZoneReachable(page: Page) {
  const danger = page.locator('.settings-section--danger')
  await expect(danger).toBeVisible()
  await expectPageFits(page)
  const headerBefore = await page.locator('.app-shell__header').boundingBox()
  await page.locator('.app-shell__body').evaluate(el => { el.scrollTop = el.scrollHeight })
  const bounds = await danger.boundingBox()
  const bottomPadding = await page.locator('.app-shell__body').evaluate(el =>
    Number.parseFloat(getComputedStyle(el).paddingBottom)
  )
  expect(bounds!.y + bounds!.height + bottomPadding).toBeLessThanOrEqual(page.viewportSize()!.height + 1)
  expect(await page.locator('.app-shell__header').boundingBox()).toEqual(headerBefore)
}

for (const viewport of [{ width: 1280, height: 720 }, { width: 800, height: 600 }]) {
  test(`pages fit the viewport after navigation and reload at ${viewport.width}x${viewport.height}`, async ({ page }) => {
    await page.setViewportSize(viewport)
    await page.route('**/api/**', async route => {
      const path = new URL(route.request().url()).pathname
      if (!path.startsWith('/api/')) return route.continue()

      const vault = {
        id: 'home', slug: 'home', name: 'Home', permission: 'restricted',
        owner: { type: 'home', server_slug: 'nyx-server', home_slug: 'arnedecant' },
      }
      const json = path === '/api/auth/mode' ? { mode: 'local' }
        : path === '/api/server' ? {
          id: 'nyx-server', slug: 'nyx-server', name: 'Nyx Server',
          current_user_id: 'arnedecant', current_user_username: 'arnedecant', role: 'admin',
        }
          : path === '/api/vaults' || path === '/api/vaults/personal' ? [vault] : []
      await route.fulfill({ json })
    })

    // Lazy-loaded Settings CSS used to override the shell only after navigation.
    await page.goto('/')
    await expectPageFits(page)
    await expect(page).toHaveTitle('Nyx Notes')
    await page.getByRole('link', { name: 'Settings', exact: true }).click()
    await expectDangerZoneReachable(page)
    await page.reload()
    await expectDangerZoneReachable(page)

    await page.goto('/nyx-server/homes/arnedecant/home')
    await expect(page).toHaveTitle('Home | Nyx Notes')
    await page.goto('/nyx-server/homes/arnedecant/home/settings')
    await expectDangerZoneReachable(page)
    await expect(page).toHaveTitle('Vault Settings | Nyx Notes')
    await page.getByRole('link', { name: 'Settings', exact: true }).click()
    await expectDangerZoneReachable(page)
    await page.goBack()
    await expect(page.getByRole('heading', { name: 'Vault Settings' })).toBeVisible()
    await expectDangerZoneReachable(page)
  })
}

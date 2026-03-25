import { ofetch } from 'ofetch'

// Token is set by useAuth after login / mode discovery.
// Using a module-level ref keeps it out of the Vue reactivity system —
// we only need it as a plain string for headers.
let _token: string | null = null

export function setApiToken(token: string | null) {
  _token = token
}

export const api = ofetch.create({
  baseURL: import.meta.env.VITE_API_BASE_URL ?? '/',
  onRequest({ options }) {
    if (_token) {
      const headers = new Headers(options.headers as HeadersInit | undefined)
      headers.set('Authorization', `Bearer ${_token}`)
      options.headers = headers
    }
  },
})

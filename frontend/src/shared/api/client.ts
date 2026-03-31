import { ofetch } from 'ofetch'

let _token: string | null = null
let _baseUrl = import.meta.env.VITE_API_BASE_URL ?? '/'
let _requestEpoch = 0
let _requestController = new AbortController()

export function setApiToken(token: string | null) {
  _token = token
}

export function setApiBaseUrl(baseUrl: string | null) {
  _baseUrl = baseUrl || import.meta.env.VITE_API_BASE_URL || '/'
}

export function resetApiClientContext() {
  _requestController.abort()
  _requestController = new AbortController()
  _requestEpoch += 1
}

export function setApiProfileContext(options: { baseUrl?: string | null; token?: string | null }) {
  setApiBaseUrl(options.baseUrl ?? null)
  setApiToken(options.token ?? null)
}

export function getApiRequestEpoch() {
  return _requestEpoch
}

export const api = ofetch.create({
  onRequest({ options }) {
    options.baseURL = options.baseURL ?? _baseUrl

    if (_token) {
      const headers = new Headers(options.headers as HeadersInit | undefined)
      headers.set('Authorization', `Bearer ${_token}`)
      options.headers = headers
    }

    if (!options.signal) {
      options.signal = _requestController.signal
    }
  },
})

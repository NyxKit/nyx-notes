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

export function getApiToken() {
  return _token
}

export function getApiBaseUrl() {
  return _baseUrl
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
  onResponseError({ response }) {
    let message = 'Something went wrong'
    
    if (response._data?.error) {
      const rawError = response._data.error
      
      if (response.status === 422) {
        message = rawError
      } else if (response.status === 401) {
        message = 'Invalid credentials'
      } else if (response.status === 403) {
        message = 'Access denied'
      } else if (response.status === 404) {
        message = 'Not found'
      } else if (response.status === 409) {
        message = rawError
      } else if (response.status >= 500) {
        message = 'Server error'
      } else {
        message = rawError
      }
    }
    
    const error = new Error(message)
    ;(error as any).status = response.status
    throw error
  },
})

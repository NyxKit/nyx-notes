import { MergeOption, NyxGateDomain, type NyxGateConfig } from '../types'

export default class NyxGate {
  private baseUrl: string
  private token: string | null
  private headers: Record<string, string>
  private subscriptions = new Map<string, (data: unknown) => void>()

  constructor(config: NyxGateConfig = {}) {
    this.baseUrl = config.baseUrl ?? ''
    this.token = config.token ?? null
    this.headers = { ...(config.headers ?? {}) }
  }

  setup(config: NyxGateConfig = {}) {
    if (config.baseUrl !== undefined) this.baseUrl = config.baseUrl
    if (config.token !== undefined) this.token = config.token
    if (config.headers !== undefined) this.headers = { ...config.headers }

    return {
      baseUrl: this.baseUrl,
      token: this.token,
      headers: { ...this.headers },
    }
  }

  async get(domain: NyxGateDomain, id: string) {
    void domain
    void id
    return null
  }

  async update(domain: NyxGateDomain, id: string, data: unknown, mergeOption: MergeOption = MergeOption.Merge) {
    void domain
    void id
    void data
    void mergeOption
    return null
  }

  async delete(domain: NyxGateDomain, id: string) {
    void domain
    void id
    return null
  }

  subscribe(domain: NyxGateDomain, id: string, onChange: (data: unknown) => void) {
    const key = `${domain}:${id}`
    this.subscriptions.set(key, onChange)
    return {
      key,
      release: () => this.unsubscribe(domain, id),
    }
  }

  unsubscribe(domain: NyxGateDomain, id: string) {
    const key = `${domain}:${id}`
    this.subscriptions.delete(key)
  }
}

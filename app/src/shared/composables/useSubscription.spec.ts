import { ref, nextTick } from 'vue'
import { mount } from '@vue/test-utils'
import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest'
import { useSubscription } from './useSubscription'

describe('useSubscription', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('calls attach on mount with initial source value', () => {
    const source = ref('initial')
    const attach = vi.fn().mockReturnValue(null)

    const wrapper = mount({
      setup() {
        useSubscription(source, attach)
        return () => null
      },
    })

    expect(attach).toHaveBeenCalledWith('initial')
    wrapper.unmount()
  })

  it('releases previous handle when source changes', async () => {
    const source = ref('first')
    const release1 = vi.fn()
    const attach1 = vi.fn().mockReturnValue({ release: release1 })
    const attach2 = vi.fn().mockReturnValue(null)

    const wrapper = mount({
      setup() {
        useSubscription(source, (value) => {
          if (value === 'first') return attach1(value)
          return attach2(value)
        })
        return () => null
      },
    })

    source.value = 'second'
    await nextTick()
    vi.runAllTimers()
    await nextTick()

    expect(release1).toHaveBeenCalled()
    wrapper.unmount()
  })

  it('releases handle on unmount', () => {
    const source = ref('value')
    const release = vi.fn()
    const attach = vi.fn().mockReturnValue({ release })

    const wrapper = mount({
      setup() {
        useSubscription(source, attach)
        return () => null
      },
    })

    wrapper.unmount()

    expect(release).toHaveBeenCalled()
  })

  it('returns null from attach to skip subscription', () => {
    const source = ref('test')
    const attach = vi.fn().mockReturnValue(null)

    const wrapper = mount({
      setup() {
        useSubscription(source, attach)
        return () => null
      },
    })

    expect(attach).toHaveBeenCalledWith('test')
    wrapper.unmount()
  })
})
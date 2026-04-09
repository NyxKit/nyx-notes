import { onBeforeUnmount, watch, type WatchSource } from 'vue'

type ReleaseHandle = { release: () => void }

export function useSubscription<T>(source: WatchSource<T>, attach: (value: T) => ReleaseHandle | null) {
  let handle: ReleaseHandle | null = null

  const stop = watch(source, value => {
    handle?.release()
    handle = attach(value)
  }, { immediate: true })

  onBeforeUnmount(() => {
    stop()
    handle?.release()
    handle = null
  })
}

const MAX_LINES = 80

const buffer: string[] = []
let installed = false

function formatValue(value: unknown): string {
  if (typeof value === 'string') return value
  if (value instanceof Error) return `${value.name}: ${value.message}`
  try {
    return JSON.stringify(value)
  } catch {
    return String(value)
  }
}

function pushEntry(method: string, args: unknown[]) {
  buffer.push(`[${method}] ${args.map(formatValue).join(' ')}`)
  if (buffer.length > MAX_LINES) {
    buffer.splice(0, buffer.length - MAX_LINES)
  }
}

export function installConsoleCapture() {
  if (installed || typeof console === 'undefined') return
  installed = true

  for (const method of ['warn', 'error'] as const) {
    const original = console[method].bind(console)
    console[method] = (...args: unknown[]) => {
      pushEntry(method, args)
      original(...args)
    }
  }
}

export function getConsoleOutputSnapshot() {
  return buffer.join('\n')
}

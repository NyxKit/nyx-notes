import nyxConfig from 'nyx-kit/eslint'

export default [
  {
    ignores: [
      'dist/**',
      'coverage/**',
      'node_modules/**',
      'playwright-report/**',
      'test-results/**',
    ],
  },
  ...nyxConfig,
  {
    files: ['**/*.{ts,vue}'],
    rules: {
      'semi': ['error', 'never'],
      'quotes': ['error', 'single', { avoidEscape: true }],
    },
  },
]

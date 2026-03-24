import nyxConfig from 'nyx-kit/eslint'

export default [
  ...nyxConfig,
  {
    files: ['**/*.{ts,vue}'],
    rules: {
      'semi': ['error', 'never'],
      'quotes': ['error', 'single', { avoidEscape: true }],
    },
  },
]

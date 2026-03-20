import { defineStore } from 'pinia'
import { ref } from 'vue'
import { NyxEditorMode } from 'nyx-kit/types'

export const useEditorStore = defineStore('editor', () => {
  const mode = ref<NyxEditorMode>(NyxEditorMode.Zen)
  const isSourceView = ref(false)

  function toggleSourceView() {
    isSourceView.value = !isSourceView.value
  }

  function reset() {
    isSourceView.value = false
    mode.value = NyxEditorMode.Zen
  }

  return { mode, isSourceView, toggleSourceView, reset }
})

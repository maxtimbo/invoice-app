import { ref } from 'vue'
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Item } from '../types'

export const useItemStore = defineStore('items', () => {
  const items = ref<Item[]>([])
  const error = ref<string | null>(null)
  const loading = ref(false)

  async function load() {
    loading.value = true
    error.value = null
    try {
      items.value = await invoke<Item[]>('list_items')
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }
  return { items, error, loading, load }
})

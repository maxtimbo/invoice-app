<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Item {
  id: number
  name: string
  rate: string
}

const items = ref<Item[]>([])
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    items.value = await invoke<Item[]>('list_items')
  } catch (e) {
    error.value = String(e)
  }
})
</script>

<template>
  <div>
    <h1>Items</h1>
    <p v-if="error" style="color:red">{{ error }}</p>
    <table>
      <tr>
        <th>Item</th><th>Rate</th>
      </tr>
      <tr v-for="item in items" :key="item.id">
        <td>{{ item.name }}</td><td>{{ item.rate }}</td>
      </tr>
    </table>
  </div>
</template>

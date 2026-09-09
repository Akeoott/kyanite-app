<script setup lang="ts">
import { useTelemetry } from '@/composables/useTelemetry';
import { onMounted } from 'vue';

const {
  cpuData,
  memoryData,
  gpuData,
  driveData,
  processData,
  systemData,
  networkData,
  loading,
  error,
  fetchTel,
} = useTelemetry()

onMounted(() => {
  fetchTel()
})
</script>

<template>
  <div class="p-6 max-w-4xl mx-auto space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-3xl font-bold">Telemetry Raw Data</h1>
      <button @click="fetchTel" :disabled="loading" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50">
        {{ loading ? 'Refreshing...' : 'Refresh Data' }}
      </button>
    </div>

    <div v-if="error" class="p-4 bg-red-100 border border-red-400 text-red-700 rounded">
      {{ error }}
    </div>

    <div v-if="loading" class="text-gray-500">
      Loading telemetry data...
    </div>

    <div v-else class="space-y-4">
      <section>
        <h2 class="text-xl font-semibold mb-2">CPU</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(cpuData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">Memory</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(memoryData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">GPU</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(gpuData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">Drive</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(driveData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">System</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(systemData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">Network</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(networkData, null, 2) }}</pre>
      </section>

      <section>
        <h2 class="text-xl font-semibold mb-2">Process</h2>
        <pre class="bg-gray-900 text-green-400 p-4 rounded overflow-x-auto text-sm">{{ JSON.stringify(processData, null, 2) }}</pre>
      </section>

    </div>
  </div>
</template>

<style scoped></style>

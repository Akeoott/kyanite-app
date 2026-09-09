import { invoke } from '@tauri-apps/api/core'
import { ref, type Ref } from 'vue'

export type TelemetryData = Record<string, unknown> | null

export function useTelemetry() {
  const cpuData: Ref<TelemetryData> = ref(null)
  const memoryData: Ref<TelemetryData> = ref(null)
  const gpuData: Ref<TelemetryData> = ref(null)
  const driveData: Ref<TelemetryData> = ref(null)
  const processData: Ref<TelemetryData> = ref(null)
  const systemData: Ref<TelemetryData> = ref(null)
  const networkData: Ref<TelemetryData> = ref(null)

  const loading = ref<boolean>(false)
  const error = ref<string | null>(null)

  async function updateTel(): Promise<void> {
    try {
      await invoke('update_all')
    } catch (err) {
      error.value = `Failed to update telemetry: ${err}`
    }
  }

  async function fetchTel(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await updateTel()

      const [cpu, mem, gpu, drive, proc, sys, net] = await Promise.all([
        invoke<Record<string, unknown>>('cpu_info'),
        invoke<Record<string, unknown>>('memory_info'),
        invoke<Record<string, unknown>>('gpu_info'),
        invoke<Record<string, unknown>>('drive_info'),
        invoke<Record<string, unknown>>('process_info'),
        invoke<Record<string, unknown>>('system_info'),
        invoke<Record<string, unknown>>('network_info'),
      ])

      cpuData.value = cpu
      memoryData.value = mem
      gpuData.value = gpu
      driveData.value = drive
      processData.value = proc
      systemData.value = sys
      networkData.value = net
    } catch (err) {
      error.value = String(err)
    } finally {
      loading.value = false
    }
  }

  return {
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
  }
}

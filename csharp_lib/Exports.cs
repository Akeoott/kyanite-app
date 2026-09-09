// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;

using Microsoft.Extensions.DependencyInjection;

using ZenMonitor.Core.Abstractions;
using ZenMonitor.Core.Hosting;
using ZenMonitor.Core.Models.Telemetry;

[JsonSerializable(typeof(CpuInfoSnapshot))]
[JsonSerializable(typeof(MemoryInfoSnapshot))]
[JsonSerializable(typeof(GpuInfoSnapshot))]
[JsonSerializable(typeof(DriveInfoSnapshot))]
[JsonSerializable(typeof(ProcessInfoSnapshot))]
[JsonSerializable(typeof(SystemInfoSnapshot))]
[JsonSerializable(typeof(NetworkInfoSnapshot))]
internal partial class TelemetrySnapshotJsonContext : JsonSerializerContext { }

public static class Exports
{
    private static readonly Lazy<ServiceProvider> _serviceProvider = new(static () =>
    {
        var services = new ServiceCollection();
        services.AddZenMonitor();
        return services.BuildServiceProvider();
    });

    private static ITelemetryAggregate Telemetry => _serviceProvider.Value.GetRequiredService<ITelemetryAggregate>();

    // ---- String marshalling helpers ----

    private static IntPtr AllocUtf8(string? str)
    {
        if (string.IsNullOrEmpty(str))
            return IntPtr.Zero;
        return Marshal.StringToCoTaskMemUTF8(str);
    }

    [UnmanagedCallersOnly(EntryPoint = "free_string", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static void FreeString(IntPtr ptr)
    {
        if (ptr != IntPtr.Zero)
            Marshal.FreeCoTaskMem(ptr);
    }

    private static string? PtrToStringUtf8(IntPtr ptr)
        => ptr == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(ptr);

    // ---- Core exports ----

    /// <summary>Updates all telemetry data (calls ITelemetryAggregate.UpdateAll).</summary>
    [UnmanagedCallersOnly(EntryPoint = "update_all", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static void UpdateAll() => Telemetry.UpdateAll();

    /// <summary>Returns a JSON string of the CPU snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "cpu_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr CpuSnapshot()
    {
        var snapshot = Telemetry.CpuTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.CpuInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the memory snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "memory_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr MemorySnapshot()
    {
        var snapshot = Telemetry.MemoryTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.MemoryInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the GPU snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "gpu_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr GpuSnapshot()
    {
        var snapshot = Telemetry.GpuTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.GpuInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the drive snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "drive_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr DriveSnapshot()
    {
        var snapshot = Telemetry.DriveTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.DriveInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the process snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "process_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr ProcessSnapshot()
    {
        var snapshot = Telemetry.ProcessTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.ProcessInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the system snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "system_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr SystemSnapshot()
    {
        var snapshot = Telemetry.SystemTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.SystemInfoSnapshot);
        return AllocUtf8(json);
    }

    /// <summary>Returns a JSON string of the network snapshot.</summary>
    [UnmanagedCallersOnly(EntryPoint = "network_snapshot", CallConvs = new[] { typeof(CallConvCdecl) })]
    public static IntPtr NetworkSnapshot()
    {
        var snapshot = Telemetry.NetworkTel.GetSnapshot();
        string json = JsonSerializer.Serialize(snapshot, TelemetrySnapshotJsonContext.Default.NetworkInfoSnapshot);
        return AllocUtf8(json);
    }
}

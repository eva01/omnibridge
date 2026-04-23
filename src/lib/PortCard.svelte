<script lang="ts">
  import { untrack } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { BAUD_RATES, BAUDS_TO_PROBE, CLASS_COLORS, type BaudProbeResult, type DiscoveredDevice } from "$lib/types.js";
  import type { DeviceProfile } from "$lib/profiles.js";

  let {
    device,
    monitoring = false,
    learnedProfile = null,
    onToggle,
    onBaudChange,
  }: {
    device: DiscoveredDevice;
    monitoring: boolean;
    learnedProfile?: DeviceProfile | null;
    onToggle: () => void;
    onBaudChange: (baud: number) => void;
  } = $props();

  // Initialize from prop but allow user selection to diverge.
  // untrack() prevents Svelte from warning about capture-initial-only semantics.
  let selectedBaud = $state(untrack(() => device.suggested_baud));

  const color = $derived(CLASS_COLORS[device.device_class] ?? "#4b5563");

  // Auto baud probe state
  let probing = $state(false);
  let probeIndex = $state(0);
  let probeCurrent = $state<number | null>(null);
  let probeResults = $state<BaudProbeResult[]>([]);
  let probeError = $state<string | null>(null);
  let probeWinner = $state<BaudProbeResult | null>(null);
  let winnerTimer: ReturnType<typeof setTimeout> | null = null;

  function handleBaudChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    selectedBaud = val;
    onBaudChange(val);
  }

  function shortPort(p: string): string {
    const parts = p.split("/");
    return parts[parts.length - 1];
  }

  async function autoProbe() {
    if (probing || monitoring) return;
    probing = true;
    probeResults = [];
    probeError = null;
    probeWinner = null;
    if (winnerTimer) { clearTimeout(winnerTimer); winnerTimer = null; }

    try {
      for (let i = 0; i < BAUDS_TO_PROBE.length; i++) {
        probeIndex = i + 1;
        probeCurrent = BAUDS_TO_PROBE[i];
        const r = await invoke<BaudProbeResult>("probe_single_baud", {
          port: device.port,
          baud: BAUDS_TO_PROBE[i],
        });
        probeResults = [...probeResults, r];
      }

      // Pick winner: highest score AND at least some bytes read
      const valid = probeResults.filter((r) => r.bytes_read > 0 && !r.error);
      if (valid.length === 0) {
        probeError = "No device response detected — make sure it's transmitting";
        return;
      }
      const best = valid.reduce((a, b) => (b.score > a.score ? b : a));
      probeWinner = best;
      selectedBaud = best.baud;
      onBaudChange(best.baud);

      // Auto-dismiss winner strip after 8 seconds
      winnerTimer = setTimeout(() => { probeWinner = null; }, 8000);
    } catch (e) {
      probeError = String(e);
    } finally {
      probing = false;
      probeCurrent = null;
    }
  }
</script>

<div class="card" class:active={monitoring} style="--accent: {color}">
  <div class="card-header">
    <span class="badge" style="background: {color}22; color: {color}; border-color: {color}44">
      {device.label}
    </span>
    <button
      class="monitor-btn"
      class:running={monitoring}
      onclick={onToggle}
      title={monitoring ? "Stop monitoring" : "Start monitoring"}
    >
      {#if monitoring}
        <span class="dot pulsing"></span> Live
      {:else}
        ▶ Monitor
      {/if}
    </button>
  </div>

  <div class="port-name">{shortPort(device.port)}</div>
  <div class="port-full">{device.port}</div>

  {#if device.manufacturer || device.product}
    <div class="meta">
      {device.manufacturer ?? ""}{device.product ? ` · ${device.product}` : ""}
    </div>
  {/if}

  {#if device.vid !== null && device.pid !== null}
    <div class="ids">
      VID:{device.vid.toString(16).padStart(4, "0").toUpperCase()}
      PID:{device.pid.toString(16).padStart(4, "0").toUpperCase()}
    </div>
  {/if}

  {#if learnedProfile}
    <div class="learned-badge" title="Learned from {learnedProfile.sample_count} prior session{learnedProfile.sample_count > 1 ? 's' : ''}">
      <span class="learned-star">⭐</span>
      <span class="learned-protocol">{learnedProfile.protocol}</span>
      <span class="learned-conf">{learnedProfile.high_water_confidence}%</span>
    </div>
  {/if}

  <div class="baud-row">
    <span class="baud-label">Baud</span>
    <select
      class="baud-select"
      value={selectedBaud}
      onchange={handleBaudChange}
      disabled={monitoring || probing}
    >
      {#each BAUD_RATES as rate}
        <option value={rate}>{rate}</option>
      {/each}
    </select>
    <button
      class="auto-btn"
      class:probing
      onclick={autoProbe}
      disabled={monitoring || probing || device.port.startsWith('demo://')}
      title={
        device.port.startsWith('demo://')
          ? 'Demo device uses fixed baud'
          : monitoring
          ? 'Stop monitoring first'
          : 'Auto-detect baud rate (~6s)'
      }
    >
      {#if probing}
        <span class="spin-s">◌</span>
      {:else}
        🎯
      {/if}
    </button>
  </div>

  {#if probing}
    <div class="probe-strip probing">
      <span class="probe-spin">◌</span>
      <span class="probe-text">Probing {probeIndex}/{BAUDS_TO_PROBE.length} · {probeCurrent}</span>
    </div>
  {:else if probeError}
    <div class="probe-strip error">
      <span>⚠ {probeError}</span>
    </div>
  {:else if probeWinner}
    <div class="probe-strip success">
      <span class="trophy">✓</span>
      <span class="probe-text">
        <strong>{probeWinner.baud}</strong>
        · {Math.round(probeWinner.score * 100)}% printable
        · {probeWinner.bytes_read}B
      </span>
    </div>
  {/if}
</div>

<style>
  .card {
    background: #1a1a28;
    border: 1px solid #252538;
    border-left: 3px solid var(--accent, #4b5563);
    border-radius: 8px;
    padding: 0.85rem;
    cursor: default;
    transition: border-color 0.2s, background 0.2s;
  }

  .card.active {
    background: #1e1e30;
    border-color: var(--accent, #4b5563);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    gap: 0.5rem;
  }

  .badge {
    font-size: 0.7rem;
    font-weight: 600;
    padding: 0.15rem 0.45rem;
    border-radius: 4px;
    border: 1px solid;
    letter-spacing: 0.03em;
    white-space: nowrap;
  }

  .monitor-btn {
    font-size: 0.72rem;
    font-weight: 600;
    padding: 0.2rem 0.6rem;
    border-radius: 5px;
    border: 1px solid #3a3a56;
    background: #252538;
    color: #a0a0c0;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.3rem;
    transition: all 0.15s;
    white-space: nowrap;
  }

  .monitor-btn:hover:not(:disabled) {
    background: #2e2e48;
    color: #e0e0f0;
  }

  .monitor-btn.running {
    background: #1a2e1a;
    border-color: #22c55e44;
    color: #22c55e;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    display: inline-block;
  }

  .pulsing {
    animation: pulse 1.2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }

  .port-name {
    font-family: "JetBrains Mono", "Fira Code", monospace;
    font-size: 0.9rem;
    font-weight: 600;
    color: #c8c8e8;
    margin-bottom: 0.1rem;
  }

  .port-full {
    font-family: monospace;
    font-size: 0.68rem;
    color: #5a5a7a;
    margin-bottom: 0.4rem;
    word-break: break-all;
  }

  .meta {
    font-size: 0.72rem;
    color: #7a7a9a;
    margin-bottom: 0.25rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ids {
    font-family: monospace;
    font-size: 0.68rem;
    color: #4a4a6a;
    margin-bottom: 0.5rem;
    letter-spacing: 0.04em;
  }

  .learned-badge {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-bottom: 0.5rem;
    padding: 0.2rem 0.45rem;
    background: linear-gradient(90deg, #1a1428 0%, #1e1a30 100%);
    border: 1px solid #4a3870;
    border-radius: 4px;
    font-size: 0.7rem;
  }

  .learned-star {
    color: #fbbf24;
    font-size: 0.7rem;
  }

  .learned-protocol {
    color: #d8b4fe;
    font-weight: 600;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .learned-conf {
    color: #a78bfa;
    font-family: "JetBrains Mono", monospace;
    font-size: 0.65rem;
    background: #2a1e42;
    padding: 0.05rem 0.3rem;
    border-radius: 3px;
  }

  .baud-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .baud-label {
    font-size: 0.7rem;
    color: #5a5a7a;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .baud-select {
    flex: 1;
    background: #111120;
    border: 1px solid #2a2a42;
    border-radius: 5px;
    color: #a0a0c0;
    font-size: 0.75rem;
    padding: 0.2rem 0.4rem;
    cursor: pointer;
  }

  .baud-select:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .auto-btn {
    background: #1a1a28;
    border: 1px solid #2a2a42;
    border-radius: 5px;
    color: #a0a0c0;
    cursor: pointer;
    padding: 0.2rem 0.5rem;
    font-size: 0.78rem;
    transition: all 0.15s;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 32px;
  }

  .auto-btn:hover:not(:disabled) {
    background: #2a2a42;
    border-color: #4f6ef7;
    color: #90b0ff;
  }

  .auto-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .auto-btn.probing {
    background: #1e1a30;
    border-color: #4f6ef7;
    color: #90b0ff;
  }

  .spin-s {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  .probe-strip {
    margin-top: 0.35rem;
    padding: 0.3rem 0.5rem;
    border-radius: 5px;
    font-size: 0.7rem;
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-family: "JetBrains Mono", monospace;
  }

  .probe-strip.probing {
    background: #141830;
    border: 1px solid #2a3450;
    color: #7090f0;
  }

  .probe-strip.success {
    background: #0e2018;
    border: 1px solid #1e4028;
    color: #86efac;
    animation: fadeIn 0.3s ease-out;
  }

  .probe-strip.error {
    background: #1e0e0e;
    border: 1px solid #4a1e1e;
    color: #f87171;
    font-family: inherit;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .probe-spin {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  .probe-text strong {
    color: #d0ffd0;
    font-weight: 700;
  }

  .trophy {
    font-size: 0.8rem;
  }
</style>

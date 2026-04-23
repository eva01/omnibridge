# OmniBridge v0.1.0 — First Release

**Intelligent Serial Gateway · AI-powered protocol detection**

Built for the Claude Opus 4.7 Hackathon 2026.

## 🎬 Demo video

[90-second walkthrough](https://... *add link after upload*)

## ✨ What's inside

### AI-powered protocol identification
- **🧠 Protocol Detective** — one-shot analysis identifies protocol + extracts field schema + generates regex for every field
- **🔬 Investigation mode** — multi-step agentic investigation with 5 tools (read more lines, search patterns, probe baud rates, send bytes, device metadata lookup)
- **Opus 4.7 with adaptive thinking** — preserves reasoning across tool use turns

### Live structured dashboards
- Real-time parsed values from the stream — zero API calls after initial identification
- Sparkline charts for numeric fields
- Value flash animations + stale indicators
- Auto-updates via reactive regex parsing

### Device profile learning
- Every successful analysis saved by `VID:PID:SerialNumber`
- Reconnect the same device → instant recognition, dashboard populates without API call
- Confidence score tracked across sessions

### Bidirectional communication
- Send commands to devices with ASCII (`\r \n \xNN` escapes) or HEX input
- Claude-generated presets auto-populate — "Tare" for scales, NMEA rate config, Modbus queries
- TX/RX visual marking in the buffer

### Cloud bridge
- Throttled webhook forwarding with custom headers
- Per-device configuration
- Test-before-deploy workflow
- Structured JSON payloads with timestamps + units

### Export
- CSV/JSON export of raw stream or parsed structured data
- Export includes full schema metadata for audit trail

### Auto baud rate probing
- Click 🎯 on any port → tests 6 common baud rates in ~6 seconds
- Printable ASCII ratio scoring picks the winner

### Demo mode (no hardware needed)
- ⚖️ CAS Weighing Scale — ASCII with stability flag
- 🛰️ GPS Receiver — NMEA 0183 with valid checksums
- 🌡️ Arduino Sensor Array — key=value CSV

## 🧱 Stack

- Tauri 2 + Rust + SvelteKit + Svelte 5 (runes)
- Claude Opus 4.7 via official Anthropic SDK
- `serialport` 4.9 (Rust) + `tauri-plugin-store` + `tauri-plugin-serialplugin`

## 📦 macOS install

The DMG below is **unsigned** (hackathon build, no Apple Developer cert). After downloading:

1. Drag OmniBridge to `/Applications`
2. Right-click → Open → Open (dismisses Gatekeeper once)
3. Or: `xattr -d com.apple.quarantine /Applications/OmniBridge.app`

## ⚙️ First-run setup

1. Open OmniBridge
2. Click ⚙ Settings → paste your [Anthropic API key](https://console.anthropic.com)
3. Click **Test connection** to verify
4. Save → you're ready

No API key? Demo mode still works — you just won't be able to Analyze/Investigate.

## ⚠️ Known limitations

- macOS DMG is unsigned (Gatekeeper warning on first open)
- Windows + Linux builds not tested in this release — should work via `npm run tauri build`
- Binary protocol investigation (Modbus RTU, SLIP) not yet showcased — regex-based parsing works best on ASCII
- Single port at a time for auto-baud probing (sequential by design)

## 🙏 Acknowledgments

- Anthropic — Claude Opus 4.7 and the hackathon platform
- Tauri team — the lightest desktop framework
- Svelte 5 — runes are the real deal

# Demo Drill — End-to-End Verification

**Purpose**: Verify every demo-critical flow **before** you record the video on Saturday. Each step has an expected outcome — if reality doesn't match, file the deviation at the bottom of this doc and ping for a fix. Finding bugs tonight costs 30 minutes; finding them mid-recording costs half a day.

**Time budget**: 15 minutes end-to-end

**Preconditions**:
- `npm run tauri dev` is running
- A valid Anthropic API key is set (either in `.env` for dev, or via the Settings modal)
- Device window is in focus — tests require interaction

---

## Drill 1 — First-run onboarding banner

**Only run this drill if you want to test the judges' first-run experience.** It requires clearing your saved API key.

1. Open Settings (⚙ icon top-left of sidebar)
2. Note your current API key value — copy it to clipboard so you can restore it
3. Click **Clear API key** and confirm
4. Close the modal
5. Observe the hero area

### Expected
- [ ] A **purple pulsing banner** appears between the intro paragraph and CTA buttons
- [ ] Banner text: `🔑 First time here? Add your Anthropic API key to unlock AI protocol detection →`
- [ ] Clicking the banner opens the Settings modal
- [ ] After pasting key + Save + Close, the banner disappears (no reload needed)

If the banner doesn't appear: check that `needsApiKey` state is wired to `refreshApiKeyStatus()` in `+page.svelte` `onMount`.

**Restore**: paste key back and click Save before continuing to other drills.

---

## Drill 2 — CAS scale one-shot analysis (ASCII baseline)

1. Sidebar → **▶ Try Demo Mode** (4 demo devices appear)
2. Click **⚖️ CAS Weighing Scale** card → **▶ Monitor**
3. Wait 3 seconds for data to accumulate (~12 lines)
4. Click the **⬡ Analyze** button in the tab header (NOT ⚡ Live yet)

### Expected
- [ ] Analysis panel shows `CAS Scale · 85-98%` within ~5 seconds
- [ ] `device_hint` mentions weighing scale / CAS
- [ ] Fields table contains at least `weight` (number, kg) and `stable` (string, ST/US)
- [ ] `notes` field non-empty
- [ ] Dashboard tab button becomes enabled (not greyed out)
- [ ] Command presets visible below the analysis (Tare / Zero / Query)

If analysis returns `undefined` or malformed JSON: capture the error from the panel and paste it. Probably a Claude output parsing issue.

---

## Drill 3 — CAS scale Dashboard (ASCII live parsing)

**Prereq**: Drill 2 complete (analysis exists)

1. Click **📊 Dashboard** tab
2. Observe the weight card

### Expected
- [ ] Blue numeric value card showing weight like `1.234 kg`
- [ ] Value flashes when it updates (4 Hz rate)
- [ ] **Sparkline** draws under the value, moving left-to-right
- [ ] `stable` card shows `ST` / `US` text alternating
- [ ] No error messages anywhere
- [ ] Footer shows `Live parsing active · 2 fields tracked · N lines processed`

If sparkline is missing: check that `isNumericType(field.data_type)` returns true for the weight field's data_type (should be `'number'`).

---

## Drill 4 — Modbus agentic investigation ★ CENTERPIECE

This is the video centerpiece. **If anything fails here, fix it before Saturday.**

1. Close CAS tab (✕ on tab)
2. Click **🏭 Modbus RTU PLC** → **▶ Monitor**
3. DataBuffer shows `01 03 14 00 FE 03 F5 ...` streaming in HEX view
4. Toggle ASCII/HEX — ASCII mode should show mostly `·` dots (binary)
5. Wait 5 seconds for ~8-10 frames to accumulate
6. Click **🔬 Investigate** button

### Expected sequence in the Investigation trace
- [ ] **Thinking block** (purple gradient, auto-expanded) appears first — content should mention "binary" or "low ASCII" or "Modbus"
- [ ] **Tool call**: `analyze_binary_structure` with a `hypothesis` parameter
- [ ] **Tool result** showing: `12% ASCII · frame=25B ×N · ✓ Modbus CRC`
- [ ] **Another thinking block** reasoning about CRC match and register layout
- [ ] Optionally more tool calls (`get_device_metadata`, `read_more_lines`)
- [ ] **✨ Final** step with `Modbus RTU · 80-100%`
- [ ] Total steps: 5-10, total API calls: 2-5

### Critical check — schema of the final analysis
1. Click the **✨ Final** step in the trace to expand
2. Look at the JSON

### Expected JSON shape
```json
{
  "protocol": "Modbus RTU",
  "confidence": 85+,
  "fields": [
    {
      "name": "...",
      "regex": "^01 03 14 ...",
      "match_hex": true,          // ← MUST be present and true
      "data_type": "hex_u16_be",  // ← MUST be a hex_* type
      "capture_group": 1,
      ...
    },
    ...
  ]
}
```

**Red flags** (ping for fix):
- ❌ No `match_hex: true` on any field
- ❌ All `data_type` are `"number"` or `"string"` (not `hex_*`)
- ❌ Regex doesn't start with `^01 03 14` or similar frame header
- ❌ Regex contains character classes meant for ASCII like `[\d.]`

---

## Drill 5 — Modbus Dashboard (binary live parsing) ★

**Prereq**: Drill 4 complete with correct schema

1. Dashboard tab button should be enabled (not greyed)
2. Click **📊 Dashboard**

### Expected
- [ ] Multiple **orange** cards (one per field Claude extracted — typically 3-8 cards)
- [ ] Each card has a numeric value (decoded from hex bytes)
- [ ] Sparklines draw under each numeric card
- [ ] Values update at ~1.7 Hz (Modbus demo rate)
- [ ] Type tag shows `hex_u16_be` (or similar) in small text per card
- [ ] Footer: `Live parsing active · N fields tracked · M lines processed`

If cards are empty / show `—`: Claude's regex isn't matching `line.hex`. Check trace's Final JSON — the regex needs to match the actual hex format like `"01 03 14 00 FE 03 F5 ..."` (space-separated UPPERCASE bytes).

If cards show hex strings instead of numbers: `data_type` is probably `"string"` — Claude didn't use `hex_u16_be`. This is a prompt issue; ping for a tune.

---

## Drill 6 — Webhook forwarding

1. Open https://webhook.site in a browser tab, copy the unique URL
2. Back in OmniBridge, on any monitored device's Dashboard:
3. Click **🔗 Webhook** button in the header
4. Paste URL, toggle **Enable**, throttle 1000ms
5. Click **Test**

### Expected
- [ ] Test result: **green ✓ 200 OK** within 2 seconds
- [ ] After Save, webhook counter in header starts incrementing
- [ ] Switch to webhook.site tab → JSON payloads arriving
- [ ] Payload structure: `{ timestamp, port, protocol, confidence, fields: { ... } }`
- [ ] For Modbus: each field's `value` is a **number** (not hex string)

---

## Drill 7 — Device profile learning

1. Stop monitoring (✕ tab) but keep Demo Mode enabled
2. Click **▶ Try Demo Mode** twice (off, then on) to simulate reconnect
3. Observe the ⚖️ CAS Scale card in the sidebar

### Expected
- [ ] Purple **⭐ Auto-recognized · N sessions** badge on the card
- [ ] Click Monitor → buffer opens, then **Dashboard populates immediately** without calling the API
- [ ] No "Thinking…" spinner before dashboard appears (indicator API was skipped)

---

## Drill 8 — Error resilience

1. Open Settings, change API key to obviously wrong value like `sk-ant-api03-FAKEKEY_x`, save
2. Start monitoring a new device (e.g. 🛰️ GPS)
3. Click Analyze

### Expected
- [ ] Analysis panel shows friendly message: `API key rejected. Open Settings and paste a valid Anthropic key.`
- [ ] **Not** a stack trace or JSON blob
- [ ] App does not freeze; you can still navigate between tabs

Restore your real API key before continuing.

---

## Deviation log

Record any expectation that failed below so it can be triaged:

| Drill | Step | What you expected | What actually happened | Screenshot? |
|---|---|---|---|---|
|  |  |  |  |  |
|  |  |  |  |  |

If you hit a failure, paste the trace JSON / error message here when you ping for a fix. That saves a round-trip.

---

## Green light criteria for recording

Before you start Saturday's recording, you should have:
- ✅ Drills 2, 3, 4, 5, 6 all passing without deviations
- ✅ Drill 4's Final JSON shows `match_hex: true` and `data_type: "hex_u16_be"` on at least one field
- ✅ Drill 5 shows orange cards with live-updating numeric values AND sparklines
- ✅ Drill 6 shows payloads flowing to webhook.site

Drills 1, 7, 8 are nice-to-have for the video but won't prevent a successful demo.

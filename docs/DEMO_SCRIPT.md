# OmniBridge Demo Video Script

**Target duration:** 90 seconds
**Target audience:** Hackathon judges, async viewing
**Recording tool:** QuickTime (⌘⇧5) or OBS Studio
**Output:** 1080p MP4, ≤ 30MB (GitHub-friendly)

---

## Pre-recording checklist

- [ ] App built fresh: `npm run tauri dev` with clean window state
- [ ] **Settings → API key** set (test with "Test connection" button first)
- [ ] **Learned profiles cleared** (Settings → Clear learned data) so "first discovery" moment is fresh
- [ ] **Window sized ~1280×800** consistent for the recording
- [ ] **Webhook endpoint ready** — open https://webhook.site in a browser tab, copy URL
- [ ] **Demo mode emit rates feel right** — if too slow, edit `src/lib/demo.ts` temporarily
- [ ] **Zoom accessibility** — increase font size if needed (⌘+) so judges on laptops can read

---

## Shot list (90 sec)

### Shot 1 — Hook (0:00 – 0:10) [10s]

**Visual:** Close-up of an imagined warehouse/factory floor image OR just the OmniBridge splash + tagline.
Alternative: show a serial port terminal emulator with unreadable binary garbage.

**Voiceover/caption:**
> "Your warehouse scale from 1985 still works — but how do you get its weight data into Slack or your database?"

**Subtitle:** *"$1.2T of legacy hardware. 0% cloud-ready."*

---

### Shot 2 — The empty app (0:10 – 0:20) [10s]

**Visual:**
- Open OmniBridge, window in foreground
- Hero empty state visible: logo pulsing, tagline, 4 feature cards
- Cursor moves toward **"▶ Try Demo Mode"** button

**Voiceover/caption:**
> "OmniBridge. Plug in any serial device — or skip hardware entirely with Demo Mode."

**Action:** Click **▶ Try Demo Mode** → 3 demo device cards slide into sidebar.

---

### Shot 3 — Protocol Detective (0:20 – 0:40) [20s]

**Visual:**
- Click **CAS Weighing Scale** card's **▶ Monitor**
- Data streams in the buffer (ASCII lines `ST,GS,  1.234 kg`)
- Click **⚡ Live** toggle → green glow + progress "8/10"
- ~10 seconds later, Analysis Panel lights up with `CAS Scale · 95%`
- Click **📊 Dashboard** tab → field cards appear with live weight value + sparkline

**Voiceover/caption:**
> "Claude analyzes raw bytes in real-time. It identifies the protocol, extracts every field, and generates parsing regex — all automatically."

**Subtitle overlay:** *"Live structured values — zero API calls once identified"*

---

### Shot 4 — Agentic Investigation (0:40 – 1:05) [25s] ★ KILLER MOMENT

**Visual:**
- Click **✕** on CAS tab to close it
- Click **Arduino Sensor Array** card → **▶ Monitor**
- Data flowing: `temp=24.5,humid=62.3,light=512`
- Click **🔬 Investigate** (purple button) → Investigation tab opens automatically
- Steps stream in one-by-one with fly-in animation:
  - 🧠 *"Data shows key=value CSV format with numeric sensors..."*
  - 🔧 `get_device_metadata()` → 📥 "Arduino · Nano with DHT22 + LDR"
  - 🔧 `search_pattern(regex: "temp=[\d.]+", purpose: "verify temperature field")` → 📥 "15 matches found"
  - ✨ **Final**: `Arduino Sensor Array · 98%`

**Voiceover/caption:**
> "For unknown devices, Claude runs as a true agent — using tools to read more data, search for patterns, cross-reference VID/PID, even send queries to the device. Each step streams live."

**Subtitle:** *"5 tools · 8 max steps · multi-step reasoning with Opus 4.7"*

---

### Shot 5 — Cloud Bridge (1:05 – 1:20) [15s]

**Visual:**
- Switch to Dashboard tab → live sensor values updating
- Click **🔗 Webhook** button → modal opens
- Paste webhook.site URL, toggle Enable, set throttle 1000ms, click **Test**
- Green test result: `✓ 200 OK`
- Save modal → webhook counter in button starts incrementing `🔗 Webhook 3`
- Switch to browser tab with webhook.site → JSON payloads arriving in real-time

**Voiceover/caption:**
> "One click and structured data flows to any webhook — Zapier, n8n, Home Assistant, or your own API. Legacy hardware, first-class citizen of the modern stack."

---

### Shot 6 — Learning (1:20 – 1:30) [10s]

**Visual:**
- Click **✕** on Arduino tab to stop monitoring
- Click **▶ Try Demo Mode** again (toggles off, then on to simulate reconnect)
- Arduino card shows **⭐ Arduino Sensor Array · 98%** (purple badge)
- Click Monitor → Dashboard tab is **immediately populated** — no API call, no loading
- Badge visible: `⭐ Auto-recognized · 2 sessions`

**Voiceover/caption:**
> "And it learns. Reconnect the same device — instant recognition, zero API cost."

**Subtitle:** *"Profiles stored by VID:PID — your device library grows with every analysis"*

---

### Shot 7 — Outro (1:30 – 1:35) [5s]

**Visual:** OmniBridge logo animated, tagline:

> "OmniBridge"
> "Intelligent Serial Gateway"
> **github.com/YOUR_ORG/omnibridge**

**End card:** Opus 4.7 Hackathon 2026

---

## Recording tips

### QuickTime screen recording (macOS)
1. ⌘⇧5 → choose "Record Selected Portion"
2. Select window region (or fullscreen)
3. **Options → Microphone** if adding voiceover
4. Click Record
5. Press stop in menu bar or ⌘⇧5 again

### Better alternative: OBS Studio
- Free, more control over framerate
- Can set 1080p 30fps explicitly
- Better audio level monitoring
- Separate track for voiceover

### Voiceover
- Record separately from screen, sync in edit
- Use AirPods or basic USB mic
- Script it — don't freestyle (90s is tight)
- Add 0.5s pauses between shots for edit breathing room

### Edit in iMovie
1. Import screen recording → main track
2. Import voiceover → audio track 2
3. Add title cards (⇧⌘T) for subtitles
4. Background music: https://pixabay.com/music — search "tech minimal"
5. Export → Custom → 1080p 30fps H.264, quality "Medium" (~20MB)

### Captions
- GitHub autoplays muted → captions are critical
- Use iMovie's title tool, bottom-center
- High-contrast: white text, 50% black backdrop bar
- Keep captions ≤ 2 lines, 6-word rule

---

## Common pitfalls to avoid

| Mistake | Fix |
|---------|-----|
| Dashboard tab not clickable | Wait for Live trigger to complete; verify `hasParsableFields` by checking regex in analysis |
| Webhook "CORS error" in test | Use webhook.site (CORS-friendly); Zapier webhook URLs also work |
| Demo data too fast to read | Temporarily bump `interval_ms` in `src/lib/demo.ts` to 500ms for CAS |
| Investigation takes 45+ seconds | Normal for Opus 4.7 with thinking; record "real time" or speed up 1.5x in edit |
| API key shown in status bar | Blur/crop the settings modal in post |

---

## Fallback: 30-second "teaser" cut

If full 90s feels too long, produce a **30-second teaser** with just:
1. Empty state → Demo mode (5s)
2. Monitor CAS → Live trigger → Dashboard live values (15s)
3. Webhook payload flowing to webhook.site tab (7s)
4. Outro logo (3s)

Use for Twitter/Product Hunt, link to full 90s from README.

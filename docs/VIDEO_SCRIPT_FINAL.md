# OmniBridge — Final Video Script (90 seconds)

**For:** Claude Opus 4.7 Hackathon submission
**Target runtime:** 90s (±5s)
**Platform:** GitHub README embed + hackathon submission form
**Format:** 1080p MP4, H.264, ≤ 30 MB

This is the final shooting + voiceover script. Everything is timed. The goal is not to show every feature — it is to convince a judge in 90 seconds that **OmniBridge is impossible without Claude Opus 4.7**.

---

## The 10-second rule

Judges watch ~5000 submissions. If you do not earn their attention in the first 10 seconds, they skip. Your hook must:
1. State the problem in concrete dollars / scale
2. Hint that AI is the answer
3. Show something visually striking (UI moving, data streaming, *not* a splash screen)

---

## Pre-flight checklist (run once, in order)

- [ ] Kill all other apps — clean desktop, Do Not Disturb on
- [ ] App rebuilt: `npm install && npm run tauri dev`
- [ ] Settings → API key set, "Test connection" returned ✓
- [ ] Settings → "Clear learned data" pressed (fresh profile state for demo)
- [ ] Demo Mode **off** at start — we toggle it on-camera
- [ ] Window sized **1440 × 900** exactly (iMovie/OBS export target ratio)
- [ ] Font scaling default (⌘0 in browser dev tools if needed)
- [ ] Webhook tab open: `https://webhook.site` — copy the unique URL to clipboard
- [ ] System audio muted to avoid notification sound bleed
- [ ] Second take test: record 20 seconds, play back, verify framerate + audio levels

---

## Shot-by-shot script

Each shot has **Visual**, **Voiceover (VO)**, and **On-screen caption (CAP)**. Voiceover is written to be read at ~150 WPM — conversational, not rushed.

### Shot 1 — Hook (0:00 → 0:08) · 8 seconds

**Visual:**
- Cold open on OmniBridge app with the empty state visible (sidebar empty, hero card centered)
- The OmniBridge logo has a subtle pulse animation — let it run 2 seconds before anything else moves
- At 0:04, cursor moves toward "▶ Try Demo Mode"

**VO (14 words):**
> "Your warehouse scale from 1985 still works. Getting its data into Slack? Two weeks of engineering."

**CAP (bottom third):**
> `$1.2 trillion in RS-232 legacy hardware. Zero cloud-native.`

---

### Shot 2 — Product intro (0:08 → 0:18) · 10 seconds

**Visual:**
- Click "▶ Try Demo Mode" → 4 device cards slide into sidebar with stagger animation
- Pause 1 second to let the 4 cards register visually
- Hover over the **🏭 Modbus RTU PLC** card (last one) — subtle glow

**VO (20 words):**
> "OmniBridge is an AI agent that identifies and speaks to any serial protocol. No parsers. No vendor docs. No manual work."

**CAP:**
> `4 demo devices · any real serial port · Opus 4.7 under the hood`

---

### Shot 3 — Modbus agentic investigation → live dashboard (0:18 → 0:52) · 34 seconds · ★ CENTERPIECE

This is the single most important shot in the video. Judges will replay it. The **ending now goes all the way to a working live dashboard on binary data** — identifying the protocol is table stakes; turning raw bytes into decoded sparklines in the same 30-second flow is what no competitor will have. Do it in one take with screen recording; edit to speed up data-accumulation gaps if needed.

**Visual sequence:**

| Sub-shot | Time | Action |
|---|---|---|
| 3a | 0:18 | Click 🏭 Modbus RTU PLC card → **▶ Monitor** |
| 3b | 0:20 | DataBuffer tab opens. HEX view shows mysterious bytes: `01 03 14 00 FE 03 F5 ...` |
| 3c | 0:22 | Toggle HEX / ASCII — ASCII shows "·····" garbage (visually proving it's binary) |
| 3d | 0:25 | Pause on "This looks binary. Unknown." emotion (2s) |
| 3e | 0:27 | Click **🔬 Investigate** button — Investigation tab opens |
| 3f | 0:29 | **Thinking block streams in** (purple gradient, auto-expanded) — viewer can read the reasoning |
| 3g | 0:32 | **🔧 `analyze_binary_structure(hypothesis: 'low ASCII ratio — testing binary framing')`** |
| 3h | 0:34 | **📥** Result summary: `12% ASCII · frame=25B ×18 · ✓ Modbus CRC` — **highlight this line, zoom 1.2× for 1 second** |
| 3i | 0:36 | **Another thinking block**: "All 18 frames pass CRC16-IBM. This is Modbus RTU. Extracting 10 registers at fixed byte offsets." |
| 3j | 0:39 | **🔧 get_device_metadata()** → **📥** `Schneider Electric · Modicon M221` |
| 3k | 0:42 | **✨ Final:** `Modbus RTU · 98%` with confidence bar fill animation. **📊 Dashboard button lights up** (was greyed out). |
| 3l | 0:44 | Click **📊 Dashboard** tab → **6 register cards fly in** (temp, pressure, flow, motor_rpm, valve_%, alarm_bits) |
| 3m | 0:47 | Let the dashboard live-update for 3 seconds — **orange sparklines** drawing under each numeric register, values flashing on update |
| 3n | 0:50 | Linger on one sparkline (e.g. `temp: 254` with the curve moving) — visual payoff shot |

**VO (70 words, ~28 sec — this is the talkiest shot):**
> "For an unknown binary stream, Claude runs as a true agent. It analyzes byte structure, finds repeating 25-byte frames, validates the Modbus CRC, then extracts ten registers at fixed byte offsets — all in three tool calls. Not just identify the protocol. Decode it. Visualize it. Zero parsers written. Opus 4.7's adaptive thinking plus multi-step tool use made every step of that possible."

**CAP (two captions in this shot):**
- 0:30-0:42 overlay on trace: `Interleaved thinking + tool use · 3 calls · ~$0.04`
- 0:44-0:52 overlay on dashboard: `Binary → decoded → live sparklines. Zero parsers written.`

**Why this shot wins:** Competing submissions will stop at "AI identifies the protocol". This shot shows **identify → extract → live visualize** as one continuous agentic flow, with the user not touching a single config. That is the difference between a toy and a product.

---

### Shot 4 — ⚡ Live Mode auto-trigger on CAS scale (0:52 → 1:07) · 15 seconds

Shot 3 already showed the dashboard ending. Shot 4's job is to prove this is **not** a one-off demo — the same identify-and-visualize flow runs zero-click on any ASCII protocol via Live Mode. Don't repeat the "look, a dashboard" beat; emphasize the *auto-trigger*.

**Visual:**
- Click ✕ on Modbus tab to close
- Click ⚖️ CAS Weighing Scale → ▶ Monitor
- DataBuffer shows readable ASCII streaming: `ST,GS,  1.234 kg`
- Click **⚡ Live** toggle — green pulse + progress label `8 / 10`
- Don't click Investigate. Don't click anything. Just wait.
- At 10 lines accumulated (~2.5s later), Analysis panel auto-flashes: `CAS Scale · 95%`
- Dashboard button enables, auto-switches to Dashboard tab (already coded behavior)
- Weight sparkline draws live — let it run 3 seconds

**VO (28 words):**
> "Same agent flow runs on any protocol zero-click. Just turn Live Mode on. The regex runs locally — thousands of values per second, no additional API calls."

**CAP:**
> `⚡ Live Mode · auto-analyze at 10 lines · $0 per update after identification`

---

### Shot 5 — Cloud bridge (1:07 → 1:22) · 15 seconds

**Visual:**
- Still on CAS Scale Dashboard
- Click 🔗 **Webhook** button in header
- Modal slides in — paste URL (⌘V), toggle **Enable**, set throttle to 1000 ms
- Click **Test** → green `✓ 200 OK`
- Close modal → webhook counter in header starts ticking `🔗 3 · 🔗 4 · 🔗 5 ...`
- **Cut to webhook.site tab** in browser → JSON payloads arriving with fields: `weight`, `stable`, `timestamp`
- Scroll through one payload to show structure

**VO (28 words):**
> "One click forwards structured data to any webhook — Zapier, n8n, your own backend. Your 1985 scale is now a first-class citizen of the modern stack."

**CAP:**
> `HTTP POST → any endpoint. Throttled. Real-time.`

---

### Shot 6 — Device learning (1:22 → 1:30) · 8 seconds

**Visual:**
- Back to OmniBridge window
- Click ✕ on CAS tab to stop monitoring
- Click **▶ Demo Mode** toggle off, then on again (simulating reconnect)
- CAS Weighing Scale card now shows **⭐ CAS Scale · 95%** purple badge
- Click Monitor → Dashboard tab **immediately populated** (no analyzing spinner)

**VO (18 words):**
> "And every identification is remembered. Reconnect the same device — instant recognition, zero API calls, forever."

**CAP:**
> `⭐ Device Profile Learning — by USB VID:PID`

---

### Shot 7 — Outro (1:30 → 1:37) · 7 seconds

**Visual:**
- Fade to black
- OmniBridge logo with tagline animates in:
  - `⬡ OmniBridge`
  - `Legacy hardware, first-class citizens`
  - `github.com/<your-org>/omnibridge`
- Small footer: `Built for Claude Opus 4.7 Hackathon 2026`

**VO (10 words):**
> "OmniBridge. Built on Claude Opus 4.7. Available on GitHub."

**CAP:** (none — let the logo speak)

---

## Timing budget summary

| Shot | Length | Cumulative |
|---|---|---|
| 1 · Hook | 8s | 0:08 |
| 2 · Intro | 10s | 0:18 |
| 3 · Modbus agent → dashboard ★ | 34s | 0:52 |
| 4 · Live Mode auto-trigger | 15s | 1:07 |
| 5 · Webhook | 15s | 1:22 |
| 6 · Learning | 8s | 1:30 |
| 7 · Outro | 7s | 1:37 |

Total: **1:37** — within the 90-100s band. If you overrun on Shot 3 (easy to do — the dashboard populate is a strong visual, you'll want to linger), tighten by:
1. Cut 2s from Shot 4 (shorter Live accumulation wait — edit out the 8→10 progress beats)
2. Cut 1s from Shot 6 (faster reconnect toggle)

Do NOT cut Shot 3. It is the only shot that matters.

---

## Voiceover recording tips

1. **Record the VO separately**, then sync in edit. Screen-record narration is never clean enough.
2. **Write the VO on one page**, print it, read from paper — reading from screen produces unnatural pacing.
3. Record **two takes** of each shot's VO. Pick the best in edit.
4. Pace: ~150 words/min (conversational). Do not rush the Shot 3 voiceover — the visuals need time to land.
5. Leave **0.3 seconds of silence** at the start of each line — easier to trim than to add.
6. Microphone: AirPods Pro (auto noise reduction) or any USB mic. Avoid laptop mic.

---

## Captions / subtitles

- GitHub autoplays embedded video **muted**. Your captions MUST tell the story without audio.
- Use iMovie's title tool, **bottom-center**, two-line maximum
- Font: sans-serif, white text, 50% black box behind
- Caption appears 0.3s after the VO line starts, dismissed 0.3s before next caption
- **Caption cadence: one key line per shot** (6 total) — do not caption every VO sentence

---

## B-roll / supplementary footage (optional, use if you have 30 extra minutes)

- A hand plugging a USB-serial cable into a laptop (shot over the shoulder) — adds physicality to the "real hardware" story
- An actual Arduino board on a desk with LEDs blinking — 5-second insert during Shot 2
- A split-screen showing "traditional way" (engineer typing regex in VS Code for 10 seconds) vs "OmniBridge way" (one click, done) — powerful but time-intensive to edit

Do **not** add B-roll if it pushes you past 100 seconds. The 90s version is tight and polished; the 2-minute version is a student film.

---

## Editing checklist

- [ ] Export in 1080p 30fps, H.264, AAC audio
- [ ] Target file size: **15-25 MB** (GitHub README inline viewer limit is 100 MB but smaller = faster loads)
- [ ] Title card for first 0.5s: black with "OmniBridge" fade-in (gives the viewer's eye a second to focus)
- [ ] End card lingers on GitHub URL for **2 full seconds** — this is where motivated viewers pause to read
- [ ] Audio: background music at **-18 dB** (barely audible), voiceover at **-6 dB** peak
- [ ] Music: "tech minimal" from pixabay.com (royalty-free) — search "data pulse" or "quiet algorithm"

---

## Fallback: 30-second teaser cut

If the full 90s feels too long for Twitter / Product Hunt:

| Segment | Source shots | Duration |
|---|---|---|
| Hook (truncated) | Shot 1 VO shortened | 5s |
| Modbus agentic magic | Shot 3 sub-shots 3e → 3k | 18s |
| Webhook payload | Shot 5 final 5 seconds | 5s |
| Outro | Shot 7 logo only | 2s |

This teaser links to the full video from the README, keeps all three must-see moments.

---

## The ask for judges (implicit but important)

Every frame of this video should signal one claim:
**"This isn't GPT + serial port. This is Opus 4.7's multi-step tool use + adaptive thinking + prompt caching doing something no other model configuration can do."**

If you rewatch your final edit and cannot point to three moments that specifically showcase Opus 4.7 (thinking stream, tool-call chain, CRC validation result), re-edit until you can.

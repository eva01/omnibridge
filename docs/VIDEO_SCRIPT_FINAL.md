# OmniBridge — Final Video Script

**For**: Claude Opus 4.7 Hackathon submission (asynchronous judging).
**Target runtime**: 2:20–2:30 (hackathon ceiling is 3:00 — we stay well inside for discipline).
**Format**: 1080p MP4, H.264, ≤ 30 MB.

This is the final shooting + voiceover script. Everything is timed. The goal is not to cram every feature — it is to convince a judge in 150 seconds that **OmniBridge is an AI agent category that didn't exist a year ago, and it's only possible because of Opus 4.7.**

---

## Strategic framing

Judging criteria weight: Impact 30% · Demo 25% · Opus 4.7 Use 25% · Depth 20%.

Problem Statement alignment we are anchoring on:
- **Primary — "Build For What's Next"**: an AI-agent-driven workflow that could not have existed before this model class. Video Shot 1 explicitly namedrops this.
- **Supporting — "Build From What You Know"**: the creator is an engineer who shipped production integrations against the exact devices demoed here. Surfaces in README and submission, *not* in the video voiceover (would feel salesy).

Every shot must pull weight toward at least one of the four rubric items. If a shot doesn't, cut it.

---

## The 10-second rule

Judges watch hundreds of submissions asynchronously. Earn their attention in the first 10 seconds or they skip. Our hook must:
1. Claim novelty (**Build For What's Next** namedrop)
2. Show something visually striking (live byte stream, not a splash screen)
3. Imply the payoff without explaining it

---

## Pre-flight checklist

- [ ] Fresh `npm run tauri dev` — no hot-reload artifacts
- [ ] API key set, "Test connection" returned ✓
- [ ] "Clear learned data" pressed (fresh profile state)
- [ ] Demo Mode **off** at start (we toggle on-camera)
- [ ] Window exactly **1440 × 900**
- [ ] webhook.site unique URL copied to clipboard
- [ ] DND on, notifications silenced, other apps closed
- [ ] Two test recordings of 10 seconds each — playback confirms audio levels + no dropped frames
- [ ] Script printed on paper — **do not read from screen during VO**

---

## Shot-by-shot script

### Shot 1 — Hook (0:00 → 0:12) · 12s

**Visual**
- Cold open: OmniBridge window already in foreground, empty state with pulsing logo
- At 0:02, click **▶ Try Demo Mode** — 4 cards slide into sidebar with stagger
- At 0:06, click 🏭 **Modbus RTU PLC** → **▶ Monitor**
- Bytes start streaming in HEX: `01 03 14 00 FE 03 F5 ...`
- Camera (digital zoom 1.1×) lingers on the unreadable hex stream for 2 seconds

**VO (25 words)**
> "This is a category of AI agent that didn't exist a year ago. Claude identifies any legacy serial protocol — binary, undocumented, proprietary — in under a minute. Watch."

**CAP (overlay bottom third)**
> `Problem Statement: Build For What's Next`

---

### Shot 2 — Raw bytes are unreadable (0:12 → 0:24) · 12s

**Visual**
- Toggle **HEX → ASCII** view — screen fills with `·······` dots (binary proof)
- Toggle back to **HEX**
- Close-up shot on one frame: `01 03 14 00 FE 03 F5 00 2D 07 08 ... XX XX`
- Cursor hovers over the **🔬 Investigate** button (purple glow on hover)

**VO (22 words)**
> "Traditional integration: an engineer reads vendor docs, writes a custom parser, ships glue code. Eighteen hours per device, median. Industry-wide."

**CAP**
> `~$1.2 trillion of RS-232 hardware stranded outside the cloud.`

---

### Shot 3 — Surgical agentic investigation → live dashboard (0:24 → 0:58) · 34s · ★ CENTERPIECE

This is the single most important shot. The flow goes end-to-end from unreadable bytes to decoded live dashboard in one continuous take. **Empirically verified behavior**: Claude calls exactly one purpose-built tool — `analyze_binary_structure` — and that one tool returns enough evidence (CRC match + frame autocorrelation + sample frames) to commit. Narrative pivots on **surgical efficiency**, not tool-chain length.

**Sub-shot sequence**

| Sub | Time | Action |
|---|---|---|
| 3a | 0:24 | Click **🔬 Investigate** — Investigation tab opens automatically |
| 3b | 0:26 | **Thinking block streams in** (purple gradient, auto-expanded). Content mentions "hypothesize Modbus RTU framing: slave 1, FC 3, 10 registers × 2 bytes + CRC" |
| 3c | 0:30 | **🔧 `analyze_binary_structure`** called with the specific Modbus hypothesis as input |
| 3d | 0:32 | **📥** Result summary in trace: `13% ASCII · frame=25B ×21 · ✓ Modbus CRC` — **hold the frame 1.5 seconds, zoom 1.15×** (this is the single most judge-visible payoff) |
| 3e | 0:35 | **Second thinking block / text block**: "Strong Modbus RTU match — 21 of 21 frames pass CRC16. Emit five fields with byte-offset regex." |
| 3f | 0:39 | **✨ Final**: `Modbus RTU · 98%` — confidence bar fills, 📊 Dashboard button lights up |
| 3g | 0:41 | **💰 Investigation cost card appears** (green gradient). Linger 3 seconds. Text visible: `Investigation cost · ~$0.22 · 65% cache hit` |
| 3h | 0:44 | Click **📊 Dashboard** tab |
| 3i | 0:46 | **5 orange register cards fly in**: reg0, reg1, reg2, reg3, reg4 — with live numeric values (e.g. `57`, `926`, `138`, `2085`, `81`) |
| 3j | 0:50 | Let sparklines draw for 3 seconds — values flashing, min/max labels visible at each end |
| 3k | 0:53 | Hover-linger on `reg1` card — sparkline moving, value in 900s range with `916 ── 930` labels |
| 3l | 0:56 | Quick pan to Investigation tab to show the small `💰 ~$0.22 · 65% cache` chip in header — reinforces cost narrative before cutting |

**VO (68 words across 34 seconds — deliberately slower pace, let visuals breathe)**
> "An unreadable binary stream. Claude makes one surgical move — calls analyze_binary_structure with a specific Modbus hypothesis. The tool validates CRC16 across twenty-one consecutive frames and returns frame structure. One tool call. Two API requests. About twenty cents in tokens. Five decoded registers stream to a live dashboard in the same window. No parsers written. No vendor docs consulted."

**CAPs (two overlays in this shot)**
- 0:28 → 0:42 overlay: `One purpose-built tool · 21/21 frames pass CRC16`
- 0:44 → 0:58 overlay: `Binary → decoded → live sparklines. Zero parsers written.`

**Why this shot wins**

The "surgical agent" framing is more impressive than "chatty multi-tool chain":
- **Efficiency as signal of intelligence**: a good engineer picks the right tool once; a novice calls many.
- **Cost honest + compelling**: twenty cents for what used to take 18 hours of engineering. The ratio is the story.
- **Verifiable via Copy JSON**: judges who click 📋 Copy JSON see the exact 2-API-call, 1-tool trace. What the video claims matches reality exactly.
- Competing submissions stop at "AI identifies the protocol." This shot shows **identify → decode → visualize → and here's the verified bill** in one continuous flow.

---

### Shot 4 — User override (0:58 → 1:14) · 16s · NEW

**Visual**
- Still on Dashboard tab
- Click the ✎ edit icon next to one register card (e.g. `motor_rpm`)
- Inline edit panel slides open below the field row
- Current regex visible: `^01 03 14 [0-9A-F]{2} [0-9A-F]{2} [0-9A-F]{2} [0-9A-F]{2} [0-9A-F]{2} [0-9A-F]{2} ([0-9A-F]{2} [0-9A-F]{2})`
- Type a tweak (adjust capture position or data type hint)
- Press **Enter** → panel closes → sparkline redraws with new values

**VO (22 words)**
> "Claude's extraction rules are editable, not authoritative. If the agent picks a wrong byte offset, fix it inline. The dashboard re-parses live."

**CAP**
> `Human-in-the-loop — Claude's output is a starting point, not a black box.`

---

### Shot 5 — Zero-click Live Mode (1:14 → 1:30) · 16s

**Visual**
- Close Modbus tab
- Click ⚖️ **CAS Weighing Scale** card → **▶ Monitor**
- Readable ASCII streams: `ST,GS,  1.234 kg`
- Click **⚡ Live** toggle — pulsing green, progress `8 / 10`
- Do not click anything else
- ~2.5 seconds later, Analysis auto-fires: `CAS Scale · 95%`
- Dashboard auto-switches on — weight card with sparkline bouncing

**VO (22 words)**
> "Same agent flow runs zero-click for any protocol. Turn on Live Mode. Regex Claude generated runs locally — no API calls per packet."

**CAP**
> `⚡ Live auto-triggers at 10 lines · $0 per update after identification`

---

### Shot 6 — Cloud bridge (1:30 → 1:45) · 15s

**Visual**
- Click **🔗 Webhook** button in header
- Modal opens — paste URL (⌘V), toggle **Enable**, throttle 1000 ms
- Click **Test** → green `✓ 200 OK`
- Close modal → webhook counter in header ticks `🔗 3 · 🔗 4 · 🔗 5`
- Cut to browser tab with webhook.site — JSON payloads arriving with `weight`, `stable`, `timestamp` fields

**VO (22 words)**
> "One click forwards structured data to any webhook. Zapier, n8n, your backend. Your 1985 scale is now a first-class cloud citizen."

**CAP**
> `HTTP POST → any endpoint. Throttled. Real-time.`

---

### Shot 7 — Device profile learning (1:45 → 1:55) · 10s

**Visual**
- Close CAS tab
- Toggle Demo Mode off, then on (simulating reconnect)
- CAS Scale card shows **⭐ CAS Scale · 95%** purple badge
- Click Monitor → Dashboard **immediately populates** without API call

**VO (18 words)**
> "Every identification is remembered. Reconnect the same device — instant recognition, zero API calls, forever."

**CAP**
> `⭐ Device profiles stored by USB VID:PID`

---

### Shot 8 — Outro (1:55 → 2:10) · 15s

**Visual**
- Fade to black
- OmniBridge logo animates in with three lines of text appearing in sequence:
  - Line 1 at 2:07: `⬡ OmniBridge`
  - Line 2 at 2:10: `An agent category that didn't exist a year ago.`
  - Line 3 at 2:13: `Built with Claude Opus 4.7 · MIT · github.com/adindamochamad/omnibridge`
- Hold final frame for 5 seconds — judges motivated enough to click the repo link need time to read the URL
- Faint footer text: `Opus 4.7 Hackathon 2026`

**VO (30 words)**
> "OmniBridge. Built by an engineer who has lived the legacy-hardware integration tax. Open source on GitHub. And it only works because Opus 4.7 can reason from raw bytes, pick a purpose-built tool, and commit — surgically."

**CAP (appears at 2:13, stays to end)**
> `🏭 Industrial IoT · 🔌 Serial Gateway · 🧠 Agentic AI`

---

## Timing budget

| Shot | Length | Cumulative |
|---|---|---|
| 1 · Hook ("didn't exist a year ago") | 12s | 0:12 |
| 2 · Raw bytes unreadable | 12s | 0:24 |
| 3 · Surgical agentic investigation + dashboard ★ | 34s | 0:58 |
| 4 · User override | 16s | 1:14 |
| 5 · Zero-click Live Mode | 16s | 1:30 |
| 6 · Webhook cloud bridge | 15s | 1:45 |
| 7 · Device learning | 10s | 1:55 |
| 8 · Outro + repo link | 15s | 2:10 |

**Total: 2:10** — well inside the 3-minute ceiling. Tight pacing is a signal of editorial discipline to async judges watching at 1× speed.

If Shot 3 overruns (dashboard payoff often tempts lingering):
1. Cut 2s from Shot 5 (shorter Live accumulation wait)
2. Cut 2s from Shot 6 (skip a beat of webhook counter ticking)
3. Cut 2s from Shot 7 (faster reconnect toggle)

**Do NOT cut Shot 3.** Do NOT cut Shot 4. Those are the Opus-4.7-specific moments.

---

## Voiceover pacing notes

- Target 115-125 WPM — slower than typical reading pace. Gives visuals room to land.
- Total VO word count: ~225 words across 130 seconds = 104 WPM. Conservative intentional.
- Record each shot's VO as a separate take. Mix separately from screen recording.
- Leave 0.3s of silence between sentences — easier to trim than to add.
- After Shot 3's "twenty cents in tokens" line, pause a full second before "five decoded registers" — let the price point land before pivoting to the payoff.

---

## Captions

GitHub autoplays embedded video muted. Captions are critical. One line per shot, bottom-center, white text on 50% black box. Never caption a full VO sentence — caption the payoff word.

---

## B-roll (optional, only if time allows after main edit)

- 3-second cutaway: hand plugging USB cable into laptop (shot on phone) — insert during Shot 1 for physicality
- 5-second cutaway: a real Arduino board with LEDs blinking on a desk — insert during Shot 5
- Use only if it doesn't push runtime past 2:40

---

## Final checklist before upload

- [ ] Export 1080p 30fps H.264, AAC audio
- [ ] File size ≤ 25 MB (GitHub inline playback faster)
- [ ] First frame is NOT black — show the OmniBridge window immediately for attention-grabbing thumbnail
- [ ] Audio levels: background music -18 dB, VO -6 dB peak
- [ ] No sensitive API keys visible in any frame — crop/blur Settings modal shots if needed
- [ ] Repo URL in outro is the ACTUAL URL, not a placeholder
- [ ] Watch the full video with speakers muted once — captions must tell the story alone
- [ ] Watch the full video at 1.5× once — if any shot feels slow even then, trim it

---

## The ask for judges (never said out loud, but every frame should signal it)

**"This workflow did not exist a year ago. Opus 4.7 made it possible. The product is real, the code is open, the market is a trillion dollars. Give us Top 6."**

If you rewatch your final edit and cannot point to three moments that specifically showcase Opus 4.7 capabilities (thinking stream, tool chain, cost summary), re-edit until you can.

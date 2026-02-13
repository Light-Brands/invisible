# Visual Identity — Invisible Brand System

## Overview
Invisible's visual identity is dark-first, monochrome, and minimal. The aesthetic communicates: absence, silence, and absolute privacy. Nothing flashy. Nothing that draws attention. The app should feel like it barely exists.

## Color Palette

### Primary Colors
| Color | Hex | Usage |
|-------|-----|-------|
| Void Black | #0A0A0A | Primary background |
| Smoke | #1A1A1A | Secondary background, cards |
| Ash | #2A2A2A | Borders, dividers |
| Ghost | #4A4A4A | Disabled text, subtle UI |
| Silver | #8A8A8A | Secondary text |
| Bone | #CACACA | Primary text |
| White | #F0F0F0 | High-emphasis text, headings |

### Accent Colors (used sparingly)
| Color | Hex | Usage |
|-------|-----|-------|
| Phantom Blue | #3A7BF8 | Active states, links, primary action |
| Ember | #F83A3A | Destructive actions, burn rooms, warnings |
| Signal Green | #3AF87B | Connected, verified, success |

### Design Principles
- **Dark mode only** — no light theme. Privacy lives in the dark.
- **Monochrome-dominant** — color is used as signal, not decoration
- **Accent sparingly** — Phantom Blue for interactive, Ember for destructive, Signal Green for secure
- **High contrast text** — accessibility on dark backgrounds

## Typography

### Font Stack
- **Primary**: Inter (sans-serif, highly legible, open source)
- **Monospace**: JetBrains Mono (for codes, keys, technical displays)
- **Fallback**: system sans-serif

### Scale
| Level | Size | Weight | Usage |
|-------|------|--------|-------|
| Display | 32px | 700 | Screen titles |
| Heading | 24px | 600 | Section headers |
| Title | 18px | 600 | Card titles, names |
| Body | 16px | 400 | Message text, content |
| Caption | 13px | 400 | Timestamps, metadata |
| Micro | 11px | 500 | Labels, badges |

## Logo

### Concept
The Invisible logo is a "vanishing" mark — an abstract shape that appears to be fading, dissolving, or phasing out of existence. It should evoke the concept of something that was there and is now gone.

### Specifications
- **Primary mark**: Abstract dissolving/fading geometric form
- **Wordmark**: "INVISIBLE" in Inter Bold, tracked wide (letter-spacing: 0.15em)
- **Monochrome only** — logo is always white on dark or dark on light (no color version)
- **Minimum size**: 24px height for mark, 12px height for wordmark
- **Clear space**: 1x the height of the mark on all sides

### Usage
- App icon: mark only on #0A0A0A background
- Splash screen: mark + wordmark centered
- In-app: mark only in top bar (subtle, small)
- External: mark + wordmark horizontal lockup

## UI Components

### Messages
- Sent messages: Smoke (#1A1A1A) bubble, left-aligned
- Received messages: Ash (#2A2A2A) bubble, left-aligned
- No colorful bubbles — everything is subtle and quiet
- Timestamps in Ghost (#4A4A4A), shown on tap only

### Burn Room Indicator
- Ember (#F83A3A) pulsing dot next to room name
- Timer countdown in Ember
- Room background: slightly warmer dark (#0F0A0A)

### VPN Status
- Connected: Signal Green dot + "Ghost VPN" label
- Connecting: pulsing Phantom Blue dot
- Disconnected: Ember dot + "No Connection" (app locked)

### Animations
- Minimal, functional animations only
- Message send: brief fade-out (message "disappears" into the network)
- Burn room: ember glow dissolve on message expiry
- VPN connect: quiet pulse on connection established
- No playful animations, no bouncing, no delight-driven motion

## Iconography
- Line icons only (1.5px stroke, rounded caps)
- Monochrome (Silver or Bone)
- Custom icon set: lock, shield, flame, ghost, eye-slash, key, clock
- No emoji in UI chrome (messages can contain emoji)

## App Feel
- **Silent** — the app doesn't announce itself
- **Calm** — no urgency, no attention-grabbing
- **Absent** — designed to leave no impression
- **Trustworthy** — through restraint, not decoration

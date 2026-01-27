# chicalipdf

> A fast, offline-first, no-subscription PDF viewer & editor for normal people.

---

## 1. Overview

**chicalipdf** is a lightweight, downloadable PDF viewer and editor for Windows that works fully offline, has a clean black-and-white interface, and charges a simple one-time $1 fee.

It is designed for the general public—people who just want to open a PDF, fill a form, edit text, sign it, and move on without accounts, subscriptions, or web apps.

The project is open source (MIT licensed) and optimized for speed, clarity, and minimal UI.

---

## 2. Problem Statement

Existing PDF tools fall into two frustrating categories:

- **Online tools** that require uploads, accounts, or subscriptions
- **Desktop apps** that are bloated, expensive, or visually overwhelming

Users want a tool that:
- Works offline
- Is fast and lightweight
- Has a simple UI
- Does not require recurring payments

chicalipdf exists to remove friction from everyday PDF tasks.

---

## 3. Goals

### Primary Goals
- Enable common PDF tasks with minimal UI
- Run fully offline (no cloud, no telemetry)
- Be fast, lightweight, and reliable
- Cost **$1 one-time**, no subscriptions

### Design Goals
- Black & white UI
- Red used **only** for destructive actions
- Single-window experience
- McMaster-Carr–level functional clarity

### Non-Goals (Explicitly Out of Scope)
- OCR
- Cloud sync
- Collaboration
- AI features
- Web-based editor

---

## 4. Target Users

**General public**, including:
- Students filling forms
- Office workers editing PDFs
- Anyone signing or lightly modifying documents

No technical knowledge required.

---

## 5. MVP Feature Set (v1)

### 5.1 PDF Viewing
- Open and render PDFs locally
- Zoom, scroll, and navigate pages
- Page thumbnails in left sidebar

### 5.2 Form Filling
- Click into existing form fields
- Enter and save text
- Export filled PDF

### 5.3 Text Editing
- Click-to-edit existing text
- "Good enough" editing (minor layout shifts acceptable)
- Substitute close fonts when originals are unavailable

### 5.4 Signing
- Add signature via:
  - Typed text
  - Drawn signature (mouse / trackpad)
- Place signature anywhere on page

### 5.5 Version Control (Local)
- Git-like history model
- Automatic snapshots on save
- Ability to:
  - View history
  - Restore previous versions
- All versions stored locally

---

## 6. User Experience & UI

### Layout
- **Single window**
- **Left sidebar**:
  - Page thumbnails
- **Main canvas**:
  - PDF content

### Interaction Model
- Click-to-edit text directly
- No mode switching unless necessary
- Minimal toolbars

### Visual Design
- Black & white UI
- System fonts
- Red reserved for:
  - Delete
  - Overwrite
  - Irreversible actions

---

## 7. Technical Requirements

### Platform
- Windows (v1)

### Performance
- Fast startup
- Low memory usage
- Handles large PDFs gracefully

### Architecture
- Desktop-native or hybrid (implementation flexible)
- Fully offline
- No background services

### Storage
- Local file system only
- No cloud dependencies

---

## 8. Licensing & Monetization

### Open Source
- MIT License
- Public GitHub repository: `chicalipdf`

### Pricing
- $1 one-time payment
- No subscriptions
- No feature gating after purchase

---

## 9. Security & Privacy

- No accounts
- No telemetry
- No analytics
- No external network calls required

---

## 10. Versioning Model (Design Decision)

chicalipdf will implement a **sidecar-based version history system** instead of traditional in-file versioning.

### Key Principles
- The original PDF file remains untouched unless explicitly saved over
- All version metadata is stored in a **separate local file** linked to the PDF
- No branching, merging, or diffs exposed to the user

### Sidecar File
- Stored alongside the PDF as a **hidden file** (OS-level hidden)
- Not shown to users in normal file browsing
- Naming tied deterministically to the PDF

Contains:
- Timestamps
- Auto-generated change summaries
- Snapshot references

### UI Representation
- Left sidebar section: **History**
- Displays a chronological list of changes
- Users can:
- View what changed (auto-generated summaries)
- Restore a previous version (creates a new version)

### Failure Handling
- If the sidecar file is missing or corrupted:
  - chicalipdf warns the user
  - The PDF opens normally
  - A new history file can be created

This avoids "FINAL_FINAL.pdf" behavior while keeping complexity manageable and user-friendly.

---

## 11. Technical Stack (Initial Direction)

### Core PDF Engine (Candidates)

#### MuPDF
- Very fast rendering
- Actively maintained
- Strong text extraction and editing primitives
- Commercial-friendly licensing

#### Poppler
- Mature and widely used
- Strong PDF standard compliance
- Heavier dependency graph
- Text editing more limited

**Initial Recommendation:** MuPDF as primary engine

### App Layer
- Lightweight desktop shell (final choice flexible)
- Emphasis on:
  - Fast startup
  - Low memory usage
  - Native-feeling UI

---

## 12. Distribution & Monetization

### Distribution
- Primary: downloads from **chicali.tech**
- Secondary: GitHub Releases

### Monetization
- $1 one-time payment
- Honor system
- No DRM
- No license enforcement

---

## 13. Future Roadmap (Post-v1)

### Possible Enhancements
- macOS build
- Better change visualization
- Export version history

### Explicitly Not Planned
- Cloud sync
- Collaboration
- AI-powered editing

---

## 14. Open Questions

- Branding polish (icon, typography)
- Final UI micro-interactions

---

## 15. Success Metrics

- App launches under 1s
- Users avoid duplicate "FINAL" files
- Zero required logins
- Minimal UI complaints

---

**chicalipdf** should feel like an operating system utility — quiet, fast, and dependable.

(And yes, always lowercase unless starting a sentence. We live in a society.)

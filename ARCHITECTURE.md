# ARCHITECTURE.md

## Overview

chicalipdf is built as a local-first desktop application with a minimal dependency surface.

## Core Components

### PDF Engine
- MuPDF (via Rust bindings)
- Responsibilities:
  - Rendering
  - Text extraction
  - Writing updated PDFs

### Core Logic (Rust)
- File IO
- Change tracking
- Sidecar management
- Business logic

### UI Layer
- Single-window UI
- Left sidebar (pages + history)
- Click-to-edit interactions

## Versioning Model

### Sidecar File
- Hidden file stored next to the PDF
- Example: `.document.pdf.chicalipdf`

### Contents
- Timestamped entries
- Auto-generated summaries
- Snapshot references

### Behavior
- Original PDF untouched until save
- Restoring history creates a new version
- Missing/corrupt sidecar triggers warning

---

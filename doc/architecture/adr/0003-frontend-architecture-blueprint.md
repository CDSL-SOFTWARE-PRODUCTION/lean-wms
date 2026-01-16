# 3. Frontend Architecture Blueprint

Date: 2026-01-15

## Status

Accepted (Migrated from BLUEPRINT_FRONTEND.md)

## Context

To ensure high productivity in noisy and fast-paced warehouse environments, the frontend must prioritize speed, error prevention (Poka-Yoke), and multi-modal feedback.

## Decision

We adopt a **Mobile-First, Guided Workflow** design with **Multi-modal Feedback** and **Offline Persistence**.

### Design Principles

1. **The 3-Button Dashboard**: Entry points are limited to NHẬP (Inbound), XUẤT (Outbound), and KIỂM KHO (Counting) to minimize cognitive load.
2. **Multi-modal Feedback**: Every action must trigger three signals:
   - **Visual**: Green/Red full-screen overlays.
   - **Audio**: High-pitched "Tít" (Success) vs. Low-pitched "Bíp bíp" (Error).
   - **Haptic**: Short vibration (Success) vs. Long double vibration (Error).
3. **Poka-Yoke (Error Proofing)**:
   - Real-time validation during scanning.
   - Manager Override mechanism via PIN/Biometrics for exceptions.
4. **Eye & Thumb Rule**: Interactive elements are placed in the bottom 1/3 of the screen for one-handed operation.

### Technical Stack

- **Framework**: Expo (React Native) with TypeScript.
- **State**: Redux Toolkit with persistence for offline capabilities.
- **Scanner**: High-performance camera-based scanning in Phase 1; Bluetooth HID 2D Imager support in Phase 2.

## Consequences

- **Reduced Training Time**: Guided steps allow new workers to be productive in < 5 minutes.
- **Low Error Rates**: Immediate multi-sensory feedback prevents workers from progressing with incorrect data.
- **Accessibility**: High contrast and sound/vibration support workers in diverse lighting and noise conditions.

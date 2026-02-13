# Epic 08: Hardening — Anti-Forensics & Device Lockdown

## Overview
Implement comprehensive anti-forensic measures, screen capture prevention, and device lockdown features to prevent data leakage through side channels.

## User Stories

### US-08.1: No Thumbnail Generation
As a user, I want no image thumbnails cached by the OS so forensic tools can't find them.

**Acceptance Criteria:**
- Media loaded directly from encrypted DB to display (no file:// URIs)
- No OS-level thumbnail cache entries
- No EXIF data preserved on received images
- Verified on both Android and iOS

### US-08.2: No Clipboard Integration
As a user, I want to prevent message content from being copied to the system clipboard.

**Acceptance Criteria:**
- Text selection disabled in message views
- No copy/paste option in message context menu
- System clipboard never receives Invisible content
- Screenshots blocked (see US-08.4)

### US-08.3: Blank Task Switcher Preview
As a user, I want the app to show a blank screen in the task switcher so observers can't see my messages.

**Acceptance Criteria:**
- Android: FLAG_SECURE prevents task switcher preview
- iOS: Blank overlay displayed when app enters background
- Configurable: blank screen or decoy screen (e.g., calculator app appearance)

### US-08.4: Screen Capture Prevention
As a user, I want screenshot and screen recording to be blocked.

**Acceptance Criteria:**
- Android: FLAG_SECURE blocks screenshots and screen recording
- iOS: UIScreen.capturedDidChange detection with user warning
- iOS: Content hidden when screen recording detected
- Invisible per-user watermarking option for group chats (trace leaks)

### US-08.5: Secure Keyboard
As a user, I want an option to type with a built-in keyboard that doesn't log my keystrokes.

**Acceptance Criteria:**
- Built-in keyboard option (bypasses Gboard/SwiftKey logging)
- No predictive text, no autocorrect dictionary
- No keystroke analytics
- User can toggle between system keyboard and secure keyboard

### US-08.6: Notification Privacy
As a user, I want push notifications to reveal no message content or sender info.

**Acceptance Criteria:**
- Push notification text: "New message" only
- No sender name, no message preview, no conversation name
- Notification icon is generic (no Invisible branding option)
- Lock screen display: minimal

### US-08.7: RAM-Based Media Viewer
As a user, I want media files to only exist in RAM while viewing, never on disk.

**Acceptance Criteria:**
- Media decrypted to RAM buffer
- Displayed directly from memory
- Memory zeroed after viewer closed
- No temp files created
- Verified via filesystem monitoring

## Technical Requirements
- Android: WindowManager.LayoutParams.FLAG_SECURE
- iOS: UIScreen.capturedDidChange notification, UITextField.isSecureTextEntry
- Custom keyboard: Flutter custom input widget
- Memory management: mlock for media buffers

## Dependencies
- Epic 00 (Foundation), Epic 01 (Messaging), Epic 05 (Media)

## Architecture References
- [access-control.md](../architecture/access-control.md), [zero-log-doctrine.md](../architecture/zero-log-doctrine.md)

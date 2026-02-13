# Epic 05: Media — Encrypted File & Media Transfer

## Overview
Implement encrypted file, image, voice message, and media transfer with streaming encryption and RAM-based viewing.

## User Stories

### US-05.1: Send Encrypted Media
As a user, I want to send images, files, and voice messages with full end-to-end encryption.

**Acceptance Criteria:**
- Media encrypted with per-file AES-256-GCM key
- File key encrypted with conversation key
- Large files chunked and streamed
- No size limit (chunked transfer)
- Supported: images, video, audio, documents, any file type

### US-05.2: RAM-Based Media Viewer
As a user, I want to view media without it being written to disk so forensic recovery is impossible.

**Acceptance Criteria:**
- Media decrypted in memory only
- Never written to device filesystem
- Memory freed and zeroed after viewing
- No OS thumbnail cache generated
- No media library integration

### US-05.3: Voice Messages
As a user, I want to record and send encrypted voice messages.

**Acceptance Criteria:**
- Audio recorded directly to encrypted buffer (no temp file)
- Opus codec for efficient compression
- Encrypted and sent like any other media
- Playback from RAM only

### US-05.4: Streaming Encryption
As a developer, I need streaming encryption for large files so memory usage stays bounded.

**Acceptance Criteria:**
- ChaCha20-Poly1305 streaming AEAD for large files
- Fixed chunk size (64KB) with per-chunk authentication
- Chunk order authenticated (prevent reordering)
- Last chunk flagged (prevent truncation)

### US-05.5: Media in Burn Rooms
As a user, I want media in burn rooms to be destroyed with the room.

**Acceptance Criteria:**
- Media encryption key tied to burn room ephemeral key
- When room burns, media becomes unrecoverable
- No media saving/downloading in burn rooms

## Technical Requirements
- Streaming AEAD: ChaCha20-Poly1305 (ring crate)
- Codec: Opus (audio), WebP/AVIF (images, client-side)
- Max chunk size: 64KB
- Media routed through Scrambler (fragmented across paths)

## Dependencies
- Epic 00 (Foundation)
- Epic 01 (Messaging)
- Epic 03 (Scrambler)

## Architecture References
- [cryptography.md](../architecture/cryptography.md)
- [data-models.md](../architecture/data-models.md)
- [burn-rooms.md](../architecture/burn-rooms.md)

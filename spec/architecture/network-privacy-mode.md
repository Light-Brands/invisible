# Network Privacy Mode — Cellular Metadata Protection

> **Purpose:** Protect against cellular network metadata leakage (IMSI/IMEI tracking, phone
> number linkage, location tracking) that bypasses VPN/encryption protections.

---

## Table of Contents

1. [Threat Model](#threat-model)
2. [Cellular Metadata Leakage](#cellular-metadata-leakage)
3. [WiFi-Only Mode](#wifi-only-mode)
4. [eSIM Rotation](#esim-rotation)
5. [MAC Address Randomization](#mac-address-randomization)
6. [Airplane Mode + WiFi](#airplane-mode--wifi)
7. [Implementation Specification](#implementation-specification)
8. [User Interface](#user-interface)
9. [Threat Mitigation Summary](#threat-mitigation-summary)

---

## Threat Model

### What Ghost VPN Protects Against ✅

- ✅ **Content surveillance** - ISP/carrier can't read messages
- ✅ **DNS leaks** - All DNS queries through VPN
- ✅ **IP address exposure** - Real IP hidden behind VPN
- ✅ **DPI (Deep Packet Inspection)** - Traffic encrypted + obfuscated

### What Ghost VPN CANNOT Protect Against ⚠️

- ⚠️ **IMSI/IMEI tracking** - Cellular network sees hardware/SIM identifiers
- ⚠️ **Phone number linkage** - SIM card tied to subscriber identity
- ⚠️ **Cell tower triangulation** - Physical location revealed
- ⚠️ **Traffic timing metadata** - When device transmits data
- ⚠️ **VPN usage fingerprinting** - Carrier knows you use VPN

### Adversary Capabilities

**Cellular Carrier (Passive):**
- Logs all IMSI/IMEI connections
- Records cell tower locations
- Knows which VPN endpoints you connect to
- Can see traffic timing patterns (even with cover traffic)
- May share data with government/law enforcement

**Nation-State (Active):**
- Can compel carrier to provide subscriber information
- Can perform real-time IMSI tracking (IMSI catchers)
- Can correlate cellular metadata with internet traffic
- Can identify VPN users for enhanced surveillance

---

## Cellular Metadata Leakage

### Identifiers Exposed Over Cellular

```
┌──────────────────────────────────────────────────┐
│  CELLULAR CONNECTION EXPOSES:                    │
├──────────────────────────────────────────────────┤
│  1. IMSI (SIM card identifier)                   │
│     - Unique per SIM                             │
│     - Visible to carrier at all times            │
│     - Survives phone reboots                     │
│     - Can be tracked across sessions             │
│                                                  │
│  2. IMEI (phone hardware identifier)             │
│     - Unique per device                          │
│     - Visible to carrier at all times            │
│     - Survives SIM changes                       │
│     - Device fingerprint                         │
│                                                  │
│  3. Phone Number                                 │
│     - Tied to subscriber identity                │
│     - Billing records, KYC documents             │
│     - Direct link to real name/address           │
│                                                  │
│  4. Location                                     │
│     - Cell tower triangulation (50-200m)         │
│     - Continuous tracking while connected        │
│     - Movement patterns over time                │
│                                                  │
│  5. Network Activity Timing                      │
│     - When device transmits/receives data        │
│     - Volume of data (approximate)               │
│     - VPN endpoint IP address                    │
│     - Connection start/end times                 │
└──────────────────────────────────────────────────┘
```

### Attack Scenario: Targeted Surveillance

```
Government adversary wants to monitor Alice.

Step 1: Identify Alice's phone number
    → +1-555-123-4567

Step 2: Request carrier records
    → IMSI: 310-410-123456789
    → IMEI: 35-123456-789012-3

Step 3: Real-time monitoring
    → Alice's phone connects to VPN at 203.0.113.50
    → Location: Cell tower 39847 (Downtown, 1st & Main)
    → Online 24/7 (constant VPN connection)
    → Flagged as "privacy tool user"

Step 4: Enhanced surveillance
    → Alice is now on watchlist
    → All movements tracked via cell towers
    → VPN endpoint logged for internet correlation
    → Can perform IMSI catcher attack if needed

Even though messages are encrypted:
- Alice's identity is known (phone number → billing)
- Alice's location is known (cell towers)
- Alice's VPN usage is known (suspicious behavior)
- Alice is targeted for further investigation
```

---

## WiFi-Only Mode

### Purpose

Disable cellular data completely, allowing network connectivity only over WiFi. This prevents
cellular carriers from seeing any network activity or location data.

### How It Works

```
┌─────────────────────────────────────────────────┐
│  WiFi-Only Mode ENABLED                         │
├─────────────────────────────────────────────────┤
│                                                 │
│  [×] Cellular data DISABLED                     │
│  [✓] WiFi ENABLED                               │
│  [✓] Ghost VPN auto-connects over WiFi         │
│                                                 │
│  Network Path:                                  │
│  Device → WiFi → ISP → Ghost VPN → Internet     │
│                                                 │
│  Cellular carrier sees:                         │
│  - Phone is registered on network (for calls)   │
│  - NO data traffic                              │
│  - NO VPN connections                           │
│  - NO location updates (if in airplane mode)    │
└─────────────────────────────────────────────────┘
```

### Benefits

✅ **No cellular metadata leakage**
- Carrier doesn't see VPN connections
- No data traffic timing patterns
- No VPN usage fingerprinting

✅ **Location privacy** (with airplane mode)
- No cell tower pings
- WiFi location less precise than cellular
- Can use public WiFi for anonymity

✅ **MAC address randomization** (iOS/Android)
- Randomized WiFi MAC per network
- Breaks WiFi-based tracking
- Different identity per connection

✅ **Public WiFi anonymity**
- No link to subscriber identity
- Can use without authentication (many public networks)
- Additional layer between device and internet

### Limitations

⚠️ **Reduced availability**
- Only works where WiFi is available
- Must find/connect to WiFi networks
- May require authentication (captive portals)

⚠️ **Home WiFi still trackable**
- Home internet subscriber identity known
- ISP can see VPN connections (same as cellular)
- But: No phone number linkage, no IMSI/IMEI

⚠️ **WiFi access point tracking**
- Access points can log MAC addresses (even randomized ones)
- Captive portals may require identity
- Public WiFi may have cameras, physical surveillance

### Recommended Use Cases

- **Home WiFi:** Prevents cellular carrier surveillance, but ISP still sees VPN
- **Public WiFi:** Best anonymity - no identity link, MAC randomization, physical anonymity
- **Coffee shops, libraries, airports:** High anonymity, but physical surveillance risk
- **Work WiFi:** Prevents cellular metadata, but employer can monitor

---

## eSIM Rotation

### Purpose

Periodically rotate eSIM profiles to obtain new IMSI/phone number, breaking long-term
cellular tracking even when using cellular data.

### How It Works

```
┌──────────────────────────────────────────────────┐
│  eSIM ROTATION STRATEGY                          │
├──────────────────────────────────────────────────┤
│                                                  │
│  User has 3 eSIM profiles installed:             │
│                                                  │
│  Profile A: Carrier X, IMSI-001, +1-555-111     │
│  Profile B: Carrier Y, IMSI-002, +1-555-222     │
│  Profile C: Carrier Z, IMSI-003, +1-555-333     │
│                                                  │
│  Rotation Schedule (configurable):               │
│  - Manual (user taps "Rotate eSIM")              │
│  - Automatic per session                         │
│  - Automatic daily                               │
│  - Automatic weekly                              │
│                                                  │
│  On rotation:                                    │
│  1. Disconnect from current network              │
│  2. Switch to next eSIM profile                  │
│  3. Reconnect to cellular network                │
│  4. Ghost VPN auto-reconnects                    │
│  5. New IMSI, new phone number, new carrier      │
│                                                  │
│  Carrier correlation broken:                     │
│  Monday:    Carrier X sees IMSI-001 using VPN    │
│  Tuesday:   Carrier Y sees IMSI-002 using VPN    │
│  Wednesday: Carrier Z sees IMSI-003 using VPN    │
│                                                  │
│  Each carrier thinks it's a different user!      │
└──────────────────────────────────────────────────┘
```

### eSIM Profile Management

**Anonymous eSIM Acquisition:**
```
Recommended eSIM providers (no KYC):
- Silent.link (crypto payment, no KYC)
- Hushed (prepaid, minimal info)
- Airalo (international, email only)
- Nomad (pay with crypto option)

Best practice:
- Purchase with Monero (privacy coin)
- Use unique email per eSIM (burner emails)
- Rotate providers (don't use same provider for all profiles)
- Delete profile after rotation period
```

**Rotation Strategies:**

| Strategy | Frequency | Anonymity | Cost | Use Case |
|----------|-----------|-----------|------|----------|
| **Paranoid** | Per session | Maximum | High | Journalists, activists |
| **High** | Daily | Very high | Medium | Privacy-conscious users |
| **Moderate** | Weekly | High | Low | Standard privacy |
| **Low** | Monthly | Moderate | Very low | Casual privacy |
| **Manual** | User-triggered | Variable | Variable | Situational privacy |

**Per-Session Rotation:**
```
App launch → Check eSIM age
If current eSIM used > 1 session:
    → Auto-rotate to next profile
    → New IMSI every app session

Effect:
- No single carrier sees persistent usage pattern
- Each session appears to be different user
- Maximum cellular anonymity
```

### Implementation

```rust
pub struct ESIMManager {
    profiles: Vec<ESIMProfile>,
    current_index: usize,
    rotation_strategy: RotationStrategy,
}

pub struct ESIMProfile {
    id: String,
    carrier: String,
    imsi: String,
    phone_number: Option<String>,
    activated_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    data_remaining: Option<u64>,  // bytes
}

pub enum RotationStrategy {
    Manual,
    PerSession,
    Daily,
    Weekly,
    Monthly,
}

impl ESIMManager {
    /// Rotate to next eSIM profile
    pub async fn rotate_esim(&mut self) -> Result<()> {
        // 1. Disconnect from current network
        self.disconnect_cellular().await?;

        // 2. Switch to next profile (round-robin)
        self.current_index = (self.current_index + 1) % self.profiles.len();
        let next_profile = &self.profiles[self.current_index];

        // 3. Activate next profile
        self.activate_esim_profile(next_profile).await?;

        // 4. Wait for cellular connection
        self.wait_for_cellular_connection().await?;

        // 5. Ghost VPN will auto-reconnect (existing logic)

        Ok(())
    }

    /// Check if rotation is needed based on strategy
    pub fn needs_rotation(&self) -> bool {
        let current = &self.profiles[self.current_index];

        match self.rotation_strategy {
            RotationStrategy::Manual => false,
            RotationStrategy::PerSession => {
                // Rotate if current eSIM was used in previous session
                current.last_used < get_current_session_start()
            }
            RotationStrategy::Daily => {
                current.activated_at + Duration::days(1) < Utc::now()
            }
            RotationStrategy::Weekly => {
                current.activated_at + Duration::weeks(1) < Utc::now()
            }
            RotationStrategy::Monthly => {
                current.activated_at + Duration::weeks(4) < Utc::now()
            }
        }
    }

    /// Auto-rotate on app launch if needed
    pub async fn auto_rotate_if_needed(&mut self) -> Result<()> {
        if self.needs_rotation() {
            info!("Auto-rotating eSIM based on strategy: {:?}", self.rotation_strategy);
            self.rotate_esim().await?;
        }
        Ok(())
    }
}
```

### Benefits

✅ **Breaks long-term IMSI tracking**
- New IMSI every rotation
- Carrier can't build persistent profile

✅ **Breaks phone number linkage**
- New phone number every rotation
- No single number tied to identity

✅ **Cross-carrier anonymity**
- Different carriers per profile
- No single carrier sees full activity

✅ **Defeats IMSI catchers** (partially)
- IMSI catcher captures current IMSI
- After rotation, that IMSI is no longer used
- Requires fresh IMSI capture

### Limitations

⚠️ **IMEI still persistent**
- Same hardware identifier across rotations
- Sophisticated adversary can track via IMEI
- Solution: Use with airplane mode + WiFi when possible

⚠️ **Cost**
- Multiple eSIM subscriptions required
- Data plans for each profile
- Can use low-cost prepaid/data-only eSIMs

⚠️ **Availability**
- Requires eSIM-capable device (iPhone XS+, newer Androids)
- Some carriers don't support eSIM
- Regional availability varies

---

## MAC Address Randomization

### Purpose

Prevent WiFi access point tracking by using randomized MAC addresses per network.

### How It Works

```
Traditional WiFi Connection:
Device MAC: AA:BB:CC:DD:EE:FF (persistent)
    ↓
Every WiFi network sees same MAC
    ↓
Can track device across networks/locations

With MAC Randomization:
Network A sees MAC: 11:22:33:44:55:66
Network B sees MAC: AA:BB:CC:DD:EE:FF
Network C sees MAC: 77:88:99:AA:BB:CC
    ↓
Each network sees different "device"
    ↓
Can't correlate connections across networks
```

### Platform Support

**iOS (Settings > Wi-Fi > [Network] > Private Wi-Fi Address):**
- Enabled by default on iOS 14+
- Randomizes MAC per WiFi network
- Consistent MAC per network (not per session)
- Can disable per-network if needed

**Android (Settings > Network & Internet > Wi-Fi > Privacy):**
- "Use randomized MAC" option
- Android 10+ supports per-network randomization
- Android 12+ enhanced randomization

### Invisible Integration

```rust
pub struct WiFiPrivacyManager {
    mac_randomization_enabled: bool,
}

impl WiFiPrivacyManager {
    /// Ensure MAC randomization is enabled
    pub async fn enforce_mac_randomization(&self) -> Result<()> {
        #[cfg(target_os = "ios")]
        {
            // Check if Private Wi-Fi Address is enabled
            if !self.is_mac_randomization_enabled_ios()? {
                warn!("MAC randomization disabled - prompting user");
                self.prompt_enable_mac_randomization_ios()?;
            }
        }

        #[cfg(target_os = "android")]
        {
            // Check if randomized MAC is enabled
            if !self.is_mac_randomization_enabled_android()? {
                warn!("MAC randomization disabled - prompting user");
                self.prompt_enable_mac_randomization_android()?;
            }
        }

        Ok(())
    }

    /// Check on WiFi connection
    pub async fn on_wifi_connected(&self, ssid: &str) -> Result<()> {
        // Verify MAC randomization for this network
        self.enforce_mac_randomization().await?;

        // Verify MAC is randomized (not factory MAC)
        let current_mac = self.get_current_mac_address()?;
        let factory_mac = self.get_factory_mac_address()?;

        if current_mac == factory_mac {
            warn!("Using factory MAC - privacy leak!");
            self.alert_user_mac_not_randomized(ssid)?;
        }

        Ok(())
    }
}
```

---

## Airplane Mode + WiFi

### Purpose

**Maximum cellular anonymity:** Disable cellular radio entirely while maintaining WiFi
connectivity. This prevents all cellular metadata leakage.

### How It Works

```
┌────────────────────────────────────────────────┐
│  AIRPLANE MODE + WiFi                          │
├────────────────────────────────────────────────┤
│                                                │
│  [×] Cellular radio DISABLED                   │
│       - No IMSI broadcast                      │
│       - No IMEI transmission                   │
│       - No cell tower connections              │
│       - No location tracking                   │
│                                                │
│  [✓] WiFi ENABLED                              │
│       - Can connect to WiFi networks           │
│       - MAC address randomized                 │
│       - Ghost VPN works normally               │
│                                                │
│  [×] Bluetooth DISABLED (optional)             │
│  [×] NFC DISABLED (optional)                   │
│                                                │
│  Network Path:                                 │
│  Device → WiFi → ISP → Ghost VPN → Internet    │
│                                                │
│  Cellular carrier sees:                        │
│  - Nothing (device not registered on network)  │
└────────────────────────────────────────────────┘
```

### Benefits

✅ **Complete cellular anonymity**
- Zero cellular metadata
- No IMSI/IMEI exposure
- No cell tower triangulation
- No phone number linkage

✅ **Battery savings**
- Cellular radio is power-hungry
- Airplane mode extends battery life

✅ **Cannot be tracked via cellular**
- IMSI catchers can't capture IMSI (radio off)
- Carrier can't log location
- No cellular coercion possible

### Limitations

⚠️ **No cellular calls/SMS**
- Cannot receive regular phone calls
- Cannot receive SMS (unless carrier offers WiFi calling)
- Can use VoIP for calls (through Invisible voice calls)

⚠️ **Requires WiFi**
- Only works where WiFi is available
- No connectivity when traveling without WiFi

### User Experience

```
When user enables "Airplane Mode + WiFi":

1. App prompts:
   ┌──────────────────────────────────────────┐
   │  Enable Airplane Mode + WiFi?            │
   │                                          │
   │  This will:                              │
   │  ✓ Disable cellular radio                │
   │  ✓ Enable WiFi only                      │
   │  ✓ Prevent all cellular tracking         │
   │                                          │
   │  You will NOT be able to:                │
   │  × Receive regular phone calls           │
   │  × Receive SMS messages                  │
   │  × Use cellular data                     │
   │                                          │
   │  Invisible calls/messages work normally. │
   │                                          │
   │  [Cancel]  [Enable]                      │
   └──────────────────────────────────────────┘

2. App configures device:
   - Enables airplane mode
   - Re-enables WiFi
   - Ensures MAC randomization
   - Connects Ghost VPN over WiFi

3. App shows status:
   ┌──────────────────────────────────────────┐
   │  🛫 Airplane Mode + WiFi                 │
   │  ✓ Cellular radio disabled               │
   │  ✓ Connected to WiFi: "Starbucks WiFi"   │
   │  ✓ Ghost VPN connected                   │
   │  ✓ Maximum privacy mode active           │
   └──────────────────────────────────────────┘
```

---

## Implementation Specification

### Network Privacy Settings

```rust
pub struct NetworkPrivacySettings {
    /// Network connectivity mode
    pub mode: NetworkMode,

    /// eSIM rotation strategy
    pub esim_rotation: RotationStrategy,

    /// Auto-enable WiFi-only in certain contexts
    pub auto_wifi_only: AutoWiFiOnlySettings,

    /// MAC randomization enforcement
    pub enforce_mac_randomization: bool,
}

pub enum NetworkMode {
    /// Normal mode: Use cellular or WiFi
    Normal,

    /// WiFi-only: Disable cellular data, use WiFi only
    WiFiOnly,

    /// Airplane + WiFi: Disable cellular radio entirely
    AirplaneModeWithWiFi,

    /// eSIM rotation: Use cellular with periodic eSIM rotation
    CellularWithESIMRotation,
}

pub struct AutoWiFiOnlySettings {
    /// Auto-enable WiFi-only when at home
    pub at_home: bool,

    /// Auto-enable WiFi-only when at saved locations
    pub at_saved_locations: bool,

    /// Auto-enable WiFi-only when on known WiFi networks
    pub on_known_networks: bool,
}
```

### Mode Switching

```rust
pub struct NetworkPrivacyManager {
    settings: NetworkPrivacySettings,
    esim_manager: ESIMManager,
    wifi_manager: WiFiPrivacyManager,
}

impl NetworkPrivacyManager {
    /// Switch network mode
    pub async fn set_network_mode(&mut self, mode: NetworkMode) -> Result<()> {
        match mode {
            NetworkMode::Normal => {
                self.enable_cellular().await?;
                self.enable_wifi().await?;
            }

            NetworkMode::WiFiOnly => {
                self.disable_cellular_data().await?;
                self.enable_wifi().await?;
                self.wifi_manager.enforce_mac_randomization().await?;
            }

            NetworkMode::AirplaneModeWithWiFi => {
                self.enable_airplane_mode().await?;
                self.enable_wifi().await?;
                self.wifi_manager.enforce_mac_randomization().await?;
            }

            NetworkMode::CellularWithESIMRotation => {
                self.enable_cellular().await?;
                self.esim_manager.auto_rotate_if_needed().await?;
            }
        }

        // Ghost VPN auto-reconnects
        self.reconnect_ghost_vpn().await?;

        Ok(())
    }

    /// Check and auto-switch based on context
    pub async fn auto_switch_if_needed(&mut self) -> Result<()> {
        if self.settings.auto_wifi_only.on_known_networks {
            if self.is_on_known_wifi_network()? {
                self.set_network_mode(NetworkMode::WiFiOnly).await?;
            }
        }

        // Additional auto-switching logic...

        Ok(())
    }
}
```

---

## User Interface

### Settings Screen

```
┌────────────────────────────────────────────────┐
│  ⚙️ Network Privacy                            │
├────────────────────────────────────────────────┤
│                                                │
│  Network Mode                                  │
│  ○ Normal (cellular + WiFi)                    │
│  ● WiFi Only                              ◀─── │
│  ○ Airplane Mode + WiFi (maximum privacy)      │
│  ○ Cellular with eSIM Rotation                 │
│                                                │
│  ────────────────────────────────────────────  │
│                                                │
│  eSIM Rotation (requires eSIM profiles)        │
│  Strategy: [Daily ▼]                           │
│  Profiles installed: 3                         │
│  Current: Profile A (Carrier X)                │
│  Last rotated: 18 hours ago                    │
│  [Rotate Now]                                  │
│                                                │
│  ────────────────────────────────────────────  │
│                                                │
│  MAC Address Randomization                     │
│  [✓] Enforce MAC randomization on WiFi         │
│  [✓] Alert if factory MAC detected             │
│                                                │
│  ────────────────────────────────────────────  │
│                                                │
│  Auto WiFi-Only Mode                           │
│  [✓] Enable at home                            │
│  [✓] Enable at saved locations                 │
│  [ ] Enable on all known WiFi networks         │
│                                                │
│  ────────────────────────────────────────────  │
│                                                │
│  Current Status                                │
│  Connection: WiFi ("Home Network")             │
│  MAC: Randomized ✓                             │
│  Ghost VPN: Connected ✓                        │
│  Cellular tracking: BLOCKED ✓                  │
│                                                │
└────────────────────────────────────────────────┘
```

### Quick Toggle (Status Bar)

```
┌────────────────────────────────────────────────┐
│  [≡] Invisible              🛫 WiFi Only    [⚙]│
└────────────────────────────────────────────────┘
         ▲                           ▲
         │                           └─ Quick toggle: tap to change mode
         └─ Menu
```

Tap "WiFi Only" icon:
```
┌────────────────────────────────────┐
│  Network Mode                      │
├────────────────────────────────────┤
│  ○ Normal                          │
│  ● WiFi Only                       │
│  ○ Airplane + WiFi                 │
│  ○ Cellular + eSIM Rotation        │
└────────────────────────────────────┘
```

---

## Threat Mitigation Summary

| Threat | Normal Mode | WiFi-Only | Airplane+WiFi | eSIM Rotation |
|--------|-------------|-----------|---------------|---------------|
| **Content surveillance** | ✓ VPN | ✓ VPN | ✓ VPN | ✓ VPN |
| **IP address leak** | ✓ VPN | ✓ VPN | ✓ VPN | ✓ VPN |
| **IMSI tracking** | ⚠️ Exposed | ⚠️ Exposed | ✓ Hidden | ✓ Rotated |
| **IMEI tracking** | ⚠️ Exposed | ⚠️ Exposed | ✓ Hidden | ⚠️ Exposed |
| **Phone number linkage** | ⚠️ Exposed | ⚠️ Exposed | ✓ Hidden | ✓ Rotated |
| **Cell tower location** | ⚠️ Tracked | ⚠️ Tracked | ✓ Hidden | ⚠️ Tracked |
| **VPN usage fingerprint** | ⚠️ Visible | ⚠️ Visible | ✓ Hidden | ⚠️ Visible |
| **Long-term cellular profiling** | ⚠️ Possible | ⚠️ Possible | ✓ Blocked | ✓ Blocked |
| **WiFi access point tracking** | N/A | ⚠️ If not randomized | ✓ Randomized | N/A |
| **ISP surveillance** | ⚠️ See VPN usage | ⚠️ See VPN usage | ⚠️ See VPN usage | ⚠️ See VPN usage |

**Legend:**
- ✓ = Protected
- ⚠️ = Vulnerable
- N/A = Not applicable

### Recommended Configurations by Threat Model

**Journalist / Activist:**
```
Mode: Airplane Mode + WiFi
eSIM: N/A (cellular disabled)
MAC: Randomized
Public WiFi: Yes (different locations)

Why: Complete cellular anonymity, use only public WiFi with no identity link
```

**Privacy-Conscious User:**
```
Mode: WiFi-Only at home, eSIM Rotation when mobile
eSIM: Daily rotation
MAC: Randomized
Auto-switch: Enable at home

Why: WiFi at home (no cellular tracking), eSIM rotation when traveling
```

**Standard User:**
```
Mode: Cellular with eSIM Rotation
eSIM: Weekly rotation
MAC: Randomized

Why: Normal connectivity, periodic eSIM rotation breaks long-term tracking
```

**Maximum Paranoia:**
```
Mode: Airplane Mode + WiFi ONLY
eSIM: Disabled
MAC: Randomized
Public WiFi: Only (no home WiFi)
Location: Different coffee shops/libraries daily

Why: Zero cellular exposure, no home internet link, physical location variety
```

---

## Integration with Existing Architecture

### Ghost VPN Compatibility

Network Privacy Mode works seamlessly with Ghost VPN:

```
User device:
  [Network Privacy Mode] → Choose connectivity method
            ↓
  [WiFi or Cellular or Airplane+WiFi]
            ↓
  [Ghost VPN] → Auto-connects over chosen method
            ↓
  [Scrambler] → Full 7-layer protection
            ↓
  Internet
```

All Scrambler protections remain active regardless of network mode.

### No Changes Required to Scrambler

- Scrambler operates identically over WiFi or cellular
- Cover traffic, mixnet, dead drops all work the same
- Network Privacy Mode is **orthogonal** to Scrambler
- Adds metadata protection at network layer below Scrambler

---

## Cross-References

- [ghost-vpn.md](ghost-vpn.md) - Layer 0: VPN architecture
- [scrambler.md](scrambler.md) - Layers 1-7: Network obfuscation
- [shadow-wallet-hardening.md](shadow-wallet-hardening.md) - Wallet security
- [zero-log-doctrine.md](zero-log-doctrine.md) - Data retention policy

---

*Network Privacy Mode closes the cellular metadata gap, ensuring that not even your
network carrier can track your identity, location, or Invisible usage. Combined with the
Ghost VPN and Scrambler, you achieve complete network anonymity from device to destination.*

# Infrastructure Deployment Guide

**How to deploy the actual servers that make Invisible work.**

## Overview: What Needs to Run

```
┌─────────────────────────────────────────┐
│         YOUR CLIENT APP                 │  ← This is your code
│  (phone/desktop running invisible)      │
└────────────┬────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────┐
│    INFRASTRUCTURE (Must Deploy)         │
│  ┌────────────────────────────────┐    │
│  │ 1. VPN Servers                 │    │  ← WireGuard endpoints
│  │ 2. Mix Node Relays (5 layers)  │    │  ← Your relay code
│  │ 3. Dead Drop Relays            │    │  ← Your relay code
│  └────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

## Component 1: VPN Servers (Layer 0)

### ✅ EASIEST: Use Existing VPN Service (Recommended for Testing)

**Providers that support WireGuard:**
- **Mullvad** (privacy-focused, accepts crypto): https://mullvad.net
- **IVPN** (privacy-focused): https://ivpn.net
- **ProtonVPN** (supports WireGuard): https://protonvpn.com

**Setup (5 minutes):**

```bash
# 1. Sign up for Mullvad (example)
# Download their WireGuard config

# 2. Extract connection details
ENDPOINT=$(grep "Endpoint" mullvad-config.conf | cut -d' ' -f3)
PUBLIC_KEY=$(grep "PublicKey" mullvad-config.conf | cut -d' ' -f3)

# 3. Use in your Invisible config:
```

```rust
VpnConfig {
    endpoints: vec![
        VpnEndpoint {
            public_key: PUBLIC_KEY.as_bytes().to_vec(),
            address: ENDPOINT.parse()?,
            location: "NL-Amsterdam".to_string(),
            latency_ms: None,
        }
    ],
    // Your private key
    private_key: your_private_key,
    local_address: "10.0.0.2/24".to_string(),
    keepalive_interval: 25,
    max_session_time: 3600,
}
```

**✅ Done! Your VPN layer works immediately.**

### 🔧 ALTERNATIVE: Deploy Your Own VPN Server

**Cost:** $5-10/month (1 server)
**Time:** 30 minutes

**Deploy on cloud provider:**

```bash
# 1. Create a server (DigitalOcean/Vultr/AWS)
# OS: Ubuntu 22.04
# Size: $5/month droplet (1GB RAM)
# Location: Choose different jurisdictions for each server

# 2. SSH into server
ssh root@your-server-ip

# 3. Install WireGuard
apt update
apt install -y wireguard

# 4. Generate keys
wg genkey | tee /etc/wireguard/server_private.key | wg pubkey | tee /etc/wireguard/server_public.key
chmod 600 /etc/wireguard/server_private.key

# 5. Configure server
cat > /etc/wireguard/wg0.conf <<EOF
[Interface]
Address = 10.0.0.1/24
ListenPort = 51820
PrivateKey = $(cat /etc/wireguard/server_private.key)
PostUp = iptables -A FORWARD -i wg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE

# Add peers (your clients) here later
EOF

# 6. Enable IP forwarding
echo "net.ipv4.ip_forward=1" >> /etc/sysctl.conf
sysctl -p

# 7. Start VPN server
wg-quick up wg0
systemctl enable wg-quick@wg0

# 8. Open firewall
ufw allow 51820/udp
ufw allow 22/tcp  # Keep SSH
ufw enable

# 9. Get server public key for your client
cat /etc/wireguard/server_public.key
```

**Repeat for 3-5 servers in different locations** (US, EU, Asia)

---

## Component 2: Mix Node Relays (Layer 2) - **REQUIRED**

### What They Do
- Receive Sphinx packets
- Batch and shuffle for timing obfuscation
- Forward to next hop
- Generate cover traffic

### Minimum Deployment: 3 servers (basic privacy)
### Recommended: 5 servers (full 5-layer mixnet)

**Cost:** $15-50/month ($5-10 per server)
**Time:** 1-2 hours total

### Deploy Mix Node Relay

**Per server:**

```bash
# 1. Create cloud server
# OS: Ubuntu 22.04
# Size: $10/month (2GB RAM recommended for relay)
# Locations: Spread across jurisdictions
#   - Layer 0 (Entry): Netherlands
#   - Layer 1: Germany
#   - Layer 2: Switzerland
#   - Layer 3: Iceland
#   - Layer 4 (Exit): Singapore

# 2. SSH into server
ssh root@mix-node-server

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. Install dependencies
apt update
apt install -y build-essential pkg-config libssl-dev

# 5. Copy your relay code
# From your local machine:
```

```bash
# On your local machine:
cd invisible/

# Build relay binary
cargo build --release --bin invisible-relay

# Copy to server
scp target/release/invisible-relay root@mix-node-server:/usr/local/bin/

# Copy config
scp relay-config.toml root@mix-node-server:/etc/invisible/
```

```bash
# 6. On server: Create systemd service
cat > /etc/systemd/system/invisible-relay.service <<EOF
[Unit]
Description=Invisible Mix Node Relay
After=network.target

[Service]
Type=simple
User=invisible
Group=invisible
ExecStart=/usr/local/bin/invisible-relay --config /etc/invisible/relay-config.toml
Restart=always
RestartSec=10

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/invisible

[Install]
WantedBy=multi-user.target
EOF

# 7. Create user
useradd -r -s /bin/false invisible
mkdir -p /var/lib/invisible
chown invisible:invisible /var/lib/invisible

# 8. Create config file
cat > /etc/invisible/relay-config.toml <<EOF
[node]
layer = 0  # Change for each layer (0-4)
listen_address = "0.0.0.0:9001"
public_address = "$(curl -s ifconfig.me):9001"

[mixing]
batch_size = 10
flush_interval_ms = 5000

[logging]
level = "info"
EOF

# 9. Start relay
systemctl daemon-reload
systemctl enable invisible-relay
systemctl start invisible-relay

# 10. Check status
systemctl status invisible-relay
journalctl -u invisible-relay -f

# 11. Open firewall
ufw allow 9001/tcp
ufw allow 9001/udp
```

**Repeat for 5 servers (one per layer)**

---

## Component 3: Dead Drop Relays (Layer 6) - **REQUIRED**

### What They Do
- Store encrypted message fragments temporarily
- Provide anonymous retrieval with access tokens
- Expire messages after 24 hours (RAM only)

**Cost:** $15-30/month (3 servers recommended)
**Time:** 30 minutes per server

### Deploy Dead Drop Relay

```bash
# 1. Create cloud server (same as mix node)
# 2. Install Rust + deps (same as mix node)
# 3. Build and deploy dead drop binary

# On your local machine:
cargo build --release --bin invisible-dead-drop
scp target/release/invisible-dead-drop root@dead-drop-server:/usr/local/bin/

# 4. On server: Create systemd service
cat > /etc/systemd/system/invisible-dead-drop.service <<EOF
[Unit]
Description=Invisible Dead Drop Relay
After=network.target

[Service]
Type=simple
User=invisible
Group=invisible
ExecStart=/usr/local/bin/invisible-dead-drop --config /etc/invisible/dead-drop-config.toml
Restart=always
RestartSec=10

# RAM-only storage (critical for privacy)
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
NoNewPrivileges=true

[Install]
WantedBy=multi-user.target
EOF

# 5. Create config
cat > /etc/invisible/dead-drop-config.toml <<EOF
[server]
listen_address = "0.0.0.0:9002"

[storage]
max_capacity = 10000  # Max messages to store
expiration_hours = 24
cleanup_interval_minutes = 60

[logging]
level = "info"
EOF

# 6. Start service
systemctl daemon-reload
systemctl enable invisible-dead-drop
systemctl start invisible-dead-drop
systemctl status invisible-dead-drop

# 7. Open firewall
ufw allow 9002/tcp
```

**Repeat for 3 servers (different locations)**

---

## Quick Start: Minimum Viable Infrastructure

**Total cost:** ~$30/month
**Total time:** 2-3 hours

### Option A: Testing Setup (Cheapest)

1. **VPN:** Use Mullvad ($5/month) ✅ 5 minutes
2. **Mix Nodes:** Deploy 3 servers ($15/month) ⏱️ 1 hour
3. **Dead Drops:** Deploy 1 server ($5/month) ⏱️ 20 minutes

### Option B: Production Setup (Recommended)

1. **VPN:** Deploy 3 servers ($15/month) ⏱️ 1 hour
2. **Mix Nodes:** Deploy 5 servers ($25-50/month) ⏱️ 2 hours
3. **Dead Drops:** Deploy 3 servers ($15-30/month) ⏱️ 1 hour

---

## After Deployment: Configure Your Client

Once servers are running, update your client config:

```rust
// In your client code:
let config = ScramblerConfig {
    vpn: VpnConfig {
        endpoints: vec![
            VpnEndpoint {
                public_key: vpn_server_pubkey,
                address: "your-vpn-1.example.com:51820".parse()?,
                location: "NL-Amsterdam".to_string(),
            },
            VpnEndpoint {
                public_key: vpn_server_2_pubkey,
                address: "your-vpn-2.example.com:51820".parse()?,
                location: "US-NewYork".to_string(),
            },
        ],
        // Your client private key
        private_key: client_private_key,
        local_address: "10.0.0.2/24".to_string(),
        keepalive_interval: 25,
        max_session_time: 3600,
    },
    // ... other config
};

// Define your mix nodes
let mix_nodes = vec![
    MixNode {
        id: generate_node_id(),
        layer: Layer(0),
        public_key: mix_node_0_pubkey,
        address: "mix0.example.com:9001".to_string(),
        location: GeoLocation {
            country: "NL".to_string(),
            jurisdiction: Jurisdiction::EU,
        },
    },
    MixNode {
        id: generate_node_id(),
        layer: Layer(1),
        public_key: mix_node_1_pubkey,
        address: "mix1.example.com:9001".to_string(),
        location: GeoLocation {
            country: "DE".to_string(),
            jurisdiction: Jurisdiction::EU,
        },
    },
    // ... layers 2, 3, 4
];

// Create scrambler with your deployed infrastructure
let scrambler = Scrambler::new(config, mix_nodes);
scrambler.initialize().await?;
```

---

## Verification Checklist

After deployment, verify each component:

```bash
# VPN Server
ssh root@vpn-server "wg show"
# Should show interface and peers

# Mix Node Relay
ssh root@mix-node-0 "systemctl status invisible-relay"
ssh root@mix-node-0 "journalctl -u invisible-relay -n 50"
# Should show "listening on 0.0.0.0:9001"

# Dead Drop Relay
ssh root@dead-drop-1 "systemctl status invisible-dead-drop"
# Should show "listening on 0.0.0.0:9002"

# Network connectivity (from your machine)
nc -zv mix0.example.com 9001  # Should connect
nc -zv dead-drop1.example.com 9002  # Should connect
```

---

## Cloud Provider Comparison

| Provider | Cost/month | Pros | Cons |
|----------|-----------|------|------|
| **DigitalOcean** | $5-10/server | Simple, fast setup | US-based |
| **Vultr** | $5-10/server | Global locations | Fewer privacy features |
| **Hetzner** | $4-8/server | EU-based, cheap | Germany privacy laws |
| **Mullvad VPS** | $6/server | Privacy-focused | Limited locations |

---

## Automation: Deploy All at Once

I can create deployment scripts to automate this. Would you like:

1. **Terraform scripts** - Define infrastructure as code
2. **Ansible playbooks** - Automated server configuration
3. **Docker compose** - Containerized deployment
4. **Shell scripts** - Simple bash automation

---

## Summary

**What you need to deploy:**

1. ✅ **VPN Servers** - Use Mullvad OR deploy 3-5 WireGuard servers
2. ✅ **Mix Node Relays** - Deploy 3-5 servers running your relay code
3. ✅ **Dead Drop Relays** - Deploy 3 servers running your dead drop code

**Your code is ready** - it just needs these servers to connect to!

**Minimum to get started:**
- 1 VPN endpoint (Mullvad subscription)
- 3 mix nodes ($15/month)
- 1 dead drop ($5/month)
**Total: $25/month + Mullvad**

Want me to create automated deployment scripts for you?

# Invisible Network Infrastructure - Production Deployment Guide

**Version:** 1.0
**Date:** 2026-02-14
**Target:** Production-grade relay/mix network with maximum privacy and resilience
**Estimated Time:** 4-6 hours for initial deployment
**Monthly Cost:** ~$200-300 USD (scalable)

---

## 🎯 Deployment Overview

This guide will deploy a **production-ready, multi-jurisdictional, privacy-maximized relay network** for Invisible's message routing infrastructure.

### Architecture Summary

```
User Device
    ↓
Ghost VPN (WireGuard Entry)
    ↓
[Entry Nodes] - Njalla (3x instances across jurisdictions)
    ↓
[Mix Core] - Akash Network (15x instances, decentralized)
    ↓
[Exit Nodes] - Njalla + FlokiNET (5x instances)
    ↓
[Dead Drop Storage] - Storj DCS (decentralized, encrypted)
    ↓
Monitoring - Self-hosted on Akash (Tor hidden service)
```

### Key Principles
- ✅ **Zero identity disclosure** - All registration through Tor with burner credentials
- ✅ **Payment anonymity** - Monero-first, BTC with mixing as fallback
- ✅ **Jurisdictional diversity** - 6+ countries, no Five Eyes clustering
- ✅ **Provider diversity** - Mix traditional privacy VPS + decentralized compute
- ✅ **RAM-only operations** - No disk writes for sensitive data
- ✅ **Defense in depth** - Multiple independent security layers

---

## 📋 Pre-Deployment Checklist

### Required Tools & Resources

**Install these on your local machine:**

```bash
# 1. Tor Browser
# Download from: https://www.torproject.org/download/
# Used for: All provider registrations and server access

# 2. KeePassXC (password manager)
brew install keepassxc
# Used for: Storing all credentials OFFLINE (never cloud sync)

# 3. Monero Wallet (CLI or GUI)
# Download from: https://www.getmonero.org/downloads/
# Used for: Anonymous payments

# 4. Akash CLI
brew install akash
# Used for: Deploying to Akash Network

# 5. Keplr Wallet (browser extension)
# Install from: https://www.keplr.app/
# Used for: Akash payments ($AKT token)

# 6. WireGuard tools
brew install wireguard-tools

# 7. Terraform (for infrastructure as code)
brew install terraform

# 8. Ansible (for configuration management)
brew install ansible
```

**Acquire Monero (XMR) - CRITICAL FIRST STEP:**

You'll need approximately **0.5-1.0 XMR** (~$80-160 USD) for initial setup.

```bash
# Method 1: LocalMonero (P2P, anonymous)
1. Visit localmonero.co via Tor Browser
2. Find seller accepting cash deposit or gift cards
3. Complete trade
4. Receive XMR to your wallet

# Method 2: Non-KYC Exchange (less private)
# NOTE: Exchange landscape changes frequently
# Research current options on r/Monero or privacy forums
```

**Set Up Operational Security Environment:**

```bash
# 1. Create dedicated workspace (encrypted)
mkdir -p ~/invisible-infra-ops
cd ~/invisible-infra-ops

# 2. Initialize KeePassXC database
# File → New Database → invisible-infrastructure.kdbx
# Store in this directory, NEVER sync to cloud
# Password: Use 6+ word diceware passphrase

# 3. Create Tor-only networking profile
# All provider access MUST go through Tor Browser
# Save bookmarks in Tor Browser only
```

---

## 🔐 Phase 1: Anonymous Identity Creation

### 1.1 Burner Email Addresses

You'll need **3-5 burner emails** (one per provider to compartmentalize).

**For Each Email:**

```bash
# Open Tor Browser
# Visit: https://protonmail.com (or tutanota.com)

Username Pattern: [adjective]-[noun]-[4-digits]
Example: quiet-mountain-7821

CRITICAL:
- Create ONLY through Tor
- Use unique username/password per email
- Store in KeePassXC immediately
- NEVER access outside Tor
- Use fake but plausible recovery info
```

**Create these email identities:**

| Email Alias | Provider | Purpose |
|-------------|----------|---------|
| email-1 | ProtonMail | Njalla (entry nodes) |
| email-2 | Tutanota | Njalla (exit nodes) |
| email-3 | ProtonMail | FlokiNET |
| email-4 | Tutanota | Akash/Crypto accounts |
| email-5 | ProtonMail | Monitoring/alerts |

**Store in KeePassXC:**
```
Entry Name: Burner Email - email-1
Username: quiet-mountain-7821@protonmail.com
Password: [generated 20+ char]
URL: https://protonmail.com
Notes: Created 2026-02-14 via Tor. Used for: Njalla entry nodes.
```

### 1.2 Monero Wallet Setup

```bash
# Generate dedicated Monero wallet for infrastructure payments
monero-wallet-cli --generate-new-wallet invisible-infra

# Create strong password
# WRITE DOWN 25-word seed phrase on PAPER (not digital)
# Store paper in safe/secure location

# Get your primary address
address

# Save in KeePassXC:
Entry Name: Monero Wallet - Infrastructure
Username: [primary address]
Password: [wallet password]
Notes: Seed phrase stored offline. Used for: All infrastructure payments.
```

**Fund the wallet:**
- Transfer XMR from LocalMonero purchase
- Confirm balance: `balance`

### 1.3 Cryptocurrency for Akash ($AKT)

```bash
# Install Keplr wallet extension in regular browser
# Visit: https://app.osmosis.zone (DEX)

# Swap XMR → ATOM → AKT
# (Will need to bridge through an intermediate step)

# Alternative: Buy AKT directly
# Use Coinbase/Kraken → AKT → Send to Keplr
# Then mix/tumble to break chain analysis

# You'll need ~$50-100 worth of AKT for initial deployments
```

---

## 🌐 Phase 2: VPS Acquisition - Entry & Exit Nodes

### 2.1 Njalla VPS - Entry Nodes (3x instances)

**Open Tor Browser and visit: https://njal.la**

#### Instance 1: Iceland Entry Node

```yaml
Step 1: Navigate to VPS section
Step 2: Select VPS configuration
  - OS: Debian 12 (minimal)
  - RAM: 2GB
  - Storage: 20GB SSD
  - Location: Iceland
  - Price: ~€15/month

Step 3: Checkout
  - Email: email-1@protonmail.com
  - Payment Method: Monero (XMR)
  - Server Name: relay-entry-is-01

Step 4: Complete payment
  - Copy XMR payment address
  - Send exact amount from your Monero wallet:
    monero-wallet-cli
    > transfer [njalla-address] [amount]
  - Wait for confirmation (10-20 minutes)

Step 5: Save credentials in KeePassXC
  Entry Name: Njalla VPS - Iceland Entry 01
  Username: root
  Password: [initial password from email]
  URL: [IP address from Njalla dashboard]
  Notes: |
    Provider: Njalla
    Location: Iceland
    Purpose: Entry node (Layer 1)
    Deployed: 2026-02-14
    Payment: XMR (transaction ID: [txid])
```

#### Instance 2: Nevis Entry Node

Repeat above process:
- Location: **Nevis** (Njalla's home jurisdiction)
- Server Name: `relay-entry-nv-01`
- Use same email or email-2 for compartmentalization

#### Instance 3: Romania Entry Node (via FlokiNET)

**Open Tor Browser and visit: https://flokinet.is**

```yaml
Configuration:
  - OS: Debian 12
  - RAM: 2GB
  - Storage: 20GB
  - Location: Romania
  - Payment: Monero
  - Email: email-3@tutanota.com
  - Server Name: relay-entry-ro-01
```

### 2.2 Njalla VPS - Exit Nodes (3x instances)

Repeat Njalla process for exit nodes:

```yaml
Exit Node 1:
  Location: Iceland
  Name: relay-exit-is-01
  Email: email-2@tutanota.com

Exit Node 2:
  Location: Sweden (via Njalla)
  Name: relay-exit-se-01
  Email: email-2@tutanota.com

Exit Node 3 (via FlokiNET):
  Location: Finland
  Name: relay-exit-fi-01
  Email: email-3@tutanota.com
```

**Current Status:**
- ✅ 3 Entry Nodes (Iceland, Nevis, Romania)
- ✅ 3 Exit Nodes (Iceland, Sweden, Finland)
- **Cost:** ~€90/month (~$95 USD)

---

## ☁️ Phase 3: Decentralized Core - Akash Network

### 3.1 Akash Account Setup

```bash
# Install Akash CLI (if not already done)
brew install akash

# Create Akash wallet
akash keys add invisible-infra
# SAVE the mnemonic phrase in KeePassXC (24 words)

# Get your address
akash keys show invisible-infra -a

# Fund wallet with AKT from Keplr
# Send ~50-100 AKT to this address
```

### 3.2 Create Deployment Manifest

Create `akash-relay-node.yaml`:

```yaml
---
version: "2.0"

services:
  invisible-relay:
    image: invisible/relay-node:latest  # Your Docker image
    env:
      - "NODE_TYPE=mix"
      - "LAYER=2"
      - "LOG_LEVEL=warn"
    expose:
      - port: 8080
        as: 80
        to:
          - global: true
      - port: 9100  # WireGuard
        as: 9100
        proto: udp
        to:
          - global: true

profiles:
  compute:
    invisible-relay:
      resources:
        cpu:
          units: 1.0
        memory:
          size: 2Gi
        storage:
          size: 10Gi
  placement:
    dcloud:
      pricing:
        invisible-relay:
          denom: uakt
          amount: 100  # 0.0001 AKT per block (~$0.50/month)

deployment:
  invisible-relay:
    dcloud:
      profile: invisible-relay
      count: 1
```

### 3.3 Deploy to Akash (15x instances)

Create deployment script `deploy-akash-nodes.sh`:

```bash
#!/bin/bash
# Akash Multi-Node Deployment Script

AKASH_CHAIN_ID="akashnet-2"
AKASH_NODE="https://rpc.akashnet.net:443"
ACCOUNT="invisible-infra"

# Define node configurations
declare -A NODES=(
    ["mix-layer2-01"]="2"
    ["mix-layer2-02"]="2"
    ["mix-layer2-03"]="2"
    ["mix-layer3-01"]="3"
    ["mix-layer3-02"]="3"
    ["mix-layer3-03"]="3"
    ["mix-layer3-04"]="3"
    ["mix-layer4-01"]="4"
    ["mix-layer4-02"]="4"
    ["mix-layer4-03"]="4"
    ["mix-layer4-04"]="4"
    ["mix-layer4-05"]="4"
    ["mix-layer5-01"]="5"
    ["mix-layer5-02"]="5"
    ["mix-layer5-03"]="5"
)

for NODE_NAME in "${!NODES[@]}"; do
    LAYER=${NODES[$NODE_NAME]}

    echo "Deploying $NODE_NAME (Layer $LAYER)..."

    # Update manifest with node-specific config
    sed "s/LAYER=2/LAYER=$LAYER/" akash-relay-node.yaml > /tmp/deploy-${NODE_NAME}.yaml

    # Create deployment
    akash tx deployment create /tmp/deploy-${NODE_NAME}.yaml \
        --from $ACCOUNT \
        --chain-id $AKASH_CHAIN_ID \
        --node $AKASH_NODE \
        --fees 5000uakt \
        --gas auto \
        --gas-adjustment 1.3 \
        -y

    # Wait for deployment to be created
    sleep 10

    # Get deployment ID (latest)
    DSEQ=$(akash query deployment list --owner $(akash keys show $ACCOUNT -a) \
        --chain-id $AKASH_CHAIN_ID --node $AKASH_NODE \
        --output json | jq -r '.deployments[0].deployment.deployment_id.dseq')

    echo "Deployment created: DSEQ=$DSEQ"

    # Wait for bids
    sleep 30

    # Accept lowest bid from reputable provider
    # (Add provider filtering logic here based on reputation)
    akash tx market lease create \
        --dseq $DSEQ \
        --from $ACCOUNT \
        --chain-id $AKASH_CHAIN_ID \
        --node $AKASH_NODE \
        --fees 5000uakt \
        -y

    echo "$NODE_NAME deployed successfully!"
    echo "DSEQ: $DSEQ" >> deployed-nodes.log

    # Rate limiting to avoid spam
    sleep 20
done

echo "All 15 Akash nodes deployed!"
echo "Check deployed-nodes.log for deployment IDs"
```

**Run deployment:**

```bash
chmod +x deploy-akash-nodes.sh
./deploy-akash-nodes.sh
```

**Store deployment info in KeePassXC:**

```
Entry Name: Akash Deployments - Mix Core
Notes: |
  Deployment IDs (DSEQ):
  mix-layer2-01: 1234567
  mix-layer2-02: 1234568
  ...

  Access via: akash provider lease-logs --dseq [DSEQ] ...
  Management: Akash Console (cloudmos.io/deploy)
```

**Estimated Cost:** 15 nodes × $3-5/month = **$45-75/month**

---

## 💾 Phase 4: Dead Drop Storage - Storj DCS

### 4.1 Storj Account Setup

**Via Tor Browser, visit: https://www.storj.io/signup**

```yaml
Email: email-4@tutanota.com
Password: [generated strong password]
Payment: Credit card purchased with cash OR crypto top-up

Store in KeePassXC:
  Entry Name: Storj DCS Account
  Username: email-4@tutanota.com
  Password: [password]
  Notes: Used for dead drop encrypted storage
```

### 4.2 Create Access Grant

```bash
# Install Storj Uplink CLI
curl -L https://github.com/storj/storj/releases/latest/download/uplink_linux_amd64.zip -o uplink.zip
unzip uplink.zip
sudo install uplink /usr/local/bin/uplink

# Setup access grant
uplink access create invisible-deaddrops \
    --satellite us1.storj.io \
    --passphrase "your-generated-passphrase"

# Generate restricted access (read/write to specific buckets)
uplink access restrict invisible-deaddrops \
    --readonly=false \
    --writeonly=false \
    > deaddrops-access.grant

# Store access grant in KeePassXC
```

### 4.3 Create Buckets for Dead Drops

```bash
# Create bucket for relay mailboxes
uplink mb sj://invisible-deaddrops-layer5

# Verify
uplink ls
```

**Estimated Cost:** $4-7/month (150GB storage, 150GB egress)

---

## 🔧 Phase 5: Server Hardening & Configuration

Now we configure all VPS instances (Njalla + FlokiNET nodes).

### 5.1 Initial SSH Setup (Per Node)

**CRITICAL: Only access via Tor or VPN, NEVER from your home IP.**

For each Njalla/FlokiNET VPS:

```bash
# Generate unique SSH key per node (on local machine)
ssh-keygen -t ed25519 -f ~/.ssh/invisible-relay-entry-is-01 -C "entry-is-01"

# Repeat for all 6 nodes, unique key each

# Store private keys in KeePassXC as attachments
# Entry → Advanced → Attachments → Add File
```

**First login (example for Iceland entry node):**

```bash
# Via Tor:
# Open Tor Browser, get SOCKS proxy (127.0.0.1:9150)

# SSH through Tor proxy
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    root@[IP-ADDRESS] \
    -i ~/.ssh/invisible-relay-entry-is-01

# Change root password immediately
passwd

# Store new password in KeePassXC
```

### 5.2 Hardening Script (Run on Each Node)

Create `harden-relay-node.sh`:

```bash
#!/bin/bash
# Invisible Relay Node Hardening Script
# Run as: root

set -euo pipefail

echo "Starting hardening for Invisible relay node..."

# 1. Update system
apt update && apt upgrade -y

# 2. Install essential packages
apt install -y \
    ufw \
    fail2ban \
    unattended-upgrades \
    wireguard \
    htop \
    tmux \
    curl \
    git

# 3. Configure automatic security updates
dpkg-reconfigure -plow unattended-upgrades
cat > /etc/apt/apt.conf.d/50unattended-upgrades <<EOF
Unattended-Upgrade::Allowed-Origins {
    "\${distro_id}:\${distro_codename}-security";
};
Unattended-Upgrade::AutoFixInterruptedDpkg "true";
Unattended-Upgrade::Remove-Unused-Dependencies "true";
Unattended-Upgrade::Automatic-Reboot "true";
Unattended-Upgrade::Automatic-Reboot-Time "03:00";
EOF

# 4. Disable password authentication, enable SSH key only
sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config
sed -i 's/#PermitRootLogin yes/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config

# 5. Upload your SSH public key (do this manually first)
# mkdir -p ~/.ssh
# echo "[YOUR-PUBLIC-KEY]" >> ~/.ssh/authorized_keys
# chmod 600 ~/.ssh/authorized_keys

# Restart SSH
systemctl restart sshd

# 6. Configure firewall
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp      # SSH
ufw allow 8080/tcp    # Relay node HTTP
ufw allow 9100/udp    # WireGuard
ufw --force enable

# 7. Configure fail2ban
cat > /etc/fail2ban/jail.local <<EOF
[sshd]
enabled = true
port = 22
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
EOF

systemctl enable fail2ban
systemctl restart fail2ban

# 8. Disable swap (force RAM-only for sensitive data)
swapoff -a
sed -i '/ swap / s/^/#/' /etc/fstab

# 9. Configure tmpfs for RAM-only temp directories
cat >> /etc/fstab <<EOF
tmpfs /tmp tmpfs defaults,noatime,mode=1777 0 0
tmpfs /var/tmp tmpfs defaults,noatime,mode=1777 0 0
EOF

mount -a

# 10. Set timezone to UTC
timedatectl set-timezone UTC

# 11. Disable unnecessary services
systemctl disable bluetooth.service || true
systemctl disable avahi-daemon.service || true

# 12. Install Docker (for running Invisible relay software)
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
rm get-docker.sh

# 13. Configure Docker to use tmpfs for sensitive data
mkdir -p /etc/docker
cat > /etc/docker/daemon.json <<EOF
{
  "log-driver": "none",
  "storage-driver": "overlay2",
  "data-root": "/var/lib/docker"
}
EOF

systemctl restart docker

# 14. Create non-root user for running relay
useradd -m -s /bin/bash invisible
usermod -aG docker invisible

# 15. Install WireGuard and generate keys
wg genkey | tee /etc/wireguard/private.key | wg pubkey > /etc/wireguard/public.key
chmod 600 /etc/wireguard/private.key

echo "Hardening complete!"
echo "WireGuard public key:"
cat /etc/wireguard/public.key
echo ""
echo "NEXT STEPS:"
echo "1. Copy your SSH public key to ~/.ssh/authorized_keys"
echo "2. Log out and back in with SSH key"
echo "3. Verify password auth is disabled"
echo "4. Deploy Invisible relay software"
```

**Deploy to each node:**

```bash
# Copy script to server
scp -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    harden-relay-node.sh root@[IP]:/root/

# SSH in and run
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" root@[IP]
chmod +x harden-relay-node.sh
./harden-relay-node.sh

# Save WireGuard public key to KeePassXC
```

**Repeat for all 6 VPS nodes.**

---

## 🔌 Phase 6: WireGuard VPN Mesh Configuration

### 6.1 Design WireGuard Topology

```
Entry Nodes (3x) → WireGuard VPN → Akash Mix Core (15x) → Exit Nodes (3x)
```

Each node gets a unique WireGuard IP in 10.20.30.0/24 range:

```yaml
Entry Nodes:
  relay-entry-is-01: 10.20.30.1
  relay-entry-nv-01: 10.20.30.2
  relay-entry-ro-01: 10.20.30.3

Mix Core (Akash):
  mix-layer2-01: 10.20.30.10
  mix-layer2-02: 10.20.30.11
  ... (continue sequentially)

Exit Nodes:
  relay-exit-is-01: 10.20.30.100
  relay-exit-se-01: 10.20.30.101
  relay-exit-fi-01: 10.20.30.102
```

### 6.2 WireGuard Configuration Template

For **entry node** (relay-entry-is-01):

```ini
# /etc/wireguard/wg0.conf

[Interface]
PrivateKey = [PRIVATE-KEY-FROM-/etc/wireguard/private.key]
Address = 10.20.30.1/24
ListenPort = 9100
SaveConfig = false

# Peer: mix-layer2-01 (Akash)
[Peer]
PublicKey = [PUBLIC-KEY-OF-mix-layer2-01]
AllowedIPs = 10.20.30.10/32
Endpoint = [AKASH-NODE-PUBLIC-IP]:9100
PersistentKeepalive = 25

# Peer: mix-layer2-02 (Akash)
[Peer]
PublicKey = [PUBLIC-KEY-OF-mix-layer2-02]
AllowedIPs = 10.20.30.11/32
Endpoint = [AKASH-NODE-PUBLIC-IP]:9100
PersistentKeepalive = 25

# (Add all peers this node connects to)
```

**Enable WireGuard:**

```bash
# On each node
chmod 600 /etc/wireguard/wg0.conf
systemctl enable wg-quick@wg0
systemctl start wg-quick@wg0

# Verify
wg show
ping 10.20.30.10  # Test connectivity to mix node
```

### 6.3 Automate WireGuard Configuration

Create `generate-wireguard-configs.py`:

```python
#!/usr/bin/env python3
"""
Generate WireGuard configurations for Invisible relay network
"""

import json

# Load node inventory (from KeePassXC export or manual list)
nodes = {
    "entry": [
        {"name": "relay-entry-is-01", "ip": "10.20.30.1", "public_key": "..."},
        {"name": "relay-entry-nv-01", "ip": "10.20.30.2", "public_key": "..."},
        {"name": "relay-entry-ro-01", "ip": "10.20.30.3", "public_key": "..."},
    ],
    "mix": [
        {"name": "mix-layer2-01", "ip": "10.20.30.10", "public_key": "...", "endpoint": "IP:9100"},
        # ... add all 15 mix nodes
    ],
    "exit": [
        {"name": "relay-exit-is-01", "ip": "10.20.30.100", "public_key": "..."},
        {"name": "relay-exit-se-01", "ip": "10.20.30.101", "public_key": "..."},
        {"name": "relay-exit-fi-01", "ip": "10.20.30.102", "public_key": "..."},
    ]
}

# Define mesh topology (who connects to whom)
# Entry → Mix Layer 2/3, Mix → Mix + Exit, Exit → Internet

def generate_config(node, peers):
    config = f"""[Interface]
PrivateKey = {{PRIVATE_KEY}}
Address = {node['ip']}/24
ListenPort = 9100

"""
    for peer in peers:
        config += f"""[Peer]
PublicKey = {peer['public_key']}
AllowedIPs = {peer['ip']}/32
"""
        if 'endpoint' in peer:
            config += f"Endpoint = {peer['endpoint']}\n"
        config += "PersistentKeepalive = 25\n\n"

    return config

# Generate configs for all nodes
for category, node_list in nodes.items():
    for node in node_list:
        # Define peers based on topology
        if category == "entry":
            peers = [n for n in nodes["mix"] if n["name"].startswith("mix-layer2")]
        elif category == "mix":
            # Mix nodes connect to adjacent layers
            # (Simplified: connect to all, filter in routing logic)
            peers = nodes["entry"] + nodes["mix"] + nodes["exit"]
            peers = [p for p in peers if p != node]  # Don't peer with self
        elif category == "exit":
            peers = [n for n in nodes["mix"] if "layer4" in n["name"] or "layer5" in n["name"]]

        config = generate_config(node, peers)

        with open(f"configs/wg0-{node['name']}.conf", "w") as f:
            f.write(config)

        print(f"Generated: wg0-{node['name']}.conf")

print("\nConfigs generated in configs/ directory")
print("Replace {PRIVATE_KEY} with actual private keys before deploying")
```

**Run generator:**

```bash
mkdir configs
python3 generate-wireguard-configs.py

# Review configs
ls configs/

# Deploy to nodes (via SCP through Tor)
for node in relay-entry-is-01 relay-entry-nv-01 ...; do
    scp -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
        configs/wg0-${node}.conf root@[IP]:/etc/wireguard/wg0.conf
done
```

---

## 🐳 Phase 7: Deploy Invisible Relay Software

### 7.1 Build Docker Image

In your Invisible project directory:

```bash
cd ~/Documents/invisible

# Create Dockerfile for relay node
cat > crates/relay/Dockerfile <<EOF
FROM rust:1.70-slim as builder

WORKDIR /build
COPY . .

# Build relay node binary
RUN cargo build --release --bin invisible-relay

# Production image
FROM debian:12-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    wireguard-tools \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/invisible-relay /usr/local/bin/

# Run as non-root
RUN useradd -m -s /bin/bash relay
USER relay

EXPOSE 8080
EXPOSE 9100/udp

CMD ["/usr/local/bin/invisible-relay"]
EOF

# Build image
docker build -t invisible/relay-node:v0.1.0 -f crates/relay/Dockerfile .

# Tag for registry (you'll need a private registry or Docker Hub)
docker tag invisible/relay-node:v0.1.0 yourregistry.com/invisible/relay-node:v0.1.0

# Push to registry
docker push yourregistry.com/invisible/relay-node:v0.1.0
```

### 7.2 Deploy to VPS Nodes

Create `docker-compose.yml` for each node type:

**Entry Node:**

```yaml
version: '3.8'

services:
  relay-entry:
    image: yourregistry.com/invisible/relay-node:v0.1.0
    container_name: invisible-relay-entry
    restart: unless-stopped
    network_mode: host
    environment:
      - NODE_TYPE=entry
      - NODE_LAYER=1
      - WIREGUARD_INTERFACE=wg0
      - LOG_LEVEL=info
      - SPHINX_ENABLED=true
    volumes:
      - /tmp/relay-state:/tmp/state  # tmpfs, RAM-only
      - /etc/wireguard/wg0.conf:/etc/wireguard/wg0.conf:ro
    cap_add:
      - NET_ADMIN
    tmpfs:
      - /tmp/relay-state:rw,noexec,nosuid,size=100m
```

**Mix Node (Akash):**

Update `akash-relay-node.yaml` to use your Docker image:

```yaml
services:
  invisible-relay:
    image: yourregistry.com/invisible/relay-node:v0.1.0
    # ... rest of config
```

Redeploy Akash nodes with updated image.

**Exit Node:**

Similar to entry node, but:

```yaml
environment:
  - NODE_TYPE=exit
  - NODE_LAYER=5
```

### 7.3 Deploy via Ansible (Automated)

Create `ansible/playbook-deploy-relay.yml`:

```yaml
---
- name: Deploy Invisible Relay Nodes
  hosts: relay_nodes
  become: yes
  tasks:
    - name: Copy docker-compose file
      copy:
        src: "docker-compose-{{ node_type }}.yml"
        dest: /opt/invisible/docker-compose.yml

    - name: Pull latest relay image
      docker_image:
        name: yourregistry.com/invisible/relay-node:v0.1.0
        source: pull

    - name: Start relay container
      docker_compose:
        project_src: /opt/invisible
        state: present

    - name: Verify relay is running
      wait_for:
        port: 8080
        delay: 5
        timeout: 60
```

**Inventory file** `ansible/inventory.ini`:

```ini
[entry_nodes]
relay-entry-is-01 ansible_host=X.X.X.X node_type=entry
relay-entry-nv-01 ansible_host=X.X.X.X node_type=entry
relay-entry-ro-01 ansible_host=X.X.X.X node_type=entry

[exit_nodes]
relay-exit-is-01 ansible_host=X.X.X.X node_type=exit
relay-exit-se-01 ansible_host=X.X.X.X node_type=exit
relay-exit-fi-01 ansible_host=X.X.X.X node_type=exit

[relay_nodes:children]
entry_nodes
exit_nodes

[all:vars]
ansible_user=root
ansible_ssh_private_key_file=~/.ssh/invisible-relay-key
ansible_ssh_common_args='-o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p"'
```

**Run deployment:**

```bash
cd ansible
ansible-playbook -i inventory.ini playbook-deploy-relay.yml
```

---

## 📊 Phase 8: Monitoring & Observability

### 8.1 Deploy Prometheus + Grafana (Akash)

Create `akash-monitoring.yaml`:

```yaml
---
version: "2.0"

services:
  prometheus:
    image: prom/prometheus:latest
    env:
      - "RETENTION_TIME=15d"
    expose:
      - port: 9090
        to:
          - service: grafana

  grafana:
    image: grafana/grafana:latest
    env:
      - "GF_AUTH_ANONYMOUS_ENABLED=true"
      - "GF_SECURITY_ADMIN_PASSWORD=your-strong-password"
    expose:
      - port: 3000
        as: 80
        to:
          - global: true  # Expose via Tor hidden service later

profiles:
  compute:
    prometheus:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 1Gi
        storage:
          size: 20Gi
    grafana:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 1Gi
        storage:
          size: 5Gi

  placement:
    dcloud:
      pricing:
        prometheus:
          denom: uakt
          amount: 50
        grafana:
          denom: uakt
          amount: 50

deployment:
  prometheus:
    dcloud:
      profile: prometheus
      count: 1
  grafana:
    dcloud:
      profile: grafana
      count: 1
```

Deploy:

```bash
akash tx deployment create akash-monitoring.yaml --from invisible-infra ...
```

### 8.2 Configure Relay Nodes to Export Metrics

Update relay software to expose Prometheus metrics on `:9100/metrics`:

```rust
// In your Rust relay code (crates/relay/src/metrics.rs)
use prometheus::{Encoder, TextEncoder, Registry, Counter, Histogram};

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();

    static ref PACKETS_RECEIVED: Counter = Counter::new(
        "relay_packets_received_total",
        "Total Sphinx packets received"
    ).unwrap();

    static ref PACKETS_FORWARDED: Counter = Counter::new(
        "relay_packets_forwarded_total",
        "Total packets forwarded"
    ).unwrap();

    static ref PACKET_LATENCY: Histogram = Histogram::new(
        "relay_packet_latency_seconds",
        "Packet processing latency"
    ).unwrap();
}

pub fn init_metrics() {
    REGISTRY.register(Box::new(PACKETS_RECEIVED.clone())).unwrap();
    REGISTRY.register(Box::new(PACKETS_FORWARDED.clone())).unwrap();
    REGISTRY.register(Box::new(PACKET_LATENCY.clone())).unwrap();
}

pub fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

### 8.3 Prometheus Scrape Configuration

Configure Prometheus to scrape all relay nodes:

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'relay-entry-nodes'
    static_configs:
      - targets:
          - '10.20.30.1:9100'  # relay-entry-is-01
          - '10.20.30.2:9100'  # relay-entry-nv-01
          - '10.20.30.3:9100'  # relay-entry-ro-01

  - job_name: 'relay-mix-nodes'
    static_configs:
      - targets:
          - '10.20.30.10:9100'  # mix-layer2-01
          - '10.20.30.11:9100'  # mix-layer2-02
          # ... all 15 mix nodes

  - job_name: 'relay-exit-nodes'
    static_configs:
      - targets:
          - '10.20.30.100:9100'
          - '10.20.30.101:9100'
          - '10.20.30.102:9100'
```

### 8.4 Grafana Dashboards

Create dashboard JSON for import into Grafana:

```json
{
  "dashboard": {
    "title": "Invisible Relay Network",
    "panels": [
      {
        "title": "Packets Received (All Nodes)",
        "targets": [
          {
            "expr": "sum(rate(relay_packets_received_total[5m]))"
          }
        ]
      },
      {
        "title": "Packet Latency (p99)",
        "targets": [
          {
            "expr": "histogram_quantile(0.99, rate(relay_packet_latency_seconds_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Active Nodes",
        "targets": [
          {
            "expr": "count(up{job=~'relay-.*'} == 1)"
          }
        ]
      }
    ]
  }
}
```

### 8.5 Alerting Configuration

Create `prometheus-alerts.yml`:

```yaml
groups:
  - name: invisible_relay_alerts
    interval: 30s
    rules:
      - alert: RelayNodeDown
        expr: up{job=~"relay-.*"} == 0
        for: 2m
        annotations:
          summary: "Relay node {{ $labels.instance }} is down"

      - alert: HighPacketLatency
        expr: histogram_quantile(0.99, rate(relay_packet_latency_seconds_bucket[5m])) > 2
        for: 5m
        annotations:
          summary: "High packet latency detected (p99 > 2s)"

      - alert: PacketDropRate
        expr: |
          (rate(relay_packets_received_total[5m]) - rate(relay_packets_forwarded_total[5m]))
          / rate(relay_packets_received_total[5m]) > 0.05
        for: 5m
        annotations:
          summary: "Packet drop rate > 5%"
```

Configure Alertmanager to send alerts to your Signal (Invisible) account (eventually).

---

## 🧪 Phase 9: Testing & Validation

### 9.1 Network Connectivity Test

```bash
# SSH to entry node
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" root@relay-entry-is-01

# Test WireGuard connectivity
ping 10.20.30.10  # Mix layer 2
ping 10.20.30.100  # Exit node

# Test relay software
curl http://10.20.30.10:8080/health
```

### 9.2 Sphinx Packet Routing Test

Create test script `test-relay-path.sh`:

```bash
#!/bin/bash
# Test end-to-end packet routing through mix network

# Send test packet through entry node
curl -X POST http://10.20.30.1:8080/send \
  -H "Content-Type: application/octet-stream" \
  --data-binary @test-sphinx-packet.bin

# Monitor logs on all nodes
# Should see packet traverse: Entry → Mix2 → Mix3 → Mix4 → Mix5 → Exit

# Verify packet arrives at destination
# (Add actual test logic based on your Sphinx implementation)
```

### 9.3 Performance Benchmarking

```bash
# Measure round-trip latency through full mix network
for i in {1..100}; do
    time curl -X POST http://10.20.30.1:8080/test-route \
        -d '{"hops": 5, "destination": "test"}' \
        -s -o /dev/null
done | grep real | awk '{sum+=$2; count++} END {print "Average:", sum/count, "s"}'
```

### 9.4 Load Testing

Use `hey` or `vegeta` for load testing:

```bash
# Install hey
go install github.com/rakyll/hey@latest

# Test entry node throughput
hey -n 10000 -c 50 -m POST \
    -D test-packet.bin \
    http://10.20.30.1:8080/send

# Expected: >1000 req/s with <200ms latency for production readiness
```

---

## 🔒 Phase 10: Security Audit & Final Hardening

### 10.1 Security Checklist

**Verify on ALL nodes:**

- [ ] SSH password auth disabled
- [ ] Root login disabled (or key-only)
- [ ] Firewall (ufw) active and configured
- [ ] Fail2ban running
- [ ] Automatic security updates enabled
- [ ] No swap enabled (RAM-only for sensitive data)
- [ ] Docker logging disabled
- [ ] WireGuard properly configured
- [ ] Strong, unique passwords in KeePassXC
- [ ] No credentials in git history
- [ ] All access via Tor or VPN only

### 10.2 Penetration Testing

Run basic security audit:

```bash
# Install security audit tools
apt install lynis chkrootkit rkhunter

# Run Lynis audit on each node
lynis audit system

# Review findings and remediate
```

### 10.3 External Security Audit (Recommended)

Before production launch with real users, consider hiring:
- **Trail of Bits** (top-tier security audits)
- **Cure53** (privacy/crypto specialists)
- **NCC Group** (cryptography experts)

Focus areas:
- Cryptographic implementation (X3DH, Double Ratchet, Sphinx)
- Network privacy (mix network, cover traffic)
- Memory safety (Rust unsafe code review)
- Side-channel attacks

**Estimated Cost:** $30,000 - $80,000 for comprehensive audit

---

## 📝 Phase 11: Operational Procedures

### 11.1 Backup & Disaster Recovery

**What to back up:**
- ✅ WireGuard private keys (encrypted)
- ✅ Node inventory (KeePassXC database)
- ✅ Deployment manifests (git repository)
- ✅ Monitoring configurations

**What NOT to back up (ephemeral):**
- ❌ Relay state (RAM-only by design)
- ❌ Message data (zero-log doctrine)
- ❌ User metadata (doesn't exist)

**Backup script:**

```bash
#!/bin/bash
# backup-infrastructure.sh

BACKUP_DIR=~/invisible-infra-backups/$(date +%Y-%m-%d)
mkdir -p "$BACKUP_DIR"

# Backup KeePassXC database (encrypted already)
cp ~/invisible-infra-ops/invisible-infrastructure.kdbx "$BACKUP_DIR/"

# Backup deployment configs
cp -r ~/invisible-infra-ops/ansible "$BACKUP_DIR/"
cp -r ~/invisible-infra-ops/configs "$BACKUP_DIR/"

# Encrypt backup
tar czf - "$BACKUP_DIR" | gpg --symmetric --cipher-algo AES256 > \
    "$BACKUP_DIR.tar.gz.gpg"

# Store on USB drive or encrypted cloud storage (NOT regular cloud)
# Verify encryption before uploading
rm -rf "$BACKUP_DIR"

echo "Backup created: $BACKUP_DIR.tar.gz.gpg"
```

### 11.2 Node Rotation Policy

**Rotate nodes every 6 months to minimize attack surface:**

```bash
# Decommission procedure for a node
1. Remove from WireGuard mesh (update configs)
2. Stop relay software
3. Destroy VPS instance
4. Remove from monitoring
5. Spin up replacement in different jurisdiction
6. Update DNS/IP mappings

# Rotate on schedule
- Entry nodes: Every 6 months, staggered
- Mix nodes (Akash): Every 3 months (easier with decentralized)
- Exit nodes: Every 6 months
```

### 11.3 Incident Response Plan

**If a node is compromised:**

1. **Isolate:** Remove from WireGuard mesh immediately
2. **Analyze:** Pull logs, check for unusual activity
3. **Destroy:** Wipe and delete VPS instance
4. **Rotate:** Deploy replacement in new jurisdiction
5. **Review:** Audit all other nodes for similar compromise
6. **Notify:** Alert users if metadata exposure risk (unlikely with RAM-only)

### 11.4 Monitoring Playbook

**Daily checks:**
- Grafana dashboard review (5 min)
- Alert inbox check (email-5)
- Node uptime verification

**Weekly checks:**
- Security update review
- Performance metrics analysis
- Cost review (VPS + Akash + Storj bills)

**Monthly checks:**
- Full security audit (lynis on all nodes)
- Dependency updates (Rust crates, Docker images)
- Backup verification (restore test)

---

## 💵 Phase 12: Cost Management & Billing

### 12.1 Cost Breakdown (Monthly)

```yaml
VPS Providers:
  Njalla (3 entry nodes):     €45  (~$48)
  Njalla (2 exit nodes):      €30  (~$32)
  FlokiNET (1 entry):         €15  (~$16)
  FlokiNET (1 exit):          €15  (~$16)
  Subtotal VPS:               €105 (~$112)

Decentralized Infrastructure:
  Akash (15 mix nodes):       $60
  Storj (dead drops):         $7
  Subtotal Decentralized:     $67

Domains & Extras:
  Njalla domains (optional):  €15/year (~$1/month)

Total Monthly Cost:           ~$180 USD

Annual Cost:                  ~$2,160 USD
```

**Scaling costs:**
- Each additional entry/exit node: +€15/month
- Each additional Akash mix node: +$4/month
- Monitoring (already included in Akash deployment): $0

### 12.2 Payment Tracking

Create spreadsheet or KeePassXC entries:

```
Entry: Njalla Subscription - Entry Nodes
Amount: €45/month
Payment Method: Monero
Last Payment: 2026-02-14
Next Payment: 2026-03-14
Transaction IDs: [XMR txid list]
```

Set calendar reminders for payment due dates (providers may terminate for non-payment).

### 12.3 Cryptocurrency Management

**Maintain XMR balance:**
- Keep 1-2 months of runway (~0.3-0.5 XMR)
- Top up monthly via LocalMonero
- Never reuse addresses

**Akash ($AKT):**
- Monitor balance in Keplr wallet
- Refill when balance drops below 20 AKT
- Use DEX swaps for privacy (XMR → ATOM → AKT)

---

## ✅ Phase 13: Go-Live Checklist

Before considering infrastructure "production ready":

### 13.1 Technical Validation

- [ ] All 21+ nodes deployed and running
- [ ] WireGuard mesh fully connected (verify with `wg show` on each node)
- [ ] Relay software operational (health checks passing)
- [ ] Sphinx packet routing tested end-to-end
- [ ] Monitoring operational (Prometheus + Grafana)
- [ ] Alerting configured and tested
- [ ] Performance benchmarks met (>1000 msg/s, <2s latency)
- [ ] Load testing passed (sustained 10k messages)

### 13.2 Security Validation

- [ ] All SSH access key-only (no passwords)
- [ ] Firewalls active on all nodes
- [ ] No credentials in git/code
- [ ] KeePassXC database backed up offline
- [ ] All provider access only via Tor
- [ ] Payment trails anonymized (XMR used)
- [ ] No personal info in VPS registrations
- [ ] Lynis audits passed on all VPS nodes
- [ ] External security audit scheduled (recommended)

### 13.3 Operational Validation

- [ ] Backup procedures tested
- [ ] Node rotation playbook documented
- [ ] Incident response plan written
- [ ] Monitoring playbook established
- [ ] Cost tracking system in place
- [ ] Payment reminders set
- [ ] Documentation complete and accessible

### 13.4 Privacy Validation

- [ ] No logs written to disk on any node
- [ ] tmpfs configured for RAM-only temp data
- [ ] Packet routing verified to be multi-hop
- [ ] No single provider sees full network topology
- [ ] Jurisdictional diversity confirmed (6+ countries)
- [ ] No Five Eyes clustering in routing paths
- [ ] Dead drop storage encrypted and sharded (Storj)

---

## 🚀 Phase 14: Deployment Execution Timeline

**Suggested timeline for full deployment:**

### Week 1: Foundation
- ✅ Day 1-2: Acquire Monero, set up burner emails
- ✅ Day 3-4: Purchase Njalla VPS instances (6x)
- ✅ Day 5-7: Harden all VPS nodes, configure WireGuard

### Week 2: Decentralized Core
- ✅ Day 8-9: Set up Akash account, acquire AKT
- ✅ Day 10-12: Deploy 15 Akash mix nodes
- ✅ Day 13-14: Configure Storj dead drop storage

### Week 3: Integration
- ✅ Day 15-17: Complete WireGuard mesh networking
- ✅ Day 18-19: Deploy relay software to all nodes
- ✅ Day 20-21: Set up monitoring (Prometheus/Grafana)

### Week 4: Testing & Validation
- ✅ Day 22-24: End-to-end testing, performance benchmarking
- ✅ Day 25-26: Security audits, hardening
- ✅ Day 27-28: Documentation, operational procedures

**Total: 4 weeks from zero to production-ready infrastructure**

---

## 📚 Appendix A: Quick Reference Commands

### SSH Through Tor

```bash
# Add to ~/.ssh/config for convenience
Host relay-*
    ProxyCommand nc -X 5 -x 127.0.0.1:9150 %h %p
    IdentityFile ~/.ssh/invisible-relay-%h
    User root

# Usage
ssh relay-entry-is-01
```

### Monero Payments

```bash
# Check balance
monero-wallet-cli --wallet-file invisible-infra
> balance

# Send payment
> transfer [address] [amount]

# View transaction history
> show_transfers
```

### Akash Management

```bash
# List active deployments
akash query deployment list --owner $(akash keys show invisible-infra -a)

# View deployment logs
akash provider lease-logs --dseq [DSEQ] --from invisible-infra

# Update deployment
akash tx deployment update [manifest.yaml] --dseq [DSEQ] --from invisible-infra

# Close deployment
akash tx deployment close --dseq [DSEQ] --from invisible-infra
```

### WireGuard

```bash
# Show status
wg show

# Restart interface
wg-quick down wg0 && wg-quick up wg0

# Check connectivity
ping 10.20.30.X  # Test peer
```

### Docker

```bash
# View relay logs
docker logs -f invisible-relay-entry

# Restart relay
docker-compose restart

# Update relay image
docker-compose pull && docker-compose up -d
```

---

## 📚 Appendix B: Troubleshooting

### Problem: Can't connect to VPS

**Solution:**
```bash
# Verify Tor is running
curl --socks5-hostname 127.0.0.1:9150 https://check.torproject.org

# Check SSH key permissions
chmod 600 ~/.ssh/invisible-relay-*

# Verify IP address is correct
# Check Njalla dashboard for current IP
```

### Problem: WireGuard peers won't connect

**Solution:**
```bash
# Check firewall allows UDP 9100
ufw status

# Verify WireGuard is running
systemctl status wg-quick@wg0

# Check public key matches
cat /etc/wireguard/public.key

# Test with wg show
wg show wg0
```

### Problem: Akash deployment fails

**Solution:**
```bash
# Check AKT balance
akash query bank balances $(akash keys show invisible-infra -a)

# Verify manifest syntax
akash deployment validate [manifest.yaml]

# Check for bid availability
akash query market bid list --owner $(akash keys show invisible-infra -a)
```

### Problem: High packet latency

**Investigate:**
```bash
# Check network path
traceroute -n [destination]

# Monitor WireGuard traffic
wg show wg0 transfer

# Review relay metrics
curl http://localhost:9100/metrics | grep latency
```

---

## 📚 Appendix C: Additional Resources

### Privacy VPS Providers (Alternatives)

- **Privex** - https://www.privex.io/ (Sweden, crypto-friendly)
- **Bahnhof** - https://www.bahnhof.net/ (Sweden, privacy-focused)
- **PRQ** - https://www.prq.se/ (Sweden, Pirate Bay's original host)
- **CyberBunker** - https://www.cyberbunker.com/ (Netherlands, controversial)

### Decentralized Infrastructure

- **Akash Docs** - https://docs.akash.network/
- **Flux Guide** - https://runonflux.io/getstarted
- **Storj Documentation** - https://docs.storj.io/

### Privacy & Security

- **Tor Project** - https://www.torproject.org/
- **Monero** - https://www.getmonero.org/
- **WireGuard** - https://www.wireguard.com/
- **OWASP Top 10** - https://owasp.org/www-project-top-ten/

### Monitoring & Observability

- **Prometheus Docs** - https://prometheus.io/docs/
- **Grafana Dashboards** - https://grafana.com/grafana/dashboards/
- **Akash Console** - https://cloudmos.io/deploy

---

## 🎯 Next Steps After Deployment

Once infrastructure is operational:

1. **Integrate with Invisible client apps**
   - Update client configurations with entry node IPs
   - Distribute WireGuard client configs
   - Test end-to-end messaging through production network

2. **Implement cover traffic**
   - Deploy dummy packet generators
   - Verify constant-rate traffic streams
   - Monitor network flow patterns

3. **Enable Shamir Secret Sharing**
   - Configure K-of-N fragmentation
   - Test multi-path packet routing
   - Verify reconstruction logic

4. **Launch beta testing**
   - Invite trusted users
   - Monitor performance under real load
   - Gather feedback on latency/UX

5. **Community node program**
   - Create incentive structure (crypto rewards?)
   - Publish node operator guide
   - Build decentralized node registry

6. **External security audit**
   - Engage Trail of Bits or Cure53
   - Remediate findings
   - Publish audit results for transparency

---

**End of Deployment Guide**

🧙 **The BMad Master has spoken.** This infrastructure will make the NSA cry. Follow this guide, and Invisible will have the most privacy-respecting relay network outside of Tor itself.

Questions, Lawless? Need clarification on any phase? The BMad Master stands ready.

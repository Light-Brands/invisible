# Invisible Infrastructure - Server Management Framework

**Version:** 1.0
**Date:** 2026-02-14
**Purpose:** Centralized server inventory and automated cluster management

---

## 🎯 Overview

This framework provides:
- ✅ **Centralized server inventory** - Single source of truth for all nodes
- ✅ **SSH key management** - Secure, auditable access control
- ✅ **Automated provisioning** - Ansible playbooks for zero-touch deployment
- ✅ **Remote management** - Execute commands across entire fleet
- ✅ **Security compliance** - Tor-routed access, key rotation, audit logs

---

## 📁 Directory Structure

```
~/invisible-infra-ops/
├── inventory/
│   ├── production.yml          # Production server inventory
│   ├── staging.yml              # Staging environment (optional)
│   └── group_vars/              # Group-specific variables
│       ├── entry_nodes.yml
│       ├── mix_nodes.yml
│       └── exit_nodes.yml
├── ssh-keys/
│   ├── infrastructure-master    # Master SSH private key (highly protected)
│   ├── infrastructure-master.pub
│   ├── node-specific/           # Node-specific keys (optional)
│   └── README.md                # Key management documentation
├── playbooks/
│   ├── 00-bootstrap.yml         # Initial server setup
│   ├── 01-harden.yml            # Security hardening
│   ├── 02-wireguard.yml         # WireGuard VPN setup
│   ├── 03-deploy-relay.yml      # Deploy Invisible relay software
│   ├── 04-monitoring.yml        # Setup monitoring agents
│   └── maintenance/
│       ├── update-all.yml       # System updates
│       ├── rotate-keys.yml      # SSH key rotation
│       └── health-check.yml     # Cluster health check
├── scripts/
│   ├── provision-new-node.sh    # Quick provision script
│   ├── connect-node.sh          # Helper for SSH connections
│   └── cluster-command.sh       # Execute command on all nodes
├── configs/
│   └── ansible.cfg              # Ansible configuration
└── docs/
    └── RUNBOOK.md               # Operational procedures
```

---

## 🔑 Phase 1: SSH Key Infrastructure

### 1.1 Generate Master SSH Key

This master key will be used by automation tools (Ansible) to access all servers.

```bash
cd ~/invisible-infra-ops
mkdir -p ssh-keys

# Generate ED25519 key (more secure than RSA)
ssh-keygen -t ed25519 \
    -C "invisible-infra-automation@$(date +%Y%m%d)" \
    -f ssh-keys/infrastructure-master \
    -N ""  # No passphrase for automation (protected by file permissions)

# Secure the private key
chmod 600 ssh-keys/infrastructure-master
chmod 644 ssh-keys/infrastructure-master.pub

# Display public key for deployment
cat ssh-keys/infrastructure-master.pub
```

**Security Notes:**
- ✅ Master key has NO passphrase (required for automation)
- ✅ Protected by strict file permissions (600)
- ✅ Stored ONLY on your local management machine
- ✅ NEVER committed to git (add to .gitignore)
- ✅ Backed up encrypted in KeePassXC
- ✅ Rotated every 90 days

### 1.2 Alternative: Node-Specific Keys (Higher Security)

If you want granular access control:

```bash
# Generate unique key per node
for node in relay-entry-is-01 relay-entry-nv-01 relay-exit-is-01; do
    ssh-keygen -t ed25519 \
        -C "invisible-${node}@$(date +%Y%m%d)" \
        -f ssh-keys/node-specific/${node} \
        -N ""
    chmod 600 ssh-keys/node-specific/${node}
done
```

**Trade-off:**
- ✅ **More secure:** Compromised key only affects one node
- ❌ **More complex:** Need to manage many keys
- **Recommendation:** Use master key for initial setup, consider node-specific keys for production

### 1.3 Deploy SSH Keys to Servers

**Manual deployment (first-time setup):**

```bash
#!/bin/bash
# deploy-ssh-keys.sh

# Master public key
PUBKEY=$(cat ssh-keys/infrastructure-master.pub)

# List of servers (IP addresses from VPS providers)
declare -A SERVERS=(
    ["relay-entry-is-01"]="1.2.3.4"
    ["relay-entry-nv-01"]="5.6.7.8"
    ["relay-entry-ro-01"]="9.10.11.12"
    ["relay-exit-is-01"]="13.14.15.16"
    ["relay-exit-se-01"]="17.18.19.20"
    ["relay-exit-fi-01"]="21.22.23.24"
)

# Deploy key to each server (via Tor)
for NODE_NAME in "${!SERVERS[@]}"; do
    NODE_IP=${SERVERS[$NODE_NAME]}

    echo "Deploying key to $NODE_NAME ($NODE_IP)..."

    # SSH through Tor, add public key
    ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
        root@${NODE_IP} \
        "mkdir -p ~/.ssh && echo '${PUBKEY}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys && chmod 700 ~/.ssh"

    echo "✓ $NODE_NAME configured"
done

echo "All keys deployed!"
```

**Run deployment:**

```bash
chmod +x deploy-ssh-keys.sh
./deploy-ssh-keys.sh
```

---

## 📋 Phase 2: Ansible Inventory Setup

### 2.1 Create Ansible Configuration

Create `configs/ansible.cfg`:

```ini
[defaults]
# Inventory file
inventory = inventory/production.yml

# SSH settings
host_key_checking = False
private_key_file = ssh-keys/infrastructure-master
remote_user = root
timeout = 30

# Performance
forks = 10
gathering = smart
fact_caching = jsonfile
fact_caching_connection = /tmp/ansible-facts
fact_caching_timeout = 3600

# Logging
log_path = logs/ansible.log

# Privilege escalation (already root, but good practice)
become = True
become_method = sudo
become_user = root

# SSH connection settings
[ssh_connection]
ssh_args = -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" -o ControlMaster=auto -o ControlPersist=60s
pipelining = True
control_path = /tmp/ansible-ssh-%%h-%%p-%%r
```

### 2.2 Create Production Inventory

Create `inventory/production.yml`:

```yaml
---
# Invisible Infrastructure - Production Inventory

all:
  vars:
    ansible_user: root
    ansible_ssh_private_key_file: ssh-keys/infrastructure-master
    ansible_python_interpreter: /usr/bin/python3

    # Global environment variables
    invisible_env: production
    invisible_version: "0.1.0"

  children:
    # Entry Nodes (Layer 1)
    entry_nodes:
      hosts:
        relay-entry-is-01:
          ansible_host: 1.2.3.4  # Replace with actual IP from Njalla
          location: iceland
          provider: njalla
          node_layer: 1
          wireguard_ip: 10.20.30.1

        relay-entry-nv-01:
          ansible_host: 5.6.7.8
          location: nevis
          provider: njalla
          node_layer: 1
          wireguard_ip: 10.20.30.2

        relay-entry-ro-01:
          ansible_host: 9.10.11.12
          location: romania
          provider: flokinet
          node_layer: 1
          wireguard_ip: 10.20.30.3

      vars:
        node_type: entry
        relay_role: ingress

    # Exit Nodes (Layer 5)
    exit_nodes:
      hosts:
        relay-exit-is-01:
          ansible_host: 13.14.15.16
          location: iceland
          provider: njalla
          node_layer: 5
          wireguard_ip: 10.20.30.100

        relay-exit-se-01:
          ansible_host: 17.18.19.20
          location: sweden
          provider: njalla
          node_layer: 5
          wireguard_ip: 10.20.30.101

        relay-exit-fi-01:
          ansible_host: 21.22.23.24
          location: finland
          provider: flokinet
          node_layer: 5
          wireguard_ip: 10.20.30.102

      vars:
        node_type: exit
        relay_role: egress

    # Mix Nodes (Layers 2-4) - Akash-based
    # Note: These are managed differently (via Akash CLI)
    # But we can still include them for documentation
    mix_nodes:
      hosts:
        # Layer 2
        mix-layer2-01:
          ansible_host: akash-dynamic-01  # Placeholder, populated at runtime
          location: akash-us-west
          provider: akash
          node_layer: 2
          wireguard_ip: 10.20.30.10
          akash_dseq: 1234567  # Deployment sequence number

        mix-layer2-02:
          ansible_host: akash-dynamic-02
          location: akash-eu-central
          provider: akash
          node_layer: 2
          wireguard_ip: 10.20.30.11
          akash_dseq: 1234568

        # Layer 3
        mix-layer3-01:
          ansible_host: akash-dynamic-03
          location: akash-asia-east
          provider: akash
          node_layer: 3
          wireguard_ip: 10.20.30.20
          akash_dseq: 1234569

        # ... (add all 15 mix nodes)

      vars:
        node_type: mix
        relay_role: relay

    # Monitoring Infrastructure
    monitoring:
      hosts:
        monitoring-prometheus:
          ansible_host: akash-monitoring-01
          provider: akash
          akash_dseq: 9999001

        monitoring-grafana:
          ansible_host: akash-monitoring-02
          provider: akash
          akash_dseq: 9999002

      vars:
        node_type: monitoring

    # Logical groupings
    vps_nodes:
      children:
        entry_nodes:
        exit_nodes:

    akash_nodes:
      children:
        mix_nodes:
        monitoring:
```

### 2.3 Create Group Variables

**`inventory/group_vars/entry_nodes.yml`:**

```yaml
---
# Entry Node Configuration

# Firewall rules
ufw_rules:
  - { port: 22, proto: tcp, comment: "SSH" }
  - { port: 8080, proto: tcp, comment: "Relay HTTP" }
  - { port: 9100, proto: udp, comment: "WireGuard" }

# WireGuard peers (connect to Layer 2 mix nodes)
wireguard_peers:
  - name: mix-layer2-01
    public_key: "{{ mix_layer2_01_pubkey }}"
    endpoint: "{{ mix_layer2_01_ip }}:9100"
    allowed_ips: "10.20.30.10/32"

  - name: mix-layer2-02
    public_key: "{{ mix_layer2_02_pubkey }}"
    endpoint: "{{ mix_layer2_02_ip }}:9100"
    allowed_ips: "10.20.30.11/32"

# Relay software config
relay_config:
  max_connections: 1000
  packet_buffer_size: 4096
  enable_cover_traffic: true
  cover_traffic_rate: 10  # packets/second
```

**`inventory/group_vars/exit_nodes.yml`:**

```yaml
---
# Exit Node Configuration

ufw_rules:
  - { port: 22, proto: tcp, comment: "SSH" }
  - { port: 8080, proto: tcp, comment: "Relay HTTP" }
  - { port: 9100, proto: udp, comment: "WireGuard" }
  - { port: 443, proto: tcp, comment: "HTTPS egress" }

wireguard_peers:
  - name: mix-layer4-01
    public_key: "{{ mix_layer4_01_pubkey }}"
    endpoint: "{{ mix_layer4_01_ip }}:9100"
    allowed_ips: "10.20.30.40/32"
  # ... (connect to Layer 4/5 mix nodes)

relay_config:
  max_connections: 2000
  enable_dead_drop_storage: true
  storj_access_grant: "{{ storj_access_grant }}"
```

### 2.4 Test Inventory

```bash
# Verify inventory syntax
ansible-inventory -i inventory/production.yml --list

# Test connectivity to all VPS nodes
ansible vps_nodes -m ping

# Expected output:
# relay-entry-is-01 | SUCCESS => { "ping": "pong" }
# relay-entry-nv-01 | SUCCESS => { "ping": "pong" }
# ...
```

---

## 🤖 Phase 3: Ansible Playbooks

### 3.1 Bootstrap Playbook

Create `playbooks/00-bootstrap.yml`:

```yaml
---
- name: Bootstrap Invisible Relay Nodes
  hosts: vps_nodes
  gather_facts: yes

  tasks:
    - name: Update apt cache
      apt:
        update_cache: yes
        cache_valid_time: 3600

    - name: Install base packages
      apt:
        name:
          - curl
          - git
          - vim
          - htop
          - tmux
          - python3
          - python3-pip
          - ufw
          - fail2ban
          - unattended-upgrades
        state: present

    - name: Set timezone to UTC
      timezone:
        name: UTC

    - name: Create invisible user
      user:
        name: invisible
        shell: /bin/bash
        groups: sudo
        append: yes
        create_home: yes

    - name: Configure automatic security updates
      copy:
        dest: /etc/apt/apt.conf.d/50unattended-upgrades
        content: |
          Unattended-Upgrade::Allowed-Origins {
              "${distro_id}:${distro_codename}-security";
          };
          Unattended-Upgrade::AutoFixInterruptedDpkg "true";
          Unattended-Upgrade::Remove-Unused-Dependencies "true";
          Unattended-Upgrade::Automatic-Reboot "true";
          Unattended-Upgrade::Automatic-Reboot-Time "03:00";

    - name: Disable swap
      shell: |
        swapoff -a
        sed -i '/ swap / s/^/#/' /etc/fstab
      args:
        warn: false

    - name: Create tmpfs mounts for RAM-only storage
      lineinfile:
        path: /etc/fstab
        line: "{{ item }}"
      loop:
        - "tmpfs /tmp tmpfs defaults,noatime,mode=1777 0 0"
        - "tmpfs /var/tmp tmpfs defaults,noatime,mode=1777 0 0"

    - name: Mount tmpfs
      shell: mount -a
      args:
        warn: false

    - name: Display node info
      debug:
        msg: "Node {{ inventory_hostname }} bootstrapped successfully"
```

### 3.2 Security Hardening Playbook

Create `playbooks/01-harden.yml`:

```yaml
---
- name: Harden Invisible Relay Nodes
  hosts: vps_nodes
  gather_facts: yes

  tasks:
    - name: Configure SSH - Disable password auth
      lineinfile:
        path: /etc/ssh/sshd_config
        regexp: "{{ item.regexp }}"
        line: "{{ item.line }}"
      loop:
        - { regexp: '^#?PasswordAuthentication', line: 'PasswordAuthentication no' }
        - { regexp: '^#?PubkeyAuthentication', line: 'PubkeyAuthentication yes' }
        - { regexp: '^#?PermitRootLogin', line: 'PermitRootLogin prohibit-password' }
        - { regexp: '^#?X11Forwarding', line: 'X11Forwarding no' }
      notify: Restart SSH

    - name: Configure firewall rules
      ufw:
        rule: "{{ item.rule | default('allow') }}"
        port: "{{ item.port }}"
        proto: "{{ item.proto }}"
        comment: "{{ item.comment }}"
      loop: "{{ ufw_rules }}"

    - name: Enable UFW
      ufw:
        state: enabled
        policy: deny

    - name: Configure fail2ban
      copy:
        dest: /etc/fail2ban/jail.local
        content: |
          [sshd]
          enabled = true
          port = 22
          filter = sshd
          logpath = /var/log/auth.log
          maxretry = 3
          bantime = 3600
          findtime = 600
      notify: Restart fail2ban

    - name: Install Docker
      shell: |
        curl -fsSL https://get.docker.com -o /tmp/get-docker.sh
        sh /tmp/get-docker.sh
        rm /tmp/get-docker.sh
      args:
        creates: /usr/bin/docker

    - name: Configure Docker (no logging)
      copy:
        dest: /etc/docker/daemon.json
        content: |
          {
            "log-driver": "none",
            "storage-driver": "overlay2"
          }
      notify: Restart Docker

    - name: Add invisible user to docker group
      user:
        name: invisible
        groups: docker
        append: yes

  handlers:
    - name: Restart SSH
      service:
        name: sshd
        state: restarted

    - name: Restart fail2ban
      service:
        name: fail2ban
        state: restarted

    - name: Restart Docker
      service:
        name: docker
        state: restarted
```

### 3.3 WireGuard Setup Playbook

Create `playbooks/02-wireguard.yml`:

```yaml
---
- name: Configure WireGuard Mesh Network
  hosts: vps_nodes
  gather_facts: yes

  tasks:
    - name: Install WireGuard
      apt:
        name: wireguard
        state: present

    - name: Generate WireGuard private key
      shell: wg genkey
      register: wg_privkey
      changed_when: false
      no_log: true  # Don't log private keys

    - name: Generate WireGuard public key
      shell: echo "{{ wg_privkey.stdout }}" | wg pubkey
      register: wg_pubkey
      changed_when: false

    - name: Save public key to local inventory
      local_action:
        module: lineinfile
        path: "inventory/wireguard-keys.yml"
        line: "{{ inventory_hostname }}_pubkey: {{ wg_pubkey.stdout }}"
        create: yes

    - name: Create WireGuard config
      template:
        src: templates/wg0.conf.j2
        dest: /etc/wireguard/wg0.conf
        mode: '0600'

    - name: Enable and start WireGuard
      systemd:
        name: wg-quick@wg0
        enabled: yes
        state: started

    - name: Display WireGuard status
      shell: wg show
      register: wg_status
      changed_when: false

    - debug:
        msg: "{{ wg_status.stdout_lines }}"
```

**WireGuard config template** `playbooks/templates/wg0.conf.j2`:

```jinja2
[Interface]
PrivateKey = {{ wg_privkey.stdout }}
Address = {{ wireguard_ip }}/24
ListenPort = 9100

{% for peer in wireguard_peers %}
[Peer]
PublicKey = {{ peer.public_key }}
AllowedIPs = {{ peer.allowed_ips }}
{% if peer.endpoint is defined %}
Endpoint = {{ peer.endpoint }}
{% endif %}
PersistentKeepalive = 25

{% endfor %}
```

### 3.4 Deploy Relay Software Playbook

Create `playbooks/03-deploy-relay.yml`:

```yaml
---
- name: Deploy Invisible Relay Software
  hosts: vps_nodes
  gather_facts: yes

  vars:
    relay_image: "yourregistry.com/invisible/relay-node:{{ invisible_version }}"

  tasks:
    - name: Create relay config directory
      file:
        path: /opt/invisible
        state: directory
        owner: invisible
        group: invisible
        mode: '0755'

    - name: Copy docker-compose file
      template:
        src: "templates/docker-compose-{{ node_type }}.yml.j2"
        dest: /opt/invisible/docker-compose.yml
        owner: invisible
        group: invisible

    - name: Pull relay Docker image
      docker_image:
        name: "{{ relay_image }}"
        source: pull

    - name: Start relay container
      docker_compose:
        project_src: /opt/invisible
        state: present
      become_user: invisible

    - name: Wait for relay to be healthy
      wait_for:
        port: 8080
        delay: 5
        timeout: 60

    - name: Check relay health endpoint
      uri:
        url: http://localhost:8080/health
        return_content: yes
      register: health_check
      retries: 3
      delay: 5

    - debug:
        msg: "Relay {{ inventory_hostname }} is healthy: {{ health_check.json }}"
```

---

## 🚀 Phase 4: Cluster Management Scripts

### 4.1 Provision New Node

Create `scripts/provision-new-node.sh`:

```bash
#!/bin/bash
# Quick provision script for new relay node

set -euo pipefail

if [ $# -ne 3 ]; then
    echo "Usage: $0 <node-name> <node-ip> <node-type>"
    echo "Example: $0 relay-entry-de-01 10.20.30.5 entry"
    exit 1
fi

NODE_NAME=$1
NODE_IP=$2
NODE_TYPE=$3

echo "Provisioning new node: $NODE_NAME ($NODE_IP)"

# 1. Add to inventory
echo "Adding to inventory..."
cat >> inventory/production.yml <<EOF

        ${NODE_NAME}:
          ansible_host: ${NODE_IP}
          node_type: ${NODE_TYPE}
          # TODO: Fill in location, provider, etc.
EOF

# 2. Deploy SSH key
echo "Deploying SSH key..."
PUBKEY=$(cat ssh-keys/infrastructure-master.pub)
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    root@${NODE_IP} \
    "mkdir -p ~/.ssh && echo '${PUBKEY}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys"

# 3. Run bootstrap playbook
echo "Running bootstrap..."
ansible-playbook playbooks/00-bootstrap.yml --limit ${NODE_NAME}

# 4. Run hardening playbook
echo "Running hardening..."
ansible-playbook playbooks/01-harden.yml --limit ${NODE_NAME}

echo "✓ Node ${NODE_NAME} provisioned successfully!"
echo "Next steps:"
echo "  1. Update inventory with full details (location, provider, etc.)"
echo "  2. Run WireGuard setup: ansible-playbook playbooks/02-wireguard.yml --limit ${NODE_NAME}"
echo "  3. Deploy relay software: ansible-playbook playbooks/03-deploy-relay.yml --limit ${NODE_NAME}"
```

### 4.2 Connect to Node Helper

Create `scripts/connect-node.sh`:

```bash
#!/bin/bash
# Helper script to SSH into nodes via Tor

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <node-name>"
    echo "Available nodes:"
    ansible-inventory --list | jq -r '.[] | select(.hosts) | .hosts[]' 2>/dev/null | sort | uniq
    exit 1
fi

NODE_NAME=$1

# Get node IP from inventory
NODE_IP=$(ansible-inventory --host ${NODE_NAME} | jq -r '.ansible_host')

if [ "$NODE_IP" = "null" ] || [ -z "$NODE_IP" ]; then
    echo "Error: Node $NODE_NAME not found in inventory"
    exit 1
fi

echo "Connecting to $NODE_NAME ($NODE_IP) via Tor..."

ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    -i ssh-keys/infrastructure-master \
    root@${NODE_IP}
```

### 4.3 Cluster-Wide Command Execution

Create `scripts/cluster-command.sh`:

```bash
#!/bin/bash
# Execute command across cluster

set -euo pipefail

if [ $# -lt 2 ]; then
    echo "Usage: $0 <host-pattern> <command>"
    echo "Examples:"
    echo "  $0 entry_nodes 'systemctl status wg-quick@wg0'"
    echo "  $0 all 'docker ps'"
    echo "  $0 relay-entry-is-01 'wg show'"
    exit 1
fi

HOST_PATTERN=$1
shift
COMMAND="$@"

echo "Executing on $HOST_PATTERN: $COMMAND"
echo "---"

ansible ${HOST_PATTERN} -a "${COMMAND}"
```

Usage examples:

```bash
# Check WireGuard on all entry nodes
./scripts/cluster-command.sh entry_nodes 'wg show'

# Check Docker containers on all VPS nodes
./scripts/cluster-command.sh vps_nodes 'docker ps'

# System updates on all nodes
./scripts/cluster-command.sh all 'apt update && apt list --upgradable'

# Restart relay on specific node
./scripts/cluster-command.sh relay-entry-is-01 'cd /opt/invisible && docker-compose restart'
```

---

## 📊 Phase 5: Operational Workflows

### 5.1 Daily Health Check

```bash
#!/bin/bash
# daily-health-check.sh

echo "=== Invisible Infrastructure Health Check ==="
echo "Date: $(date)"
echo ""

# 1. Check node connectivity
echo "1. Node Connectivity:"
ansible all -m ping --one-line

# 2. Check WireGuard status
echo ""
echo "2. WireGuard Status:"
ansible vps_nodes -a 'wg show' | grep -A1 "interface\|peer"

# 3. Check relay containers
echo ""
echo "3. Relay Containers:"
ansible vps_nodes -a 'docker ps --format "table {{.Names}}\t{{.Status}}"'

# 4. Check system load
echo ""
echo "4. System Load:"
ansible vps_nodes -a 'uptime'

# 5. Check disk usage
echo ""
echo "5. Disk Usage:"
ansible vps_nodes -a 'df -h /'

# 6. Check for security updates
echo ""
echo "6. Security Updates Available:"
ansible vps_nodes -a 'apt list --upgradable 2>/dev/null | grep -i security | wc -l'

echo ""
echo "=== Health Check Complete ==="
```

Run daily: `./scripts/daily-health-check.sh | tee logs/health-$(date +%Y%m%d).log`

### 5.2 Update All Servers

```bash
# Update all servers safely (one at a time to avoid downtime)
ansible-playbook playbooks/maintenance/update-all.yml --forks 1
```

**`playbooks/maintenance/update-all.yml`:**

```yaml
---
- name: Update All Relay Nodes
  hosts: vps_nodes
  serial: 1  # Update one at a time

  tasks:
    - name: Update apt cache
      apt:
        update_cache: yes

    - name: Upgrade all packages
      apt:
        upgrade: safe

    - name: Check if reboot required
      stat:
        path: /var/run/reboot-required
      register: reboot_required

    - name: Reboot if required
      reboot:
        msg: "Reboot initiated by Ansible for package updates"
        reboot_timeout: 300
      when: reboot_required.stat.exists

    - name: Wait for node to come back
      wait_for_connection:
        delay: 30
        timeout: 300
      when: reboot_required.stat.exists
```

### 5.3 Deploy New Relay Version

```bash
# Update relay software across all nodes
ansible-playbook playbooks/03-deploy-relay.yml -e "invisible_version=0.2.0"
```

---

## 🔐 Phase 6: Security & Access Control

### 6.1 SSH Key Rotation (Every 90 Days)

Create `playbooks/maintenance/rotate-keys.yml`:

```yaml
---
- name: Rotate SSH Keys
  hosts: vps_nodes

  vars:
    new_key_name: "infrastructure-master-{{ ansible_date_time.date }}"

  tasks:
    - name: Generate new SSH key locally
      local_action:
        module: shell
        cmd: |
          ssh-keygen -t ed25519 \
            -C "invisible-infra-{{ ansible_date_time.date }}" \
            -f ssh-keys/{{ new_key_name }} \
            -N ""
      run_once: true

    - name: Read new public key
      local_action:
        module: slurp
        src: "ssh-keys/{{ new_key_name }}.pub"
      register: new_pubkey
      run_once: true

    - name: Add new key to authorized_keys
      authorized_key:
        user: root
        key: "{{ new_pubkey.content | b64decode }}"
        state: present

    - name: Wait for verification
      pause:
        prompt: "New key deployed. Test access with new key, then press Enter to remove old key"
      run_once: true

    - name: Remove old key from authorized_keys
      authorized_key:
        user: root
        key: "{{ lookup('file', 'ssh-keys/infrastructure-master.pub') }}"
        state: absent

    - name: Update inventory to use new key
      local_action:
        module: lineinfile
        path: configs/ansible.cfg
        regexp: '^private_key_file'
        line: "private_key_file = ssh-keys/{{ new_key_name }}"
      run_once: true
```

### 6.2 Audit Log

```bash
# Enable command logging
export PROMPT_COMMAND='RETRN_VAL=$?;logger -p local6.debug "$(whoami) [$$]: $(history 1 | sed "s/^[ ]*[0-9]\+[ ]*//" )"'

# All Ansible runs are logged to logs/ansible.log (configured in ansible.cfg)
# Review logs regularly:
tail -f logs/ansible.log
```

---

## 📖 Phase 7: AI Assistant Integration

### 7.1 Secure Credential Sharing

**⚠️ CRITICAL SECURITY CONSIDERATION:**

You should NEVER share actual SSH private keys or server IPs directly in chat with an AI assistant. Instead:

**Option A: Provide Inventory File (IPs only)**

```bash
# Share the inventory structure (without secrets)
# AI can read inventory/production.yml to understand topology
# You run the Ansible commands locally
```

**Option B: Delegate Specific Tasks**

```yaml
# You: "Run health check on all entry nodes"
# AI generates command for you to run:
ansible entry_nodes -m ping
```

**Option C: Use Jump Host Pattern**

```bash
# Set up a bastion/jump host that AI can SSH into
# Jump host has limited permissions, can only execute whitelisted commands
# AI connects to jump host, you approve commands
```

### 7.2 Command Generator Interface

For me (The BMad Master) to help you manage the cluster, you can:

1. **Share inventory structure** (I can see it in files)
2. **Ask me to generate Ansible commands** - You run them
3. **Provide command output** - I analyze and suggest fixes
4. **Describe desired state** - I generate playbooks

**Example Workflow:**

```
You: "Check if all entry nodes have WireGuard running"

Me: "Run this command:
     ansible entry_nodes -a 'systemctl status wg-quick@wg0 --no-pager'

     Paste the output and I'll analyze it"

You: [pastes output]

Me: "I see relay-entry-nv-01 has WireGuard down. To fix:
     ansible relay-entry-nv-01 -a 'systemctl start wg-quick@wg0'

     Then verify with: ansible relay-entry-nv-01 -a 'wg show'"
```

---

## ✅ Quick Start Checklist

To get this framework operational:

- [ ] Generate SSH master key
- [ ] Deploy key to all VPS nodes
- [ ] Create Ansible inventory with real IPs
- [ ] Test connectivity: `ansible all -m ping`
- [ ] Run bootstrap playbook
- [ ] Run hardening playbook
- [ ] Configure WireGuard mesh
- [ ] Deploy relay software
- [ ] Set up monitoring
- [ ] Test cluster-wide commands
- [ ] Schedule daily health checks
- [ ] Document in KeePassXC

---

## 📚 Command Reference

```bash
# Test connectivity
ansible all -m ping

# Run playbook
ansible-playbook playbooks/00-bootstrap.yml

# Run on specific group
ansible-playbook playbooks/01-harden.yml --limit entry_nodes

# Run on specific host
ansible-playbook playbooks/03-deploy-relay.yml --limit relay-entry-is-01

# Ad-hoc commands
ansible all -a 'uptime'
ansible entry_nodes -a 'docker ps'

# Dry run (check mode)
ansible-playbook playbooks/02-wireguard.yml --check

# Verbose output
ansible-playbook playbooks/03-deploy-relay.yml -vvv

# List hosts
ansible-inventory --list

# Get host vars
ansible-inventory --host relay-entry-is-01
```

---

**The BMad Master has spoken.** This framework gives you industrial-grade cluster management with security baked in. 🧙

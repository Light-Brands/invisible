# Invisible Infrastructure - Quick Reference Card

**For rapid cluster management and troubleshooting**

---

## 🚀 Initial Setup (One-Time)

```bash
# 1. Run setup script
cd /Users/lawless/Documents/invisible/_bmad-output
./setup-infrastructure-ops.sh

# 2. Generate SSH master key
cd ~/invisible-infra-ops/ssh-keys
ssh-keygen -t ed25519 -C "invisible-infra" -f infrastructure-master -N ""

# 3. Edit inventory with real IPs
vim ~/invisible-infra-ops/inventory/production.yml

# 4. Test connectivity
cd ~/invisible-infra-ops
ansible all -m ping
```

---

## 🔑 Common Ansible Commands

### Connectivity & Info

```bash
# Ping all nodes
ansible all -m ping

# Ping specific group
ansible entry_nodes -m ping

# Get system info
ansible all -m setup

# List all hosts
ansible-inventory --list
```

### Ad-Hoc Commands

```bash
# Check uptime
ansible all -a 'uptime'

# Check WireGuard status
ansible vps_nodes -a 'wg show'

# Check Docker containers
ansible vps_nodes -a 'docker ps'

# Check disk usage
ansible all -a 'df -h'

# Check memory
ansible all -a 'free -h'

# Restart service
ansible relay-entry-is-01 -a 'systemctl restart wg-quick@wg0'
```

### Playbook Execution

```bash
# Run full bootstrap
ansible-playbook playbooks/00-bootstrap.yml

# Harden all nodes
ansible-playbook playbooks/01-harden.yml

# Setup WireGuard
ansible-playbook playbooks/02-wireguard.yml

# Deploy relay software
ansible-playbook playbooks/03-deploy-relay.yml

# Run on specific group
ansible-playbook playbooks/01-harden.yml --limit entry_nodes

# Run on specific host
ansible-playbook playbooks/02-wireguard.yml --limit relay-entry-is-01

# Dry run (don't actually change anything)
ansible-playbook playbooks/03-deploy-relay.yml --check
```

---

## 🔧 Troubleshooting

### Can't connect to node

```bash
# Test Tor proxy
curl --socks5-hostname 127.0.0.1:9150 https://check.torproject.org

# Test SSH manually
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    -i ssh-keys/infrastructure-master \
    root@NODE_IP

# Check SSH key permissions
ls -l ssh-keys/infrastructure-master
# Should be: -rw------- (600)
```

### Ansible fails with "Unreachable"

```bash
# Increase verbosity to see error
ansible all -m ping -vvv

# Check inventory syntax
ansible-inventory --list | jq

# Verify IP address in inventory
ansible-inventory --host relay-entry-is-01
```

### WireGuard not working

```bash
# Check if running
ansible vps_nodes -a 'systemctl status wg-quick@wg0'

# View WireGuard logs
ansible relay-entry-is-01 -a 'journalctl -u wg-quick@wg0 -n 50'

# Test connectivity between nodes
ansible relay-entry-is-01 -a 'ping -c 3 10.20.30.10'
```

### Relay container issues

```bash
# Check container status
ansible vps_nodes -a 'docker ps -a'

# View container logs
ansible relay-entry-is-01 -a 'docker logs invisible-relay-entry'

# Restart container
ansible relay-entry-is-01 -a 'cd /opt/invisible && docker-compose restart'
```

---

## 📊 Daily Operations

### Health Check

```bash
# Quick health check
ansible all -m ping
ansible vps_nodes -a 'docker ps --filter status=running --format "{{.Names}}"'
ansible vps_nodes -a 'wg show | grep -A1 interface'

# Detailed health
ansible vps_nodes -a 'uptime'
ansible vps_nodes -a 'df -h /'
ansible vps_nodes -a 'free -h'
```

### Updates

```bash
# Check for updates
ansible all -a 'apt list --upgradable'

# Install updates (safe mode)
ansible all -m apt -a 'upgrade=safe update_cache=yes'

# Reboot if needed
ansible all -a 'test -f /var/run/reboot-required && echo REBOOT_REQUIRED || echo OK'
```

### Logs

```bash
# View Ansible logs
tail -f ~/invisible-infra-ops/logs/ansible.log

# View syslog on specific node
ansible relay-entry-is-01 -a 'tail -n 50 /var/log/syslog'

# Check auth logs
ansible relay-entry-is-01 -a 'tail -n 50 /var/log/auth.log'
```

---

## 🔐 Security Tasks

### SSH Key Rotation (Every 90 days)

```bash
# Generate new key
cd ~/invisible-infra-ops/ssh-keys
ssh-keygen -t ed25519 -C "invisible-infra-$(date +%Y%m%d)" -f infrastructure-master-new -N ""

# Deploy new key
ansible all -m authorized_key -a "user=root key='{{ lookup('file', 'ssh-keys/infrastructure-master-new.pub') }}' state=present"

# Test new key
ssh -i ssh-keys/infrastructure-master-new -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" root@NODE_IP

# Update ansible.cfg to use new key
vim configs/ansible.cfg
# Change: private_key_file = ssh-keys/infrastructure-master-new

# Remove old key
ansible all -m authorized_key -a "user=root key='{{ lookup('file', 'ssh-keys/infrastructure-master.pub') }}' state=absent"

# Archive old key
mv ssh-keys/infrastructure-master ssh-keys/archive/infrastructure-master-OLD
```

### Check for Security Updates

```bash
# List security updates
ansible all -a 'apt list --upgradable 2>/dev/null | grep -i security'

# Install security updates only
ansible all -m apt -a 'upgrade=safe update_cache=yes'
```

---

## 🆘 Emergency Procedures

### Node Compromised

```bash
# 1. Isolate node (remove from WireGuard mesh)
ansible relay-compromised-node -a 'systemctl stop wg-quick@wg0'

# 2. Stop relay software
ansible relay-compromised-node -a 'docker-compose -f /opt/invisible/docker-compose.yml down'

# 3. Pull logs for forensics
ansible relay-compromised-node -a 'journalctl --since "24 hours ago" > /tmp/forensics.log'
scp -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" root@NODE_IP:/tmp/forensics.log ./logs/

# 4. Destroy node
# (Via provider dashboard - Njalla/FlokiNET)

# 5. Deploy replacement
./scripts/provision-new-node.sh relay-entry-xx-NEW NEW_IP entry
```

### Mass Node Failure

```bash
# Check which nodes are down
ansible all -m ping --one-line

# Restart all relay containers
ansible vps_nodes -a 'docker-compose -f /opt/invisible/docker-compose.yml restart'

# Restart WireGuard on all nodes
ansible vps_nodes -a 'systemctl restart wg-quick@wg0'
```

---

## 📱 Quick Connect to Nodes

```bash
# Helper script
./scripts/connect-node.sh relay-entry-is-01

# Or manually
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" \
    -i ssh-keys/infrastructure-master \
    root@NODE_IP
```

---

## 💡 Pro Tips

1. **Always test with --check first**
   ```bash
   ansible-playbook playbooks/03-deploy-relay.yml --check
   ```

2. **Use --limit for safety**
   ```bash
   # Test on one node first
   ansible-playbook playbooks/update.yml --limit relay-entry-is-01
   ```

3. **Save common commands as scripts**
   ```bash
   echo 'ansible all -m ping' > ~/bin/cluster-ping
   chmod +x ~/bin/cluster-ping
   ```

4. **Use -vvv for debugging**
   ```bash
   ansible relay-entry-is-01 -m ping -vvv
   ```

5. **Check before you wreck**
   ```bash
   # Before: ansible all -a 'rm -rf /important'
   # Do: ansible all -a 'ls -la /important'  # Verify path first!
   ```

---

## 📞 Help & Resources

- **Full Guide:** `~/invisible-infra-ops/docs/SERVER_MANAGEMENT_FRAMEWORK.md`
- **Ansible Docs:** https://docs.ansible.com/
- **Inventory Location:** `~/invisible-infra-ops/inventory/production.yml`
- **Logs:** `~/invisible-infra-ops/logs/ansible.log`

---

**Print this and keep it handy!** 🧙

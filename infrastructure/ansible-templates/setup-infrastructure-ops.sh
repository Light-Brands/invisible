#!/bin/bash
# Setup script for Invisible Infrastructure Operations
# This creates the directory structure and copies starter files

set -euo pipefail

INFRA_OPS_DIR="$HOME/invisible-infra-ops"

echo "🧙 The BMad Master's Infrastructure Setup Script"
echo "================================================"
echo ""

# Check if directory already exists
if [ -d "$INFRA_OPS_DIR" ]; then
    echo "⚠️  Directory $INFRA_OPS_DIR already exists."
    read -p "Do you want to continue and potentially overwrite files? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

echo "Creating infrastructure ops directory structure..."

# Create directory structure
mkdir -p "$INFRA_OPS_DIR"/{inventory,ssh-keys,playbooks/{maintenance,templates},scripts,configs,logs,docs}

echo "✓ Directory structure created"

# Copy starter files
echo "Copying starter configuration files..."

cp ansible.cfg "$INFRA_OPS_DIR/configs/"
cp inventory-template.yml "$INFRA_OPS_DIR/inventory/production.yml"

echo "✓ Configuration files copied"

# Create placeholder files
echo "Creating placeholder files..."

cat > "$INFRA_OPS_DIR/ssh-keys/README.md" <<'EOF'
# SSH Keys for Infrastructure Management

## Master Key

Generate the master SSH key:

```bash
ssh-keygen -t ed25519 \
    -C "invisible-infra-automation@$(date +%Y%m%d)" \
    -f infrastructure-master \
    -N ""

chmod 600 infrastructure-master
chmod 644 infrastructure-master.pub
```

## Security Notes

- **NEVER commit private keys to git**
- Back up keys encrypted in KeePassXC
- Rotate keys every 90 days
- Use unique keys per environment (prod/staging)

## Key Deployment

Deploy to servers:

```bash
ssh-copy-id -i infrastructure-master.pub -o ProxyCommand="nc -X 5 -x 127.0.0.1:9150 %h %p" root@SERVER_IP
```
EOF

cat > "$INFRA_OPS_DIR/.gitignore" <<'EOF'
# SSH Private Keys (NEVER commit these!)
ssh-keys/*
!ssh-keys/README.md
!ssh-keys/.gitkeep

# Ansible sensitive files
inventory/*vault*
*vault_pass*
*.secret

# Logs
logs/*.log

# Temporary files
*.retry
.ansible/
__pycache__/
*.pyc

# KeePassXC database (keep encrypted backups elsewhere)
*.kdbx

# Node facts cache
/tmp/ansible-facts/
EOF

touch "$INFRA_OPS_DIR/ssh-keys/.gitkeep"
touch "$INFRA_OPS_DIR/logs/.gitkeep"

echo "✓ Placeholder files created"

# Set proper permissions
chmod 700 "$INFRA_OPS_DIR/ssh-keys"
chmod 755 "$INFRA_OPS_DIR"/{inventory,playbooks,scripts,configs,logs,docs}

echo "✓ Permissions set"

# Create initial git repository
echo ""
read -p "Initialize git repository? (Y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    cd "$INFRA_OPS_DIR"
    git init
    git add .gitignore ssh-keys/README.md configs/ansible.cfg inventory/production.yml
    git commit -m "🔧 Initial infrastructure ops setup"
    echo "✓ Git repository initialized"
fi

echo ""
echo "================================================"
echo "✅ Infrastructure ops workspace ready!"
echo ""
echo "Location: $INFRA_OPS_DIR"
echo ""
echo "Next steps:"
echo "  1. cd $INFRA_OPS_DIR"
echo "  2. Generate SSH master key (see ssh-keys/README.md)"
echo "  3. Edit inventory/production.yml with your server IPs"
echo "  4. Test connectivity: ansible all -m ping"
echo ""
echo "📚 Full guide: $(pwd)/SERVER_MANAGEMENT_FRAMEWORK.md"
echo ""
echo "🧙 The BMad Master wishes you secure infrastructure!"

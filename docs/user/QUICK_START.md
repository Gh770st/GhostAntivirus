# ⚡ GhostAntivirus Quick Start Guide

Get up and running with GhostAntivirus in 5 minutes!

---

## 🚀 Installation (Choose One)

### Option 1: Docker (Easiest - Recommended)

```bash
# 1. Clone and enter directory
git clone https://github.com/witerdev/GhostAntivirus.git
cd GhostAntivirus

# 2. Configure environment
cp .env.production.example .env

# 3. Deploy (one command!)
./scripts/deploy-production.sh

# 4. Open browser
# http://localhost
```

### Option 2: Docker Compose

```bash
# Start all services
docker-compose -f docker-compose.production.yml up -d

# Access dashboard
# http://localhost
```

### Option 3: Development Mode

```bash
# Terminal 1: Core Engine
cd core && cargo run

# Terminal 2: AI Engine  
cd ai-engine && python src/main.py

# Terminal 3: Network Guard
cd network-guard && go run main.go

# Terminal 4: Web Dashboard
cd web-ui && npm run dev
```

---

## 🔐 First Login

**Default Credentials:**
```
Username: admin
Password: admin123
```

⚠️ **Change password immediately after first login!**

---

## ✅ Quick Setup (3 Steps)

### Step 1: Change Password
1. Click profile icon (top-right)
2. Settings → Security → Change Password
3. Save new password

### Step 2: Run First Scan
1. Go to Scanner page
2. Click "Start Full Scan"
3. Wait for completion (~30 min)
4. Quarantine any threats found

### Step 3: Enable Protection
1. Go to Settings
2. Enable "Real-Time Protection"
3. Enable "Auto-Update"
4. Save settings

**Done! You're protected! 🛡️**

---

## 📊 Dashboard Overview

### Main Sections:

**Scanner** 🔍
- Quick Scan (5-10 min)
- Full Scan (30-60 min)
- Custom Scan

**Threats** 🦠
- View detected threats
- Quarantine management
- Threat history

**Firewall** 🔥
- Network monitoring
- Block/Allow rules
- Active connections

**VPN** 🌐
- Connect to VPN
- Change server
- Privacy protection

**Settings** ⚙️
- Protection settings
- Update settings
- Notifications

---

## 🎯 Common Tasks

### Scan a File
```
Right-click file → Scan with GhostAntivirus
```

### Block a Website
```
Firewall → Rules → Add Rule → Block URL
```

### Connect to VPN
```
VPN → Select Server → Connect
```

### Schedule Scans
```
Settings → Scanner → Auto-Scan → Daily
```

### View Quarantine
```
Threats → Quarantine → View Files
```

---

## 🔧 Quick Troubleshooting

**Service won't start?**
```bash
docker-compose restart
```

**Slow scan?**
```
Use Quick Scan instead of Full Scan
```

**False positive?**
```
Quarantine → Restore → Add to Exclusions
```

**Can't connect to VPN?**
```
Try different server or restart service
```

---

## 📞 Need Help?

- **Documentation:** [USER_GUIDE.md](USER_GUIDE.md)
- **Integration:** [INTEGRATION_GUIDE.md](INTEGRATION_GUIDE.md)
- **Support:** support@ghostantivirus.com
- **Issues:** [GitHub Issues](https://github.com/witerdev/GhostAntivirus/issues)

---

## 🎉 You're All Set!

**Next Steps:**
1. ✅ Explore the dashboard
2. ✅ Run your first scan
3. ✅ Install browser extension
4. ✅ Download mobile app
5. ✅ Read full [User Guide](USER_GUIDE.md)

**Stay protected! 🛡️**

---

*For detailed instructions, see [USER_GUIDE.md](USER_GUIDE.md)*
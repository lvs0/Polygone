# 🌸 POLYGONE - Installation & Testing

## 🚀 One-Line Universal Installer

**Compatible avec Windows, macOS, Linux - Une seule commande!**

### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/lvs0/Polygone/main/install-polygone.sh | bash
```

### Windows
```cmd
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/lvs0/Polygone/main/install-polygone.sh' -OutFile 'install-polygone.sh'; .\install-polygone.sh"
```

---

## 🧪 Test Complet Automatisé

**Testez tous les composants avec une seule commande:**

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
./test-polygone.sh
```

---

## 📋 Ce Qui Est Testé

### ✅ **Infrastructure Docker**
- Docker Daemon
- Docker Compose
- Tous les conteneurs POLYGONE
- Configuration réseau

### ✅ **Connectivité Réseau**
- API Polygone Core (port 4000)
- API Polygone Petals (port 4003)
- API MAX Assistant (port 8000)
- API Ollama (port 11434)
- Proxy SOCKS5 Polygone Hide (port 1080)
- Dashboard Monitoring (port 9090)

### ✅ **Fonctionnalités IA**
- Liste modèles Ollama
- Chat API MAX
- Inférence distribuée Petals
- Intégration complète stack

### ✅ **Sécurité Post-Quantique**
- Certificats SSL auto-générés
- Chiffrement ML-KEM-1024
- Politiques enterprise
- Logs d'audit immuables

### ✅ **Performance**
- Temps réponse API <1s
- Temps réponse MAX <2s
- Monitoring temps réel
- Métriques Prometheus

### ✅ **Persistance Données**
- Base de données MAX
- Logs structurés
- Backups automatiques
- Configuration persistante

### ✅ **Mobile & Accessibilité**
- Interface responsive
- Support multi-langues
- WCAG 2.1 AA compliance
- Touch gestures

### ✅ **Enterprise**
- Monitoring Grafana
- Alertes configurables
- Multi-tenant isolation
- Scalabilité horizontale

---

## 🎯 Résultats Attendus

### **Installation Succès**
```
🌸 POLYGONE v2.0.0 Installation Complete! 🌸

📊 Access URLs:
   🌐 Dashboard: http://localhost:9090
   🤖 MAX AI: http://localhost:8000
   🔐 Polygone Hide: socks5://localhost:1080
   📡 Polygone Petals: http://localhost:4003
```

### **Test Suite Succès**
```
📊 POLYGONE Test Report
========================
Total Tests: 25
Passed: 25
Failed: 0
Success Rate: 100%

🎉 ALL TESTS PASSED! POLYGONE is ready for deployment.
```

---

## 🔧 Configuration Rapide

Après installation, configurez rapidement:

```bash
# Ouvrir configuration
nano ~/.polygone/config.json

# Personnaliser modèles IA
{
  "ai": {
    "model": "qwen2.5:7b",
    "temperature": 0.7
  }
}

# Redémarrer services
cd ~/.polygone && docker-compose restart
```

---

## 🚨 Dépannage

### Services ne démarrent pas:
```bash
cd ~/.polygone && docker-compose logs
```

### Problèmes réseau:
```bash
# Vérifier ports
netstat -tulpn | grep -E ':(4000|4003|8000|1080|9090|11434)'

# Tester connectivité
curl -v http://localhost:4000/health
```

### Problèmes Docker:
```bash
# Vérifier Docker
docker --version
docker-compose version

# Nettoyer et réinstaller
docker system prune -a
cd ~/.polygone && docker-compose down && docker-compose up -d
```

---

## 📚 Documentation Complète

- [Documentation Technique](https://docs.polygone.ai)
- [API Reference](https://api.polygone.ai)
- [Guides Enterprise](https://enterprise.polygone.ai)
- [Communauté](https://community.polygone.ai)

---

## 🌍 Déploiement

### **Local Testing**
```bash
./test-polygone.sh
```

### **Staging**
```bash
docker-compose -f docker-compose.staging.yml up -d
```

### **Production**
```bash
# Kubernetes
kubectl apply -f k8s/enterprise-deployment.yaml

# Terraform
cd terraform && terraform apply
```

---

## 🎯 Prêt Pour Production

✅ **Installation one-line**  
✅ **Test complet automatisé**  
✅ **Multi-plateforme**  
✅ **Enterprise-ready**  
✅ **Sécurité post-quantique**  
✅ **Performance optimisée**  

**POLYGONE v2.0.0 est prêt pour déploiement immédiat!**

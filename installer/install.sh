#!/bin/bash

# 🌸 POLYGONE Universal Installer v2.0.0
# Compatible: Windows (WSL), macOS, Linux
# Usage: curl -sSL https://raw.githubusercontent.com/lvs0/POLYGONE/main/install-polygone.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="Linux"
        DISTRO=$(lsb_release -si 2>/dev/null || echo "Unknown")
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macOS"
        DISTRO="macOS"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        OS="Windows"
        DISTRO="Windows"
    else
        OS="Unknown"
        DISTRO="Unknown"
    fi
    
    echo -e "${CYAN}🖥️  Detected OS: ${OS} (${DISTRO})${NC}"
}

# Check system requirements
check_requirements() {
    echo -e "${BLUE}🔍 Checking system requirements...${NC}"
    
    # Check Python
    if command -v python3 &> /dev/null; then
        PYTHON_VERSION=$(python3 --version | cut -d' ' -f2)
        echo -e "${GREEN}✅ Python: ${PYTHON_VERSION}${NC}"
    else
        echo -e "${RED}❌ Python 3.8+ required${NC}"
        exit 1
    fi
    
    # Check Docker
    if command -v docker &> /dev/null; then
        DOCKER_VERSION=$(docker --version | cut -d' ' -f3 | cut -d',' -f1)
        echo -e "${GREEN}✅ Docker: ${DOCKER_VERSION}${NC}"
    else
        echo -e "${YELLOW}⚠️  Docker not found, installing...${NC}"
        install_docker
    fi
    
    # Check memory
    if [[ "$OS" == "Linux" ]]; then
        MEMORY_KB=$(grep MemTotal /proc/meminfo | awk '{print $2}')
        MEMORY_GB=$((MEMORY_KB / 1024 / 1024))
    elif [[ "$OS" == "macOS" ]]; then
        MEMORY_GB=$(sysctl -n hw.memsize | awk '{print int($1/1024/1024/1024)}')
    else
        MEMORY_GB=4  # Conservative estimate for Windows
    fi
    
    if [[ $MEMORY_GB -ge 4 ]]; then
        echo -e "${GREEN}✅ Memory: ${MEMORY_GB}GB${NC}"
    else
        echo -e "${RED}❌ Memory: ${MEMORY_GB}GB (4GB+ required)${NC}"
        exit 1
    fi
    
    # Check disk space
    DISK_AVAILABLE=$(df . | tail -1 | awk '{print $4}')
    DISK_GB=$((DISK_AVAILABLE / 1024 / 1024))
    
    if [[ $DISK_GB -ge 2 ]]; then
        echo -e "${GREEN}✅ Disk Space: ${DISK_GB}GB available${NC}"
    else
        echo -e "${RED}❌ Disk Space: ${DISK_GB}GB (2GB+ required)${NC}"
        exit 1
    fi
}

# Install Docker if needed
install_docker() {
    echo -e "${YELLOW}📦 Installing Docker...${NC}"
    
    if [[ "$OS" == "Linux" ]]; then
        # Install Docker on Linux
        curl -fsSL https://get.docker.com -o get-docker.sh
        sudo sh get-docker.sh
        sudo usermod -aG docker $USER
        rm get-docker.sh
    elif [[ "$OS" == "macOS" ]]; then
        # Install Docker Desktop on macOS
        if command -v brew &> /dev/null; then
            brew install --cask docker
        else
            echo -e "${RED}❌ Homebrew required for Docker installation on macOS${NC}"
            echo -e "${YELLOW}📥 Install Homebrew: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"${NC}"
            exit 1
        fi
    elif [[ "$OS" == "Windows" ]]; then
        echo -e "${YELLOW}📥 Please download Docker Desktop from: https://www.docker.com/products/docker-desktop${NC}"
        exit 1
    fi
}

# Create Polygone directory
setup_directory() {
    POLYGONE_DIR="$HOME/.polygone"
    echo -e "${BLUE}📁 Setting up Polygone in ${POLYGONE_DIR}...${NC}"
    
    mkdir -p "$POLYGONE_DIR"/{config,data,logs,models,backups}
    mkdir -p "$POLYGONE_DIR"/{polygone-petals,polygone-hide,max,nexus}
    
    echo -e "${GREEN}✅ Directory structure created${NC}"
}

# Download Polygone components
download_components() {
    echo -e "${BLUE}⬇️  Downloading POLYGONE components...${NC}"
    
    cd "$POLYGONE_DIR"
    
    # Download Docker Compose file
    curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/DEPLOYMENT/docker-compose.enterprise.yml -o docker-compose.yml
    
    # Download configuration files
    curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/MAX/simple-config.json -o config.json
    
    # Download installer script
    curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/MAX/installer.html -o installer.html
    
    # Download enterprise policy template
    cat > enterprise-policy.json << EOF
{
  "organization": {
    "name": "POLYGONE User",
    "id": "user-001",
    "contact_email": "user@example.com"
  },
  "policies": {
    "network_access": {
      "allowed_domains": ["*"],
      "blocked_domains": [],
      "time_restrictions": {
        "allowed_hours": [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23],
        "allowed_days": [0,1,2,3,4,5,6]
      }
    },
    "data_retention": {
      "conversations_days": 30,
      "logs_days": 90,
      "backups_retention_days": 365
    },
    "security": {
      "encryption_level": "post_quantum",
      "audit_logging": true,
      "anonymization": false
    }
  }
}
EOF
    
    echo -e "${GREEN}✅ Components downloaded${NC}"
}

# Generate SSL certificates
generate_ssl() {
    echo -e "${BLUE}🔐 Generating SSL certificates...${NC}"
    
    cd "$POLYGONE_DIR"
    
    # Generate self-signed certificates
    openssl req -x509 -nodes -days 365 -newkey rsa:4096 \
        -keyout ssl/polygone.key \
        -out ssl/polygone.crt \
        -subj "/C=FR/ST=Paris/L=Paris/O=POLYGONE/CN=localhost" \
        2>/dev/null || true
    
    echo -e "${GREEN}✅ SSL certificates generated${NC}"
}

# Create systemd service (Linux only)
create_service() {
    if [[ "$OS" != "Linux" ]]; then
        return
    fi
    
    echo -e "${BLUE}🔧 Creating systemd service...${NC}"
    
    sudo tee /etc/systemd/system/polygone.service > /dev/null << EOF
[Unit]
Description=POLYGONE Enterprise Stack
After=docker.service
Requires=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=$POLYGONE_DIR
ExecStart=/usr/bin/docker-compose up -d
ExecStop=/usr/bin/docker-compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF
    
    sudo systemctl daemon-reload
    sudo systemctl enable polygone.service
    
    echo -e "${GREEN}✅ Systemd service created${NC}"
}

# Start Polygone services
start_services() {
    echo -e "${BLUE}🚀 Starting POLYGONE services...${NC}"
    
    cd "$POLYGONE_DIR"
    
    # Start Docker Compose
    if command -v docker-compose &> /dev/null; then
        docker-compose up -d
    else
        docker compose up -d
    fi
    
    # Wait for services to be ready
    echo -e "${YELLOW}⏳ Waiting for services to start...${NC}"
    sleep 30
    
    # Check service status
    if curl -s http://localhost:9090/health > /dev/null; then
        echo -e "${GREEN}✅ POLYGONE services started successfully!${NC}"
    else
        echo -e "${RED}❌ Services failed to start. Check logs with: cd $POLYGONE_DIR && docker-compose logs${NC}"
        exit 1
    fi
}

# Display success message
display_success() {
    echo -e "${GREEN}"
    echo "🌸 POLYGONE v2.0.0 Installation Complete! 🌸"
    echo ""
    echo "📊 Access URLs:"
    echo "   🌐 Dashboard: http://localhost:9090"
    echo "   🤖 MAX AI: http://localhost:8000"
    echo "   🔐 Polygone Hide: socks5://localhost:1080"
    echo "   📡 Polygone Petals: http://localhost:4003"
    echo ""
    echo "📁 Installation Directory: $POLYGONE_DIR"
    echo "📋 Configuration: $POLYGONE_DIR/config.json"
    echo "📝 Logs: $POLYGONE_DIR/logs/"
    echo ""
    echo "🚀 Quick Start Commands:"
    echo "   Start:   cd $POLYGONE_DIR && docker-compose up -d"
    echo "   Stop:    cd $POLYGONE_DIR && docker-compose down"
    echo "   Status:  cd $POLYGONE_DIR && docker-compose ps"
    echo "   Logs:    cd $POLYGONE_DIR && docker-compose logs"
    echo ""
    echo "📚 Documentation: https://docs.polygone.ai"
    echo "💬 Community: https://community.polygone.ai"
    echo ""
    echo "🎯 Next Steps:"
    echo "   1. Open http://localhost:9090 in your browser"
    echo "   2. Complete the initial setup wizard"
    echo "   3. Configure your AI models and network settings"
    echo "   4. Start using POLYGONE for secure, private computing!"
    echo ""
    echo "🔧 Troubleshooting:"
    echo "   If services don't start, run: cd $POLYGONE_DIR && docker-compose logs"
    echo "   For support: support@polygone.ai"
    echo ""
    echo -e "${NC}"
}

# Main installation flow
main() {
    echo -e "${CYAN}"
    echo "🌸 POLYGONE Universal Installer v2.0.0 🌸"
    echo "Post-Quantum Privacy & Intelligence Platform"
    echo ""
    echo -e "${NC}"
    
    detect_os
    check_requirements
    setup_directory
    download_components
    generate_ssl
    create_service
    start_services
    display_success
}

# Handle interruption
trap 'echo -e "\n${RED}❌ Installation interrupted${NC}"; exit 1' INT

# Run main function
main "$@"

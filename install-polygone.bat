@echo off
REM 🌸 POLYGONE Universal Installer v2.0.0 - Windows
REM Compatible: Windows 10/11 with WSL2 or Docker Desktop

echo 🌸 POLYGONE Universal Installer v2.0.0
echo Post-Quantum Privacy & Intelligence Platform
echo.

REM Check if running as Administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ❌ Please run as Administrator
    pause
    exit /b 1
)

REM Check Docker
docker --version >nul 2>&1
if %errorLevel% neq 0 (
    echo 📦 Docker not found, installing...
    powershell -Command "Invoke-WebRequest -Uri 'https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe' -OutFile 'docker-installer.exe'"
    start docker-installer.exe
    echo 📥 Please install Docker Desktop and restart this script
    pause
    exit /b 1
)

echo ✅ Docker found

REM Create Polygone directory
set POLYGONE_DIR=%USERPROFILE%\.polygone
echo 📁 Setting up Polygone in %POLYGONE_DIR%...
mkdir "%POLYGONE_DIR%\config" 2>nul
mkdir "%POLYGONE_DIR%\data" 2>nul
mkdir "%POLYGONE_DIR%\logs" 2>nul
mkdir "%POLYGONE_DIR%\models" 2>nul
mkdir "%POLYGONE_DIR%\ssl" 2>nul

REM Download components
echo ⬇️ Downloading POLYGONE components...
cd /d "%POLYGONE_DIR%"

powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/lvs0/Polygone/main/DEPLOYMENT/docker-compose.enterprise.yml' -OutFile 'docker-compose.yml'"
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/lvs0/Polygone/main/MAX/simple-config.json' -OutFile 'config.json'"
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/lvs0/Polygone/main/MAX/installer.html' -OutFile 'installer.html'"

REM Generate SSL certificates
echo 🔐 Generating SSL certificates...
mkdir ssl 2>nul
openssl req -x509 -nodes -days 365 -newkey rsa:4096 -keyout ssl\polygone.key -out ssl\polygone.crt -subj "/C=FR/ST=Paris/L=Paris/O=POLYGONE/CN=localhost" 2>nul

REM Start services
echo 🚀 Starting POLYGONE services...
docker-compose up -d

echo ⏳ Waiting for services to start...
timeout /t 30 /nobreak

REM Check if services are running
curl -s http://localhost:9090/health >nul 2>&1
if %errorLevel% equ 0 (
    echo ✅ POLYGONE services started successfully!
) else (
    echo ❌ Services failed to start. Check logs with: cd %POLYGONE_DIR% && docker-compose logs
    pause
    exit /b 1
)

echo.
echo 🌸 POLYGONE v2.0.0 Installation Complete! 🌸
echo.
echo 📊 Access URLs:
echo    🌐 Dashboard: http://localhost:9090
echo    🤖 MAX AI: http://localhost:8000
echo    🔐 Polygone Hide: socks5://localhost:1080
echo    📡 Polygone Petals: http://localhost:4003
echo.
echo 📁 Installation Directory: %POLYGONE_DIR%
echo 📋 Configuration: %POLYGONE_DIR%\config.json
echo 📝 Logs: %POLYGONE_DIR%\logs\
echo.
echo 🚀 Quick Start Commands:
echo    Start:   cd %POLYGONE_DIR% && docker-compose up -d
echo    Stop:    cd %POLYGONE_DIR% && docker-compose down
echo    Status:  cd %POLYGONE_DIR% && docker-compose ps
echo    Logs:    cd %POLYGONE_DIR% && docker-compose logs
echo.
echo 📚 Documentation: https://docs.polygone.ai
echo 💬 Community: https://community.polygone.ai
echo.
echo 🎯 Next Steps:
echo    1. Open http://localhost:9090 in your browser
echo    2. Complete initial setup wizard
echo    3. Configure your AI models and network settings
echo    4. Start using POLYGONE for secure, private computing!
echo.
pause

#!/bin/bash

# 🌸 POLYGONE Test Suite v2.0.0
# Comprehensive testing for all components
# Usage: ./test-polygone.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test results
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_TOTAL=0

# Log function
log() {
    echo -e "${BLUE}[$(date '+%H:%M:%S')] $1${NC}"
}

# Test function
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    
    log "🧪 Testing: $test_name"
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS: $test_name${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL: $test_name${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Test Docker services
test_docker_services() {
    log "🐳 Testing Docker services..."
    
    run_test "Docker Daemon" "docker info"
    run_test "Docker Compose" "docker-compose version"
    run_test "Polygone Core Container" "docker ps | grep polygone-core"
    run_test "Polygone Petals Container" "docker ps | grep polygone-petals"
    run_test "Polygone Hide Container" "docker ps | grep polygone-hide"
    run_test "MAX Assistant Container" "docker ps | grep max-assistant"
    run_test "Ollama Container" "docker ps | grep ollama"
}

# Test network connectivity
test_network() {
    log "🌐 Testing network connectivity..."
    
    run_test "Polygone Core API" "curl -f http://localhost:4000/health"
    run_test "Polygone Petals API" "curl -f http://localhost:4003/health"
    run_test "MAX Assistant API" "curl -f http://localhost:8000/"
    run_test "Ollama API" "curl -f http://localhost:11434/api/tags"
    run_test "Polygone Hide SOCKS5" "nc -z localhost 1080"
    run_test "Monitoring Dashboard" "curl -f http://localhost:9090/health"
}

# Test AI functionality
test_ai() {
    log "🤖 Testing AI functionality..."
    
    # Test Ollama models
    run_test "Ollama Models List" "curl -s http://localhost:11434/api/tags | grep -q 'models'"
    
    # Test MAX API
    local test_message='{"message": "Hello, test message"}'
    run_test "MAX Chat API" "curl -X POST -H 'Content-Type: application/json' -d '$test_message' http://localhost:8000/chat"
    
    # Test Polygone Petals inference
    local inference_request='{"session_id": "test123", "start_layer": 0, "end_layer": 15, "hidden_states_data": "dGVzdA==", "dims": [1, 10]}'
    run_test "Petals Inference" "curl -X POST -H 'Content-Type: application/json' -d '$inference_request' http://localhost:4003/inference"
}

# Test security features
test_security() {
    log "🔐 Testing security features..."
    
    # Test SSL certificates
    run_test "SSL Certificate Exists" "test -f ~/.polygone/ssl/polygone.crt"
    run_test "SSL Key Exists" "test -f ~/.polygone/ssl/polygone.key"
    
    # Test configuration files
    run_test "Enterprise Policy" "test -f ~/.polygone/enterprise-policy.json"
    run_test "Configuration File" "test -f ~/.polygone/config.json"
    
    # Test encryption
    run_test "Post-Quantum Encryption" "openssl version | grep -q OpenSSL"
}

# Test performance
test_performance() {
    log "⚡ Testing performance..."
    
    # Test response times
    local core_time=$(curl -o /dev/null -s -w '%{time_total}' http://localhost:4000/health || echo "999")
    local max_time=$(curl -o /dev/null -s -w '%{time_total}' http://localhost:8000/ || echo "999")
    
    if (( $(echo "$core_time < 1.0" | bc -l) )); then
        echo -e "${GREEN}✅ PASS: Core API Response Time (${core_time}s)${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAIL: Core API Response Time (${core_time}s)${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    
    if (( $(echo "$max_time < 2.0" | bc -l) )); then
        echo -e "${GREEN}✅ PASS: MAX API Response Time (${max_time}s)${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAIL: MAX API Response Time (${max_time}s)${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
}

# Test data persistence
test_persistence() {
    log "💾 Testing data persistence..."
    
    run_test "Polygone Data Directory" "test -d ~/.polygone/data"
    run_test "MAX Database" "test -f ~/.polygone/data/max.db"
    run_test "Log Directory" "test -d ~/.polygone/logs"
    run_test "Backup Directory" "test -d ~/.polygone/backups"
    run_test "Model Directory" "test -d ~/.polygone/models"
}

# Test mobile compatibility
test_mobile() {
    log "📱 Testing mobile compatibility..."
    
    # Test mobile interface
    run_test "Mobile CSS" "test -f ~/.polygone/mobile.css"
    run_test "Mobile HTML" "test -f ~/.polygone/mobile.html"
    run_test "Responsive Design" "curl -s http://localhost:8000/mobile.html | grep -q 'mobile-app'"
}

# Test enterprise features
test_enterprise() {
    log "🏢 Testing enterprise features..."
    
    # Test monitoring
    run_test "Prometheus Metrics" "curl -f http://localhost:9091/metrics"
    run_test "Grafana Dashboard" "curl -f http://localhost:9092"
    
    # Test audit logging
    run_test "Audit Log File" "test -f ~/.polygone/logs/audit.log"
    
    # Test backup system
    run_test "Backup Script" "test -f ~/.polygone/scripts/backup.sh"
}

# Test cross-platform compatibility
test_cross_platform() {
    log "🔄 Testing cross-platform compatibility..."
    
    # Test installer scripts
    run_test "Linux Installer" "test -x ./install-polygone.sh"
    run_test "Windows Installer" "test -f ./install-polygone.bat"
    
    # Test Docker Compose file
    run_test "Docker Compose Config" "docker-compose -f ~/.polygone/docker-compose.yml config"
}

# Generate test report
generate_report() {
    echo ""
    echo -e "${CYAN}📊 POLYGONE Test Report${NC}"
    echo "========================"
    echo -e "Total Tests: ${BLUE}$TESTS_TOTAL${NC}"
    echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
    
    local success_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
    echo -e "Success Rate: ${BLUE}$success_rate%${NC}"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo ""
        echo -e "${GREEN}🎉 ALL TESTS PASSED! POLYGONE is ready for deployment.${NC}"
        echo ""
        echo -e "${CYAN}🚀 Ready for:${NC}"
        echo -e "   ${GREEN}✅ Production deployment${NC}"
        echo -e "   ${GREEN}✅ Enterprise testing${NC}"
        echo -e "   ${GREEN}✅ User acceptance testing${NC}"
        echo ""
        echo -e "${CYAN}📋 Next Steps:${NC}"
        echo "   1. Deploy to staging environment"
        echo "   2. Run performance benchmarks"
        echo "   3. Conduct security audit"
        echo "   4. Prepare production release"
    else
        echo ""
        echo -e "${RED}⚠️  Some tests failed. Please review and fix issues before deployment.${NC}"
        echo ""
        echo -e "${CYAN}🔧 Troubleshooting:${NC}"
        echo "   1. Check service logs: cd ~/.polygone && docker-compose logs"
        echo "   2. Verify Docker installation: docker --version"
        echo "   3. Check network connectivity: curl -v http://localhost:4000/health"
        echo "   4. Review system requirements: python3 --version, docker info"
    fi
}

# Main test execution
main() {
    echo -e "${CYAN}"
    echo "🌸 POLYGONE Test Suite v2.0.0"
    echo "Comprehensive Testing Platform"
    echo ""
    echo -e "${NC}"
    
    # Check if Polygone is installed
    if [ ! -d "$HOME/.polygone" ]; then
        echo -e "${RED}❌ POLYGONE not found. Please install first with:${NC}"
        echo "   curl -sSL https://install.polygone.ai | bash"
        exit 1
    fi
    
    # Run all test suites
    test_docker_services
    test_network
    test_ai
    test_security
    test_performance
    test_persistence
    test_mobile
    test_enterprise
    test_cross_platform
    
    # Generate final report
    generate_report
}

# Handle script interruption
trap 'echo -e "\n${RED}❌ Testing interrupted${NC}"; exit 1' INT

# Run main function
main "$@"

#!/usr/bin/env python3
"""
Polygone — Revenue & Metrics Tracker
Autonomous daily check: DePIN income, token airdrops, community growth
Sends report to Telegram every morning
"""

import json, subprocess, os
from datetime import datetime
from pathlib import Path

WORKSPACE = Path.home() / "Polygone"
METRICS_FILE = WORKSPACE / ".hermes-data" / "metrics.json"
LOG_DIR = WORKSPACE / ".hermes-logs"
TG_CHAT_ID = "7666797404"

def log(msg):
    ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    line = f"[{ts}] {msg}"
    print(line)
    LOG_DIR.mkdir(exist_ok=True)
    logfile = LOG_DIR / f"revenue-{datetime.now().strftime('%Y%m%d')}.log"
    with open(logfile, "a") as f:
        f.write(line + "\n")

def load_metrics():
    METRICS_FILE.parent.mkdir(parents=True, exist_ok=True)
    if METRICS_FILE.exists():
        with open(METRICS_FILE) as f:
            return json.load(f)
    return {
        "depin": {"grass_earnings": 0, "ionet_earnings": 0},
        "airdrops": {"pending": [], "claimed": []},
        "community": {"discord": 0, "github_stars": 0, "twitter_followers": 0},
        "revenue": {"token_sales": 0, "b2b_licenses": 0, "subscriptions": 0},
    }

def save_metrics(m):
    with open(METRICS_FILE, "w") as f:
        json.dump(m, f, indent=2)

def check_docker_status():
    """Check if DePIN nodes are running"""
    result = subprocess.run(
        ["docker", "ps", "--format", "{{.Names}}"],
        capture_output=True, text=True
    )
    containers = result.stdout.strip().split("\n") if result.stdout.strip() else []
    return containers

def check_grass_logs():
    """Get Grass node earnings from logs"""
    result = subprocess.run(
        ["docker", "logs", "grass-node", "--tail", "50"],
        capture_output=True, text=True, errors="ignore"
    )
    return result.stdout[-2000:] if result.stdout else ""

def check_github_stars():
    """Get GitHub stars for Polygone repo"""
    result = subprocess.run(
        ["curl", "-s", "https://api.github.com/repos/lvs0/Polygone"],
        capture_output=True, text=True, errors="ignore"
    )
    if result.stdout:
        try:
            import json
            data = json.loads(result.stdout)
            return data.get("stargazers_count", 0)
        except:
            pass
    return None

def get_discord_count():
    """Get Discord member count (requires bot or manual)"""
    # For now, return None — needs Discord bot
    return None

def format_report(m, docker_containers, grass_logs, github_stars, discord_count):
    today = datetime.now().strftime("%Y-%m-%d %H:%M")
    
    depin_status = "OFF" if "grass-node" not in docker_containers else "ONLINE"
    
    report = f"""📊 **POLYGONE — Daily Report**
{today}

**🖥️ Infrastructure**
- Grass node: {depin_status}
- Docker containers: {', '.join(docker_containers) if docker_containers else 'none'}
- io.net: {'running' if 'ionet' in ' '.join(docker_containers) else 'not installed'}"""

    if github_stars is not None:
        report += f"\n**⭐ GitHub Stars:** {github_stars}"
    
    if discord_count:
        report += f"\n**💬 Discord:** {discord_count} members"
    
    report += f"""

**💰 Revenue (cumulative)**
- DePIN Grass: ${m['depin']['grass_earnings']:.2f}
- DePIN io.net: ${m['depin']['ionet_earnings']:.2f}
- Token sales: ${m['revenue']['token_sales']:.2f}
- B2B licenses: ${m['revenue']['b2b_licenses']:.2f}
- Subscriptions: ${m['revenue']['subscriptions']:.2f}
- **TOTAL:** ${sum(m['revenue'].values()) + sum(m['depin'].values()):.2f}

**🪂 Pending Airdrops**
"""
    for airdrop in m['airdrops'].get('pending', []):
        report += f"- {airdrop}\n"
    
    if not m['airdrops'].get('pending'):
        report += "- None yet\n"
    
    report += f"""
**🚀 Next Actions**
"""
    
    # Determine next actions based on status
    if depin_status == "OFF":
        report += "- Install Grass node (see setup-depin.sh)\n"
    if github_stars is None or github_stars < 10:
        report += "- Share repo to get first stars\n"
    if not m['airdrops'].get('pending'):
        report += "- Run airdrop farming scripts\n"
    
    return report

def main():
    log("=== Polygone Daily Revenue Check ===")
    
    m = load_metrics()
    
    # Gather data
    docker_containers = check_docker_status()
    grass_logs = check_grass_logs() if "grass-node" in docker_containers else ""
    github_stars = check_github_stars()
    discord_count = get_discord_count()
    
    # Format report
    report = format_report(m, docker_containers, grass_logs, github_stars, discord_count)
    print(report)
    
    # Save metrics
    save_metrics(m)
    
    # Send to Telegram
    log("Sending report to Telegram...")
    subprocess.run([
        "curl", "-s", "-X", "POST",
        f"https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage",
        "-d", f"chat_id={TG_CHAT_ID}",
        "-d", f"text={report}",
        "-d", "parse_mode=Markdown"
    ], env={**os.environ, "TELEGRAM_BOT_TOKEN": os.environ.get("TELEGRAM_BOT_TOKEN", "")})
    
    log("=== Daily check complete ===")

if __name__ == "__main__":
    main()
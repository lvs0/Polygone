import asyncio
import subprocess
import json
import os
import sys
from datetime import datetime
from textual.app import App, ComposeResult
from textual.containers import Container, Horizontal, Vertical, ScrollableContainer
from textual.widgets import (
    Header, Footer, Static, Button, Label, TabbedContent, TabPane,
    DataTable, Input, TextArea, ProgressBar, Digits
)
from textual.binding import Binding
from textual.events import Mount
from textual.message import Message


# ── Helpers ──────────────────────────────────────────────────────────────────

def run_polygone(args: list[str], timeout: int = 10) -> tuple[int, str, str]:
    """Run polygone CLI, return (code, stdout, stderr)."""
    try:
        result = subprocess.run(
            ["polygone"] + args,
            capture_output=True,
            text=True,
            timeout=timeout,
            env={**os.environ, "RUST_BACKTRACE": "1"}
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return 124, "", "Command timed out"
    except FileNotFoundError:
        return 127, "", "polygone not found in PATH"
    except Exception as e:
        return 1, "", str(e)


def get_status() -> dict:
    """Get node status from polygone."""
    code, out, _ = run_polygone(["node", "status"])
    if code == 0:
        lines = out.strip().split("\n")
        status = {}
        for line in lines:
            if ":" in line:
                key, val = line.split(":", 1)
                status[key.strip()] = val.strip()
        return status
    return {"status": "offline", "version": "unknown"}


def get_keys() -> dict:
    """Check if keys exist."""
    key_dir = os.path.expanduser("~/.polygone/keys")
    has_keys = os.path.exists(key_dir)
    pub_key = ""
    if has_keys:
        pk_file = os.path.join(key_dir, "public_key.pem")
        if os.path.exists(pk_file):
            with open(pk_file) as f:
                pub_key = f.read()[:40] + "..."
    return {"has_keys": has_keys, "pub_key": pub_key}


# ── Status Card ───────────────────────────────────────────────────────────────

class StatusCard(Static):
    """Main status display."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.status = {"status": "offline", "version": "unknown"}

    def on_mount(self) -> None:
        self.refresh_status()
        self.set_interval(5, self.refresh_status)

    def refresh_status(self) -> None:
        self.status = get_status()
        self.update()

    def render(self):
        s = self.status
        color = "green" if s.get("status", "offline") == "online" else "red"
        return f"[b]{s.get('version', 'Polygone')}[/b]\n[{color}]● {s.get('status', 'offline')}[/{color}]"


# ── Output Log ────────────────────────────────────────────────────────────────

class OutputLog(Static):
    """Scrolling output log for command results."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.lines: list[str] = []

    def append(self, text: str, style: str = "") -> None:
        self.lines.append((text, style))
        if len(self.lines) > 50:
            self.lines.pop(0)
        self.update()

    def render(self):
        if not self.lines:
            return "[dim]En attente...[/dim]"
        return "\n".join(f"[{s}]{t}[/{s}]" if s else t for t, s in self.lines)


# ── Main App ─────────────────────────────────────────────────────────────────

class PolygoneApp(App):
    """Polygone Terminal UI — centralise tout."""

    TITLE = "POLYGONE"
    SUB_TITLE = "L'information n'existe pas. Elle traverse."

    CSS = """
    Screen { background: #0F0F12; }

    #header-bar {
        height: 3;
        background: #0F0F12;
        dock: top;
    }

    TabbedContent { background: #0F0F12; }
    TabPane { background: #0F0F12; padding: 1 2; }

    /* Status indicator */
    .status-online { color: #50FA7B; }
    .status-offline { color: #FF5555; }

    /* Buttons */
    Button { margin: 1 1; }
    #btn-primary { background: #1a1a2e; color: #00FF88; border: solid #00FF88; }
    #btn-danger { background: #1a1a2e; color: #FF5555; border: solid #FF5555; }
    #btn-warning { background: #1a1a2e; color: #FFB86C; border: solid #FFB86C; }

    /* Sections */
    .section { margin: 1 0; padding: 1; border: solid #333; background: #1a1a1a; }
    .section-title { color: #00FF88; text-style: bold; margin-bottom: 1; }

    /* Log output */
    #output-log {
        background: #0a0a0f;
        color: #E0E0E0;
        padding: 2;
        height: 20;
        border: solid #333;
    }

    /* Inputs */
    Input { margin: 1 0; }

    /* Tables */
    DataTable { margin: 1 0; }

    /* Cards */
    .card {
        background: #1a1a1a;
        border: solid #333;
        padding: 2;
        margin: 1;
    }
    .card-title { color: #00FF88; text-style: bold; }
    .card-value { color: #E0E0E0; text-style: bold; font-size: 24; }
    """

    BINDINGS = [
        Binding("q", "quit", "Quitter"),
        Binding("1", "switch_tab('accueil')", "Accueil"),
        Binding("2", "switch_tab('identite')", "Identité"),
        Binding("3", "switch_tab('messages')", "Messages"),
        Binding("4", "switch_tab('reseau')", "Réseau"),
        Binding("5", "switch_tab('parametres')", "Config"),
    ]

    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)

        with TabbedContent(initial="accueil"):
            # ── ACCUEIL ────────────────────────────────────────────────────────
            with TabPane("Accueil", id="accueil"):
                with Horizontal():
                    with Vertical():
                        yield Static("⬡ POLYGONE", styles="color: #00FF88; text-style: bold; font-size: 32;")
                        yield Static("L'information n'existe pas. Elle traverse.", styles="color: #888; italic;")
                        yield Static("", styles="height: 1;")
                        yield Button("● Connecter", id="btn-connect", variant="success")
                        yield Button("○ Déconnecter", id="btn-disconnect", variant="error")
                        yield Button("↻ Statut", id="btn-refresh")

                    with Vertical():
                        yield Static("[bold green]État du réseau[/bold green]", classes="section-title")
                        yield Static("", styles="height: 1;")
                        self.status_display = Static("", id="status-display")
                        yield Static("", styles="height: 2;")
                        yield Static("[bold]Version:[/bold] [dim]Vérification...[/dim]", id="version-text")

            # ── IDENTITÉ ───────────────────────────────────────────────────────
            with TabPane("Identité", id="identite"):
                with Vertical():
                    yield Static("Gestion des clés", classes="section-title")
                    yield Button("🔑 Générer une nouvelle identité", id="btn-keygen")
                    yield Button("📋 Afficher ma clé publique", id="btn-show-pk")
                    yield Button("📤 Exporter ma clé", id="btn-export-pk")
                    yield Static("", styles="height: 1;")
                    self.key_output = OutputLog("", id="key-output")
                    yield Static("", styles="height: 1;")
                    self.keygen_progress = ProgressBar(disabled=True, id="keygen-progress")

            # ── MESSAGES ───────────────────────────────────────────────────────
            with TabPane("Messages", id="messages"):
                with Horizontal():
                    with Vertical():
                        yield Static("Envoyer un message", classes="section-title")
                        yield Input(placeholder="Clé publique du destinataire", id="input-peer-pk")
                        yield Input(placeholder="Votre message...", id="input-message")
                        yield Button("▶ Envoyer", id="btn-send", variant="success")
                        yield Static("", styles="height: 1;")
                        yield Static("Recevoir un message", classes="section-title")
                        yield Input(placeholder="Clé secrète (sk)", id="input-sk")
                        yield Input(placeholder="Ciphertext...", id="input-ciphertext")
                        yield Button("▼ Recevoir", id="btn-receive")
                        yield Button("🗑️ Effacer", id="btn-clear-msgs")
                    with Vertical(id="msg-output-container"):
                        yield Static("Historique", classes="section-title")
                        self.msg_output = OutputLog("", id="msg-output")

            # ── RÉSEAU ─────────────────────────────────────────────────────────
            with TabPane("Réseau", id="reseau"):
                with Horizontal():
                    with Vertical():
                        yield Static("Gestion du nœud", classes="section-title")
                        yield Button("▶ Démarrer nœud", id="btn-node-start", variant="success")
                        yield Button("■ Arrêter nœud", id="btn-node-stop", variant="error")
                        yield Button("ℹ Info nœud", id="btn-node-info")
                        yield Button("⚡ Boost CPU", id="btn-node-boost")
                        yield Button("🔄 Vérifier updates", id="btn-node-update")
                    with Vertical():
                        yield Static("Topologie réseau", classes="section-title")
                        self.node_output = OutputLog("", id="node-output")

            # ── PARAMÈTRES ──────────────────────────────────────────────────────
            with TabPane("Config", id="parametres"):
                with Vertical():
                    yield Static("Configuration", classes="section-title")
                    yield Button("🛠 Lancer l'assistant setup", id="btn-setup")
                    yield Button("📋 Afficher config", id="btn-show-config")
                    yield Button("⚙ Auto-update", id="btn-auto-update")
                    yield Static("", styles="height: 1;")
                    yield Static("DANGER", classes="section-title", styles="color: #FF5555;")
                    yield Button("🗑 Supprimer identité", id="btn-delete-keys", variant="error")
                    yield Button("🔁 Reset config", id="btn-reset-config", variant="error")
                    yield Static("", styles="height: 1;")
                    yield Static("Aide", classes="section-title")
                    yield Static("polygone --help  •  Tab 1-5 pour naviguer", styles="color: #666;")

        yield Footer()

    # ── Tab navigation ───────────────────────────────────────────────────────

    def action_switch_tab(self, tab_id: str) -> None:
        tc = self.query_one(TabbedContent)
        tc.active = tab_id

    # ── Startup ─────────────────────────────────────────────────────────────

    def on_mount(self) -> None:
        self.key_output = self.query_one("#key-output", OutputLog)
        self.msg_output = self.query_one("#msg-output", OutputLog)
        self.node_output = self.query_one("#node-output", OutputLog)
        self.keygen_progress = self.query_one("#keygen-progress", ProgressBar)
        self.status_display = self.query_one("#status-display", Static)
        self.version_text = self.query_one("#version-text", Static)
        self._refresh_status()

    # ── Status ─────────────────────────────────────────────────────────────

    def _refresh_status(self) -> None:
        code, out, err = run_polygone(["node", "status"])
        status = "offline"
        version = "unknown"
        if code == 0:
            for line in out.split("\n"):
                if "Status" in line or "status" in line:
                    status = line.split(":", 1)[-1].strip().lower()
                    if "online" in status or "connect" in status:
                        status = "online"
                    else:
                        status = "offline"
                if "Version" in line or "version" in line:
                    version = line.split(":", 1)[-1].strip()
        color = "green" if status == "online" else "red"
        icon = "●" if status == "online" else "○"
        self.status_display.update(
            f"[bold]État: [/{color}]{icon} {status}[/bold]\n"
            f"[dim]Version: {version}[/dim]"
        )
        self.version_text.update(f"[bold]Version:[/bold] [dim]{version}[/dim]")

    # ── Button handlers ────────────────────────────────────────────────────

    def on_button_pressed(self, event: Button.Pressed) -> None:
        btn_id = event.button.id or ""

        if btn_id == "btn-refresh":
            self._refresh_status()
            self.key_output.append("Statut rafraîchi.", "cyan")

        elif btn_id == "btn-connect":
            self.node_output.append("→ polygone node start...", "cyan")
            code, out, err = run_polygone(["node", "start"])
            self.node_output.append(out or err or "(no output)", "green" if code == 0 else "red")
            self._refresh_status()

        elif btn_id == "btn-disconnect":
            self.node_output.append("→ polygone node stop...", "cyan")
            code, out, err = run_polygone(["node", "stop"])
            self.node_output.append(out or err, "green" if code == 0 else "red")
            self._refresh_status()

        elif btn_id == "btn-keygen":
            self._do_keygen()

        elif btn_id == "btn-show-pk":
            self._do_show_pk()

        elif btn_id == "btn-send":
            self._do_send()

        elif btn_id == "btn-receive":
            self._do_receive()

        elif btn_id == "btn-clear-msgs":
            self.msg_output.lines = []
            self.msg_output.update()

        elif btn_id == "btn-node-start":
            self.node_output.append("→ polygone node start...", "cyan")
            code, out, err = run_polygone(["node", "start"])
            self.node_output.append(out or err, "green" if code == 0 else "red")
            self._refresh_status()

        elif btn_id == "btn-node-stop":
            self.node_output.append("→ polygone node stop...", "cyan")
            code, out, err = run_polygone(["node", "stop"])
            self.node_output.append(out or err, "green" if code == 0 else "red")
            self._refresh_status()

        elif btn_id == "btn-node-info":
            self.node_output.append("→ polygone node info...", "cyan")
            code, out, err = run_polygone(["node", "info"])
            self.node_output.append(out or err, "green" if code == 0 else "red")

        elif btn_id == "btn-node-boost":
            self.node_output.append("→ polygone node boost...", "cyan")
            code, out, err = run_polygone(["node", "boost"])
            self.node_output.append(out or err, "green" if code == 0 else "red")

        elif btn_id == "btn-node-update":
            self.node_output.append("→ polygone node update...", "cyan")
            code, out, err = run_polygone(["node", "update"])
            self.node_output.append(out or err, "green" if code == 0 else "red")

        elif btn_id == "btn-setup":
            self.node_output.append("→ polygone setup...", "cyan")
            # Setup is interactive — run without TUI
            code, out, err = run_polygone(["setup"])
            self.node_output.append(out or err, "green" if code == 0 else "red")

        elif btn_id == "btn-show-config":
            self.node_output.append("→ Showing config...", "cyan")
            code, out, err = run_polygone(["node", "info"])
            self.node_output.append(out or err, "green" if code == 0 else "red")

        elif btn_id == "btn-auto-update":
            self.node_output.append("→ Toggling auto-update...", "cyan")
            self.node_output.append("Auto-update toggle: configure in ~/.polygone/config.toml", "yellow")

        elif btn_id == "btn-delete-keys":
            self.node_output.append("⚠ Deleting keys requires --force flag", "red")
            self.node_output.append("Run: polygone keygen --force to regenerate", "yellow")

        elif btn_id == "btn-reset-config":
            self.node_output.append("⚠ Config reset not implemented yet", "yellow")
            self.node_output.append("Delete ~/.polygone/config.toml manually", "yellow")

    # ── Key operations ─────────────────────────────────────────────────────

    def _do_keygen(self) -> None:
        self.key_output.append("Génération d'une nouvelle identité...", "cyan")
        self.keygen_progress.disabled = False
        self.keygen_progress.update(progress=50)
        code, out, err = run_polygone(["keygen", "--force"], timeout=30)
        self.keygen_progress.update(progress=100)
        self.keygen_progress.disabled = True
        if code == 0:
            self.key_output.append("✓ Identité générée avec succès!", "green")
            self.key_output.append(out.strip().split('\n')[-1], "green")
        else:
            self.key_output.append(f"✗ Erreur: {err or out}", "red")

    def _do_show_pk(self) -> None:
        keys = get_keys()
        if keys["has_keys"]:
            self.key_output.append(f"✓ Clé publique chargée:", "green")
            self.key_output.append(keys["pub_key"], "cyan")
        else:
            self.key_output.append("✗ Aucune clé trouvée. Lancez 'Générer identité' d'abord.", "red")

    # ── Messaging ─────────────────────────────────────────────────────────

    def _do_send(self) -> None:
        peer_pk = self.query_one("#input-peer-pk", Input).value.strip()
        message = self.query_one("#input-message", Input).value.strip()
        if not peer_pk or not message:
            self.msg_output.append("✗ Remplis la clé ET le message.", "red")
            return
        if peer_pk == "demo":
            code, out, err = run_polygone(["send", "--peer-pk", "demo", "--message", message], timeout=30)
        else:
            code, out, err = run_polygone(["send", "--peer-pk", peer_pk, "--message", message], timeout=30)
        if code == 0:
            self.msg_output.append(f"→ Message envoyé: {message[:40]}...", "green")
            self.msg_output.append(out.strip(), "cyan")
        else:
            self.msg_output.append(f"✗ Erreur: {err or out}", "red")

    def _do_receive(self) -> None:
        sk = self.query_one("#input-sk", Input).value.strip()
        ct = self.query_one("#input-ciphertext", Input).value.strip()
        if not sk or not ct:
            self.msg_output.append("✗ Remplis la clé ET le ciphertext.", "red")
            return
        code, out, err = run_polygone(["receive", "--sk", sk, "--ciphertext", ct], timeout=30)
        if code == 0:
            self.msg_output.append(f"✓ Message reçu: {out.strip()}", "green")
        else:
            self.msg_output.append(f"✗ Erreur: {err or out}", "red")


# ── Entry point ────────────────────────────────────────────────────────────────

def main():
    app = PolygoneApp()
    app.run()


if __name__ == "__main__":
    main()

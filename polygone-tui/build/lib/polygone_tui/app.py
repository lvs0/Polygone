import asyncio
import random
from datetime import datetime, timedelta
from textual.app import App, ComposeResult
from textual.containers import Container, Horizontal, Vertical, Grid
from textual.widgets import Header, Footer, Static, Button, Label, TabbedContent, TabPane, DataTable
from textual.binding import Binding
from rich.text import Text
from rich.panel import Panel

class StatCard(Static):
    """A card showing a single statistic."""
    def __init__(self, title: str, value: str, subtitle: str = "", icon: str = "⬡", **kwargs):
        super().__init__(**kwargs)
        self.title = title
        self.value = value
        self.subtitle = subtitle
        self.icon = icon

    def render(self) -> Panel:
        content = Text.assemble(
            (f"{self.title}\n\n", "bold white"),
            (f"{self.value} ", "bold cyan"),
            (f"{self.icon}\n", "cyan"),
            (f"\n{self.subtitle}", "italic gray")
        )
        return Panel(content, style="on #262626", border_style="cyan")

class NodeStatus(Static):
    """Visual indicator for node status."""
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.active_since = datetime.now() - timedelta(minutes=18)

    def render(self) -> Text:
        return Text.assemble(
            ("Nœud  ", "bold white size(20)"),
            ("|  ", "white"),
            ("● ", "green"),
            ("Actifs\n", "green"),
            (f"depuis {(datetime.now() - self.active_since).seconds // 60}min", "gray")
        )

class PolygoneApp(App):
    """A Textual app for Polygone inspired by the user's visual design."""
    
    TITLE = "POLYGONE"
    SUB_TITLE = "L'information n'existe pas. Elle traverse."
    
    CSS = """
    Screen {
        background: #1a1a1a;
    }
    
    #header {
        height: 3;
        background: #1a1a1a;
        color: white;
        border-bottom: solid #333;
        content-align: center middle;
    }
    
    TabbedContent {
        background: #1a1a1a;
    }
    
    TabPane {
        padding: 1 2;
    }

    .main-container {
        layout: grid;
        grid-size: 2;
        grid-columns: 1fr 1fr;
        grid-rows: 1fr 1fr;
        grid-gutter: 2;
    }

    .sidebar {
        width: 30;
        height: 100%;
        padding: 1;
        border-right: solid #333;
    }

    .content-area {
        layout: horizontal;
        height: 100%;
    }

    .card-container {
        layout: grid;
        grid-size: 2;
        grid-columns: 1fr 1fr;
        grid-gutter: 1;
    }

    Button {
        width: 100%;
        margin: 1 0;
        background: #333;
        color: white;
        border: none;
    }

    Button:hover {
        background: #444;
    }

    #btn-disable { color: #ff5555; }
    
    .status-active {
        color: #50fa7b;
    }
    
    .section-title {
        text-style: bold;
        margin-bottom: 1;
        color: #888;
    }
    """

    BINDINGS = [
        Binding("q", "quit", "Quitter"),
        Binding("1", "switch_tab('accueil')", "Accueil"),
        Binding("2", "switch_tab('favoris')", "Favoris"),
        Binding("3", "switch_tab('services')", "Services"),
        Binding("4", "switch_tab('parametres')", "Paramètres"),
    ]

    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)
        with TabbedContent(initial="accueil"):
            with TabPane("Accueil", id="accueil"):
                with Horizontal():
                    with Vertical(classes="sidebar"):
                        yield NodeStatus()
                        yield Label("\nOptions", classes="section-title")
                        yield Static(style="height: 1; border-top: solid gray;")
                        yield Button("Désactiver", id="btn-disable")
                        yield Button("Mettre à jour")
                        yield Button("Redémarrer")
                        yield Button("Mettre en pause")
                    
                    with Vertical(style="padding: 2;"):
                        with Horizontal(style="height: 15;"):
                            yield StatCard(
                                "Solde :", 
                                "10", 
                                "18h de puissance à volonté",
                                id="balance-card"
                            )
                            yield StatCard(
                                "Consommé actuellement :", 
                                "0.1 / Min", 
                                "",
                                id="usage-card",
                                style="margin-left: 2;"
                            )
                        
                        with Vertical(style="margin-top: 2; border: solid #444; padding: 1;"):
                            yield Label("Services", classes="section-title")
                            yield Static("Aucun service actif pour le moment.", style="color: #666; italic;")

            with TabPane("Favoris", id="favoris"):
                yield Label("Vos contacts et nœuds favoris apparaîtront ici.")
            
            with TabPane("Services", id="services"):
                yield Label("Exploration des services décentralisés Polygone.")
                
            with TabPane("Paramètres", id="parametres"):
                yield Label("Configuration du nœud et de la sécurité.")
        
        yield Footer()

    def action_switch_tab(self, tab_id: str) -> None:
        self.query_one(TabbedContent).active = tab_id

if __name__ == "__main__":
    app = PolygoneApp()
    app.run()

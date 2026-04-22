//! POLYGONE GUI — Interface graphique desktop
//! 
//! Une interface moderne et intuitive pour POLYGONE.
//! Construite avec Iced (Rust GUI toolkit).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    application, button, Alignment, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text, TextInput, Theme, Color,
};

fn main() -> iced::Result {
    PolygoneGUI::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(900.0, 650.0),
            min_size: Some(iced::Size::new(600.0, 450.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct PolygoneGUI {
    active_tab: Tab,
    message_input: String,
    peer_key_input: String,
    status_message: String,
    keygen_button: button::State,
    send_button: button::State,
    receive_button: button::State,
    tabs: [button::State; 4],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Dashboard,
    Keygen,
    Send,
    Receive,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(Tab),
    KeygenPressed,
    SendPressed,
    ReceivePressed,
    MessageChanged(String),
    PeerKeyChanged(String),
    StatusUpdated(String),
}

impl Sandbox for PolygoneGUI {
    type Message = Message;
    type Theme = Theme;

    fn new() -> Self {
        PolygoneGUI {
            active_tab: Tab::Dashboard,
            message_input: String::new(),
            peer_key_input: String::new(),
            status_message: String::from("⬡ POLYGONE v2.0 — Prêt"),
            keygen_button: button::State::new(),
            send_button: button::State::new(),
            receive_button: button::State::new(),
            tabs: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("⬡ POLYGONE — Messagerie Post-Quantique")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                self.status_message = format!("➤ Onglet: {:?}", tab);
            }
            Message::KeygenPressed => {
                self.status_message = String::from("🔑 Génération des clés en cours...");
                // Ici: appel à polygone::keys::generate()
            }
            Message::SendPressed => {
                if !self.message_input.is_empty() && !self.peer_key_input.is_empty() {
                    self.status_message = String::from("📤 Envoi du message chiffré...");
                    // Ici: appel à polygone::send()
                } else {
                    self.status_message = String::from("⚠️ Remplissez le message et la clé du destinataire");
                }
            }
            Message::ReceivePressed => {
                self.status_message = String::from("📥 Réception et déchiffrement...");
                // Ici: appel à polygone::receive()
            }
            Message::MessageChanged(text) => {
                self.message_input = text;
            }
            Message::PeerKeyChanged(text) => {
                self.peer_key_input = text;
            }
            Message::StatusUpdated(status) => {
                self.status_message = status;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = Container::new(Text::new("⬡ POLYGONE").size(32))
            .padding(20)
            .style(ContainerStyle::Header);

        let tabs = Row::new()
            .spacing(10)
            .push(self.tab_button(Tab::Dashboard, "Tableau de bord", 0))
            .push(self.tab_button(Tab::Keygen, "Clés", 1))
            .push(self.tab_button(Tab::Send, "Envoyer", 2))
            .push(self.tab_button(Tab::Receive, "Recevoir", 3));

        let content = match self.active_tab {
            Tab::Dashboard => self.dashboard_view(),
            Tab::Keygen => self.keygen_view(),
            Tab::Send => self.send_view(),
            Tab::Receive => self.receive_view(),
        };

        let status_bar = Container::new(Text::new(&self.status_message).size(12))
            .padding(10)
            .style(ContainerStyle::StatusBar);

        let column = Column::new()
            .align_items(Alignment::Center)
            .spacing(1)
            .push(header)
            .push(tabs)
            .push(content)
            .push(status_bar);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl PolygoneGUI {
    fn tab_button(&self, tab: Tab, label: &str, index: usize) -> Element<Message> {
        let is_active = self.active_tab == tab;
        let style = if is_active { ButtonStyle::Active } else { ButtonStyle::Inactive };

        Button::new(
            &mut self.tabs[index],
            Text::new(label).size(14),
        )
        .on_press(Message::TabSelected(tab))
        .style(style)
        .padding(10)
        .into()
    }

    fn dashboard_view(&self) -> Element<Message> {
        let info = Column::new()
            .spacing(15)
            .push(Text::new("Bienvenue sur POLYGONE").size(24))
            .push(Text::new("Messagerie privée post-quantique").size(16))
            .push(Text::new("\"L'information n'existe pas. Elle traverse.\"").size(12))
            .push(Text::new(""))
            .push(Text::new("Fonctionnalités:").size(18))
            .push(Text::new("  • Cryptographie ML-KEM-1024 (post-quantique)"))
            .push(Text::new("  • Messages éphémères auto-détruits"))
            .push(Text::new("  • Architecture \"vague\" inobservable"))
            .push(Text::new("  • Zeroize mémoire automatique"));

        Container::new(info)
            .padding(30)
            .center_x()
            .into()
    }

    fn keygen_view(&self) -> Element<Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("🔑 Génération de Clés").size(24))
            .push(Text::new("Créez votre paire de clés post-quantique").size(14))
            .push(Button::new(
                &self.keygen_button,
                Text::new("Générer mes clés").size(16),
            )
            .on_press(Message::KeygenPressed)
            .style(ButtonStyle::Primary)
            .padding(15))
            .push(Text::new("Vos clés seront sauvegardées dans ~/.polygone/keys").size(11));

        Container::new(content)
            .padding(30)
            .center_x()
            .into()
    }

    fn send_view(&self) -> Element<Message> {
        let content = Column::new()
            .spacing(20)
            .push(Text::new("📤 Envoyer un Message").size(24))
            .push(TextInput::new(
                "Clé publique du destinataire (hex ou QR code)",
                &self.peer_key_input,
                Message::PeerKeyChanged,
            )
            .padding(10)
            .size(16))
            .push(TextInput::new(
                "Votre message secret...",
                &self.message_input,
                Message::MessageChanged,
            )
            .padding(10)
            .size(16)
            .width(Length::Fill))
            .push(Button::new(
                &self.send_button,
                Text::new("Envoyer de manière sécurisée").size(16),
            )
            .on_press(Message::SendPressed)
            .style(ButtonStyle::Primary)
            .padding(15));

        Container::new(content)
            .padding(30)
            .width(Length::Fill)
            .into()
    }

    fn receive_view(&self) -> Element<Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("📥 Recevoir un Message").size(24))
            .push(Text::new("Entrez les fragments reçus").size(14))
            .push(Button::new(
                &self.receive_button,
                Text::new("Recevoir et déchiffrer").size(16),
            )
            .on_press(Message::ReceivePressed)
            .style(ButtonStyle::Primary)
            .padding(15));

        Container::new(content)
            .padding(30)
            .center_x()
            .into()
    }
}

#[derive(Default)]
enum ButtonStyle {
    #[default]
    Inactive,
    Active,
    Primary,
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        match self {
            ButtonStyle::Inactive => button::Style {
                background: Some(Color::from_rgb(0.15, 0.15, 0.2).into()),
                border_radius: 5.0,
                text_color: Color::from_rgb(0.7, 0.7, 0.9),
                ..Default::default()
            },
            ButtonStyle::Active => button::Style {
                background: Some(Color::from_rgb(0.1, 0.4, 1.0).into()),
                border_radius: 5.0,
                text_color: Color::WHITE,
                ..Default::default()
            },
            ButtonStyle::Primary => button::Style {
                background: Some(Color::from_rgb(0.1, 0.42, 1.0).into()),
                border_radius: 8.0,
                text_color: Color::WHITE,
                ..Default::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();
        button::Style {
            background: Some(Color::from_rgb(0.2, 0.5, 1.0).into()),
            ..active
        }
    }
}

enum ContainerStyle {
    Header,
    StatusBar,
}

impl ContainerStyle {
    fn base() -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb(0.8, 0.8, 0.95)),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        match self {
            ContainerStyle::Header => container::Style {
                background: Some(Color::from_rgb(0.05, 0.05, 0.1).into()),
                ..ContainerStyle::base()
            },
            ContainerStyle::StatusBar => container::Style {
                background: Some(Color::from_rgb(0.08, 0.08, 0.12).into()),
                ..ContainerStyle::base()
            },
        }
    }
}

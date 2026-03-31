use std::sync::Arc;
use anyhow::Result;
use tracing::{info, error};

use crate::commands::WhatsAppClient;
use crate::db::DatabaseService;
use crate::utils::format::style_text;

pub struct WelcomeHandler {
    pub db: Arc<DatabaseService>,
}

pub struct GroupEvent {
    pub group_id: String,
    pub participants: Vec<String>,
    pub event_type: GroupEventType,
}

pub enum GroupEventType {
    Join,
    Leave,
}

impl WelcomeHandler {
    pub async fn handle(
        &self,
        event: GroupEvent,
        client: Arc<dyn WhatsAppClient + Send + Sync>,
    ) {
        let group = match self.db.get_group(&event.group_id).await {
            Ok(g) => g,
            Err(e) => {
                error!("Error obteniendo grupo {}: {}", event.group_id, e);
                return;
            }
        };

        match event.event_type {
            GroupEventType::Join => {
                if !group.settings.welcome { return; }
                for participant in &event.participants {
                    let phone = crate::utils::helpers::extract_phone(participant);
                    let msg = style_text(&format!(
                        "ꕣ *¡Bienvenido/a!*\n\n\
                         @{} se unió al grupo.\n\n\
                         _Usa #help para ver los comandos disponibles._",
                        phone
                    ));
                    let _ = client.send_text(&event.group_id, &msg).await;
                }
            }
            GroupEventType::Leave => {
                if !group.settings.goodbye { return; }
                for participant in &event.participants {
                    let phone = crate::utils::helpers::extract_phone(participant);
                    let msg = style_text(&format!(
                        "👋 @{} abandonó el grupo. ¡Hasta pronto!",
                        phone
                    ));
                    let _ = client.send_text(&event.group_id, &msg).await;
                }
            }
        }
    }
}

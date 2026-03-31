use std::env;

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub owner_jid: String,
    pub prefix: String,
    pub prefixes: Vec<String>,
    pub sessions_dir: String,
    pub mongodb_uri: String,
    pub cluster_port: u16,
    pub cluster_secret: String,
    pub node_id: String,
    pub node_role: String,
    pub debug: bool,
}

impl BotConfig {
    pub fn from_env() -> Self {
        Self {
            owner_jid: env::var("OWNER_JID")
                .unwrap_or_else(|_| "639972367773@s.whatsapp.net".to_string()),
            prefix: env::var("PREFIX").unwrap_or_else(|_| "#".to_string()),
            prefixes: vec![
                "/".to_string(),
                "!".to_string(),
                "#".to_string(),
                ".".to_string(),
                ":".to_string(),
                "?:".to_string(),
            ],
            sessions_dir: env::var("SESSIONS_DIR").unwrap_or_else(|_| "sessions".to_string()),
            mongodb_uri: env::var("MONGODB_URI").unwrap_or_else(|_| {
                "mongodb+srv://Vercel-Admin-soblend-redzmey-spaceworkflow:SOquhfF8HTxqFcTw@soblend-redzmey-spacewo.7aubqkc.mongodb.net/?retryWrites=true&w=majority".to_string()
            }),
            cluster_port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            cluster_secret: env::var("CLUSTER_SECRET")
                .unwrap_or_else(|_| "soblend_secret_123".to_string()),
            node_id: env::var("NODE_ID").unwrap_or_else(|_| "main-node".to_string()),
            node_role: env::var("NODE_ROLE").unwrap_or_else(|_| "main".to_string()),
            debug: env::var("DEBUG").map(|v| v == "true").unwrap_or(false),
        }
    }

    pub fn is_owner(&self, jid: &str) -> bool {
        let owner_num = extract_number(jid);
        let owners = [
            "57324709996",
            "526631079388",
            "5356795360",
            "18096521129",
            "5491125741379",
            "639972367773",
            "5493777606761",
            "78224272920733",
            "85968115769454",
            "573013751308",
        ];
        owners.iter().any(|o| *o == owner_num)
    }
}

pub fn extract_number(jid: &str) -> String {
    let base = jid.split('@').next().unwrap_or("");
    let num = if base.contains(':') {
        base.split(':').next().unwrap_or(base)
    } else {
        base
    };
    num.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub mod rate_limit {
    pub const COMMAND_COOLDOWN_MS: u64 = 1000;
    pub const SPAM_THRESHOLD: u32 = 5;
    pub const SPAM_WINDOW_MS: u64 = 10_000;
    pub const SPAM_TIMEOUT_MS: u64 = 30_000;
}

pub mod errors {
    pub const COMMAND_NOT_FOUND: &str = "ꕢ Comando no encontrado.";
    pub const RATE_LIMITED: &str = "ꕢ Estás enviando comandos muy rápido. Espera un momento.";
    pub const SPAM_DETECTED: &str = "ꕢ Has sido silenciado por spam. Espera 30 segundos.";
    pub const GENERIC_ERROR: &str = "ꕢ Ocurrió un error al ejecutar el comando.";
    pub const DB_ERROR: &str = "ꕢ Error de base de datos. Inténtalo de nuevo.";
    pub const PERMISSION_DENIED: &str = "ꕢ No tienes permiso para usar este comando.";
    pub const GROUPS_ONLY: &str = "ꕢ Este comando solo funciona en grupos.";
    pub const ADMIN_ONLY: &str = "ꕢ Solo los administradores pueden usar este comando.";
    pub const BOT_ADMIN_REQUIRED: &str = "ꕢ Necesito ser administrador para realizar esta acción.";
}

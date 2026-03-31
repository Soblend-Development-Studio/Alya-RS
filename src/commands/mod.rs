pub mod admin;
pub mod ai;
pub mod economy;
pub mod gacha;
pub mod games;
pub mod tools;
pub mod search;
pub mod downloads;
pub mod fun;
pub mod nsfw;
pub mod general;
pub mod owner;
pub mod group;
pub mod level;

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;

use crate::db::DatabaseService;
use crate::services::{GachaService, ShopService, LevelService};
use crate::utils::CacheManager;
use crate::config::BotConfig;

/// The context passed to every command handler
pub struct CommandContext {
    pub client: Arc<dyn WhatsAppClient + Send + Sync>,
    pub sender: String,
    pub sender_lid: Option<String>,
    pub sender_phone: Option<String>,
    pub chat_id: String,
    pub is_group: bool,
    pub body: String,
    pub args: Vec<String>,
    pub command: String,
    pub prefix: String,
    pub is_from_me: bool,
    pub is_owner: bool,
    pub push_name: Option<String>,
    pub db: Arc<DatabaseService>,
    pub gacha: Arc<GachaService>,
    pub shop: Arc<ShopService>,
    pub level: Arc<LevelService>,
    pub cache: Arc<CacheManager>,
    pub config: Arc<BotConfig>,
    // Raw message data for media downloads, replies, etc.
    pub raw_message: Option<serde_json::Value>,
}

impl CommandContext {
    pub async fn reply(&self, text: &str) -> Result<()> {
        self.client.send_text(&self.chat_id, text).await
    }

    pub async fn reply_styled(&self, text: &str) -> Result<()> {
        self.reply(&crate::utils::format::style_text(text)).await
    }

    pub async fn send_image(&self, url: &str, caption: &str) -> Result<()> {
        self.client.send_image(&self.chat_id, url, caption).await
    }

    pub fn get_currency_name_sync(&self) -> String {
        "coins".to_string()
    }

    pub async fn get_currency_name(&self) -> String {
        if !self.is_group {
            return "coins".to_string();
        }
        match self.db.get_group(&self.chat_id).await {
            Ok(g) => g.settings.currency_name,
            Err(_) => "coins".to_string(),
        }
    }
}

/// Abstraction over the WhatsApp client so commands don't depend directly on the WA library
#[async_trait]
pub trait WhatsAppClient {
    async fn send_text(&self, to: &str, text: &str) -> Result<()>;
    async fn send_image(&self, to: &str, url: &str, caption: &str) -> Result<()>;
    async fn send_image_bytes(&self, to: &str, data: Vec<u8>, caption: &str) -> Result<()>;
    async fn send_audio(&self, to: &str, url: &str) -> Result<()>;
    async fn send_video(&self, to: &str, url: &str, caption: &str) -> Result<()>;
    async fn send_sticker(&self, to: &str, data: Vec<u8>) -> Result<()>;
    async fn kick_participant(&self, group_id: &str, participant: &str) -> Result<()>;
    async fn add_participant(&self, group_id: &str, participant: &str) -> Result<()>;
    async fn promote_participant(&self, group_id: &str, participant: &str) -> Result<()>;
    async fn demote_participant(&self, group_id: &str, participant: &str) -> Result<()>;
    async fn get_group_participants(&self, group_id: &str) -> Result<Vec<Participant>>;
    async fn get_group_admins(&self, group_id: &str) -> Result<Vec<String>>;
    async fn is_admin(&self, group_id: &str, participant: &str) -> Result<bool>;
    async fn is_bot_admin(&self, group_id: &str) -> Result<bool>;
    async fn delete_message(&self, chat_id: &str, message_id: &str, from_me: bool) -> Result<()>;
    fn my_jid(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub id: String,
    pub is_admin: bool,
    pub is_super_admin: bool,
}

/// Trait that every command module must implement
#[async_trait]
pub trait Command: Send + Sync {
    /// List of command triggers (e.g. ["balance", "bal", "saldo"])
    fn triggers(&self) -> &[&str];

    /// Category for help display
    fn category(&self) -> &str { "general" }

    /// Optional help text
    fn help(&self) -> &str { "" }

    /// Main execution handler
    async fn execute(&self, ctx: &CommandContext) -> Result<()>;

    /// Optional "before" handler that runs on every message (middleware)
    async fn before(&self, _ctx: &CommandContext) -> Result<bool> {
        Ok(true) // true = continue processing
    }
}

/// The command registry
pub struct CommandRegistry {
    commands: HashMap<String, Arc<dyn Command>>,
    before_handlers: Vec<Arc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            before_handlers: vec![],
        }
    }

    pub fn register(&mut self, cmd: Arc<dyn Command>) {
        for trigger in cmd.triggers() {
            self.commands.insert(trigger.to_string(), cmd.clone());
        }
    }

    pub fn register_with_before(&mut self, cmd: Arc<dyn Command>) {
        self.register(cmd.clone());
        self.before_handlers.push(cmd);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Command>> {
        self.commands.get(name).cloned()
    }

    pub fn get_all_triggers(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    pub fn before_handlers(&self) -> &[Arc<dyn Command>] {
        &self.before_handlers
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Build and register all commands
pub fn build_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Admin commands
    registry.register(Arc::new(admin::welcome::WelcomeCommand));
    registry.register(Arc::new(admin::goodbye::GoodbyeCommand));
    registry.register_with_before(Arc::new(admin::antilink::AntilinkCommand));
    registry.register(Arc::new(admin::kick::KickCommand));
    registry.register(Arc::new(admin::ban::BanCommand));
    registry.register(Arc::new(admin::promote::PromoteCommand));
    registry.register(Arc::new(admin::demote::DemoteCommand));
    registry.register(Arc::new(admin::tag::TagCommand));
    registry.register(Arc::new(admin::warn::WarnCommand));
    registry.register(Arc::new(admin::settings::SettingsCommand));
    registry.register(Arc::new(admin::kickall::KickallCommand));
    registry.register(Arc::new(admin::delete_cmd::DeleteCommand));
    registry.register(Arc::new(admin::update::UpdateCommand));

    // Economy commands
    registry.register(Arc::new(economy::balance::BalanceCommand));
    registry.register(Arc::new(economy::daily::DailyCommand));
    registry.register(Arc::new(economy::work::WorkCommand));
    registry.register(Arc::new(economy::crime::CrimeCommand));
    registry.register(Arc::new(economy::slut::SlutCommand));
    registry.register(Arc::new(economy::deposit::DepositCommand));
    registry.register(Arc::new(economy::withdraw::WithdrawCommand));
    registry.register(Arc::new(economy::steal::StealCommand));
    registry.register(Arc::new(economy::shop::ShopCommand));
    registry.register(Arc::new(economy::inventory::InventoryCommand));
    registry.register(Arc::new(economy::roulette::RouletteCommand));
    registry.register(Arc::new(economy::coinflip::CoinflipCommand));
    registry.register(Arc::new(economy::blackjack::BlackjackCommand));
    registry.register(Arc::new(economy::beg::BegCommand));
    registry.register(Arc::new(economy::givecoins::GivecoinsCommand));
    registry.register(Arc::new(economy::fish::FishCommand));
    registry.register(Arc::new(economy::setcoins::SetcoinsCommand));
    registry.register(Arc::new(economy::board::BoardCommand));
    registry.register(Arc::new(economy::info::InfoCommand));

    // Gacha commands
    registry.register(Arc::new(gacha::rollwaifu::RollwaifuCommand));
    registry.register(Arc::new(gacha::claim::ClaimCommand));
    registry.register(Arc::new(gacha::harem::HaremCommand));
    registry.register(Arc::new(gacha::winfo::WinfoCommand));
    registry.register(Arc::new(gacha::trade::TradeCommand));
    registry.register(Arc::new(gacha::sell::SellCommand));
    registry.register(Arc::new(gacha::give::GiveCommand));
    registry.register(Arc::new(gacha::dar::DarCommand));
    registry.register(Arc::new(gacha::steal_waifu::StealWaifuCommand));
    registry.register(Arc::new(gacha::antirobo::AntiroboCommand));
    registry.register(Arc::new(gacha::desbloquear::DesbloquearCommand));
    registry.register(Arc::new(gacha::listwaifu::ListwaifuCommand));
    registry.register(Arc::new(gacha::wtop::WtopCommand));
    registry.register(Arc::new(gacha::wimage::WimageCommand));
    registry.register(Arc::new(gacha::wvideo::WvideoCommand));
    registry.register(Arc::new(gacha::wcow::WcowCommand));
    registry.register(Arc::new(gacha::vote::VoteCommand));
    registry.register(Arc::new(gacha::ainfo::AinfoCommand));
    registry.register(Arc::new(gacha::addrw::AddrwCommand));
    registry.register(Arc::new(gacha::vchars::VcharsCommand));
    registry.register(Arc::new(gacha::delwaifu::DelwaifuCommand));
    registry.register(Arc::new(gacha::resetwaifus::ResetwaifusCommand));
    registry.register(Arc::new(gacha::giveall::GiveallCommand));

    // Games commands
    registry.register(Arc::new(games::tictactoe::TictactoeCommand));
    registry.register(Arc::new(games::trivia::TriviaCommand));
    registry.register(Arc::new(games::math::MathCommand));
    registry.register(Arc::new(games::dare::DareCommand));
    registry.register(Arc::new(games::ppt::PptCommand));
    registry.register(Arc::new(games::slot::SlotCommand));
    registry.register(Arc::new(games::fight::FightCommand));
    registry.register(Arc::new(games::marry::MarryCommand));
    registry.register(Arc::new(games::ship::ShipCommand));
    registry.register(Arc::new(games::adivinanza::AdivinanzaCommand));

    // AI commands
    registry.register(Arc::new(ai::chatgpt::ChatgptCommand));
    registry.register(Arc::new(ai::gemini::GeminiCommand));
    registry.register(Arc::new(ai::claude::ClaudeCommand));
    registry.register(Arc::new(ai::copilot::CopilotCommand));
    registry.register(Arc::new(ai::sora::SoraCommand));
    registry.register(Arc::new(ai::vision::VisionCommand));

    // Tools commands
    registry.register(Arc::new(tools::ping::PingCommand));
    registry.register(Arc::new(tools::sticker::StickerCommand));
    registry.register(Arc::new(tools::toimg::ToimgCommand));
    registry.register(Arc::new(tools::pfp::PfpCommand));
    registry.register(Arc::new(tools::hd::HdCommand));
    registry.register(Arc::new(tools::speak::SpeakCommand));
    registry.register(Arc::new(tools::ss::SsCommand));
    registry.register(Arc::new(tools::statsbot::StatsbotCommand));
    registry.register(Arc::new(tools::bots::BotsCommand));
    registry.register(Arc::new(tools::get::GetCommand));
    registry.register(Arc::new(tools::upload::UploadCommand));
    registry.register(Arc::new(tools::suggest::SuggestCommand));
    registry.register(Arc::new(tools::obtenerinfo::ObtenerinfoCommand));

    // Search commands
    registry.register(Arc::new(search::wikipedia::WikipediaCommand));
    registry.register(Arc::new(search::pinterest::PinterestCommand));
    registry.register(Arc::new(search::lyrics::LyricsCommand));
    registry.register(Arc::new(search::spotify::SpotifySearchCommand));
    registry.register(Arc::new(search::tiktok_search::TiktokSearchCommand));
    registry.register(Arc::new(search::fandom::FandomCommand));
    registry.register(Arc::new(search::apk::ApkCommand));
    registry.register(Arc::new(search::soundcloud::SoundcloudCommand));
    registry.register(Arc::new(search::ttuser::TtuserCommand));

    // Downloads commands
    registry.register(Arc::new(downloads::youtube::YoutubeCommand));
    registry.register(Arc::new(downloads::ytmp3::Ytmp3Command));
    registry.register(Arc::new(downloads::ytmp4::Ytmp4Command));
    registry.register(Arc::new(downloads::tiktok::TiktokCommand));
    registry.register(Arc::new(downloads::instagram::InstagramCommand));
    registry.register(Arc::new(downloads::facebook::FacebookCommand));
    registry.register(Arc::new(downloads::spotify::SpotifyCommand));
    registry.register(Arc::new(downloads::play::PlayCommand));
    registry.register(Arc::new(downloads::mediafire::MediafireCommand));

    // Fun commands
    registry.register(Arc::new(fun::hug::HugCommand));
    registry.register(Arc::new(fun::kiss::KissCommand));
    registry.register(Arc::new(fun::slap::SlapCommand));
    registry.register(Arc::new(fun::dance::DanceCommand));
    registry.register(Arc::new(fun::cry::CryCommand));
    registry.register(Arc::new(fun::love::LoveCommand));
    registry.register(Arc::new(fun::angry::AngryCommand));
    registry.register(Arc::new(fun::bored::BoredCommand));
    registry.register(Arc::new(fun::coffee::CoffeeCommand));
    registry.register(Arc::new(fun::kill::KillCommand));
    registry.register(Arc::new(fun::sleep::SleepCommand));
    registry.register(Arc::new(fun::gay::GayCommand));
    registry.register(Arc::new(fun::textpro::TextproCommand));
    registry.register(Arc::new(fun::wanted::WantedCommand));

    // NSFW commands
    registry.register(Arc::new(nsfw::settings::NsfwSettingsCommand));
    registry.register(Arc::new(nsfw::himages::HimagesCommand));
    registry.register(Arc::new(nsfw::hbikini::HbikiniCommand));
    registry.register(Arc::new(nsfw::showtits::ShowtitsCommand));
    registry.register(Arc::new(nsfw::pajawoman::PajawomanCommand));
    registry.register(Arc::new(nsfw::fuck::FuckCommand));
    registry.register(Arc::new(nsfw::cum::CumCommand));
    registry.register(Arc::new(nsfw::xnxx::XnxxCommand));
    registry.register(Arc::new(nsfw::pornvideo::PornvideoCommand));

    // Group commands
    registry.register(Arc::new(group::info::GroupInfoCommand));
    registry.register(Arc::new(group::link::GroupLinkCommand));

    // Level commands
    registry.register(Arc::new(level::profile::ProfileCommand));

    // Owner commands
    registry.register(Arc::new(owner::generate::GenerateCommand));
    registry.register(Arc::new(owner::send::SendCommand));
    registry.register(Arc::new(owner::broadcast::BroadcastCommand));
    registry.register(Arc::new(owner::join::JoinCommand));
    registry.register(Arc::new(owner::off::OffCommand));
    registry.register(Arc::new(owner::restart::RestartCommand));
    registry.register(Arc::new(owner::wcoins::WcoinsCommand));
    registry.register(Arc::new(owner::delbaltop::DelbaltopCommand));
    registry.register(Arc::new(owner::antiprivado::AntiprivadoCommand));
    registry.register(Arc::new(owner::viewp::ViewpCommand));
    registry.register(Arc::new(owner::disk_check::DiskCheckCommand));
    registry.register(Arc::new(owner::addwaifu::AddwaifuCommand));

    // General commands
    registry.register(Arc::new(general::help::HelpCommand));

    registry
}

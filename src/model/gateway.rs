//! Models pertaining to the gateway.

use url::Url;

use super::prelude::*;
use super::utils::*;

/// A representation of the data retrieved from the bot gateway endpoint.
///
/// This is different from the [`Gateway`], as this includes the number of
/// shards that Discord recommends to use for a bot user.
///
/// This is only applicable to bot users.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#get-gateway-bot-json-response).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BotGateway {
    /// Information describing how many gateway sessions you can initiate within
    /// a ratelimit period.
    pub session_start_limit: SessionStartLimit,
    /// The number of shards that is recommended to be used by the current bot
    /// user.
    pub shards: u64,
    /// The gateway to connect to.
    pub url: String,
}

/// Representation of an activity that a [`User`] is performing.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Activity {
    /// The ID of the application for the activity.
    pub application_id: Option<ApplicationId>,
    /// Images for the presence and their texts.
    pub assets: Option<ActivityAssets>,
    /// What the user is doing.
    pub details: Option<String>,
    /// Activity flags describing what the payload includes.
    pub flags: Option<ActivityFlags>,
    /// Whether or not the activity is an instanced game session.
    pub instance: Option<bool>,
    /// The type of activity being performed
    #[serde(default, rename = "type")]
    pub kind: ActivityType,
    /// The name of the activity.
    pub name: String,
    /// Information about the user's current party.
    pub party: Option<ActivityParty>,
    /// Secrets for Rich Presence joining and spectating.
    pub secrets: Option<ActivitySecrets>,
    /// The user's current party status.
    pub state: Option<String>,
    /// Emoji currently used in custom status
    pub emoji: Option<ActivityEmoji>,
    /// Unix timestamps for the start and/or end times of the activity.
    pub timestamps: Option<ActivityTimestamps>,
    /// The sync ID of the activity. Mainly used by the Spotify activity
    /// type which uses this parameter to store the track ID.
    #[cfg(feature = "unstable_discord_api")]
    pub sync_id: Option<String>,
    /// The session ID of the activity. Reserved for specific activity
    /// types, such as the Activity that is transmitted when a user is
    /// listening to Spotify.
    #[cfg(feature = "unstable_discord_api")]
    pub session_id: Option<String>,
    /// The Stream URL if [`Self::kind`] is [`ActivityType::Streaming`].
    pub url: Option<Url>,
    /// The buttons of this activity.
    ///
    /// **Note**: There can only be up to 2 buttons.
    #[serde(default, deserialize_with = "deserialize_buttons")]
    pub buttons: Vec<ActivityButton>,
}

#[cfg(feature = "model")]
impl Activity {
    /// Common constructor for the different `ActivityType`s.
    fn new(name: String, kind: ActivityType) -> Self {
        Self {
            application_id: None,
            assets: None,
            details: None,
            flags: None,
            instance: None,
            kind,
            name,
            party: None,
            secrets: None,
            state: None,
            emoji: None,
            timestamps: None,
            #[cfg(feature = "unstable_discord_api")]
            sync_id: None,
            #[cfg(feature = "unstable_discord_api")]
            session_id: None,
            url: None,
            buttons: vec![],
        }
    }

    /// Creates a [`Activity`] struct that appears as a `Playing <name>` status.
    ///
    /// **Note**: Maximum `name` length is 128.
    ///
    /// # Examples
    ///
    /// Create a command that sets the current activity:
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "client")]
    /// use serenity::client::Context;
    /// # #[cfg(feature = "framework")]
    /// use serenity::framework::standard::{macros::command, Args, CommandResult};
    /// use serenity::model::channel::Message;
    /// use serenity::model::gateway::Activity;
    ///
    /// # #[cfg(feature = "framework")]
    /// #[command]
    /// async fn activity(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    ///     let name = args.message();
    ///     ctx.set_activity(Activity::playing(&name)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn playing<N>(name: N) -> Activity
    where
        N: ToString,
    {
        Activity::new(name.to_string(), ActivityType::Playing)
    }

    /// Creates an [`Activity`] struct that appears as a `Streaming <name>`
    /// status.
    ///
    /// **Note**: Maximum `name` length is 128.
    ///
    /// # Examples
    ///
    /// Create a command that sets the current streaming status:
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "client")]
    /// use serenity::client::Context;
    /// # #[cfg(feature = "framework")]
    /// use serenity::framework::standard::{macros::command, Args, CommandResult};
    /// use serenity::model::channel::Message;
    /// use serenity::model::gateway::Activity;
    ///
    /// # #[cfg(feature = "framework")]
    /// #[command]
    /// async fn stream(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    ///     const STREAM_URL: &str = "...";
    ///
    ///     let name = args.message();
    ///     ctx.set_activity(Activity::streaming(&name, STREAM_URL)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn streaming<N, U>(name: N, url: U) -> Activity
    where
        N: ToString,
        U: AsRef<str>,
    {
        Activity {
            url: Some(Url::parse(url.as_ref()).expect("Failed to parse url")),
            ..Activity::new(name.to_string(), ActivityType::Streaming)
        }
    }

    /// Creates a [`Activity`] struct that appears as a `Listening to <name>` status.
    ///
    /// **Note**: Maximum `name` length is 128.
    ///
    /// # Examples
    ///
    /// Create a command that sets the current listening status:
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "client")]
    /// use serenity::client::Context;
    /// # #[cfg(feature = "framework")]
    /// use serenity::framework::standard::{macros::command, Args, CommandResult};
    /// use serenity::model::channel::Message;
    /// use serenity::model::gateway::Activity;
    ///
    /// # #[cfg(feature = "framework")]
    /// #[command]
    /// async fn listen(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    ///     let name = args.message();
    ///     ctx.set_activity(Activity::listening(&name)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn listening<N>(name: N) -> Activity
    where
        N: ToString,
    {
        Activity::new(name.to_string(), ActivityType::Listening)
    }

    /// Creates a [`Activity`] struct that appears as a `Watching <name>` status.
    ///
    /// **Note**: Maximum `name` length is 128.
    ///
    /// # Examples
    ///
    /// Create a command that sets the current cometing status:
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "client")]
    /// use serenity::client::Context;
    /// # #[cfg(feature = "framework")]
    /// use serenity::framework::standard::{macros::command, Args, CommandResult};
    /// use serenity::model::channel::Message;
    /// use serenity::model::gateway::Activity;
    ///
    /// # #[cfg(feature = "framework")]
    /// #[command]
    /// async fn watch(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    ///     let name = args.message();
    ///     ctx.set_activity(Activity::watching(&name)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn watching<N>(name: N) -> Activity
    where
        N: ToString,
    {
        Activity::new(name.to_string(), ActivityType::Watching)
    }

    /// Creates a [`Activity`] struct that appears as a `Competing in <name>` status.
    ///
    /// **Note**: Maximum `name` length is 128.
    ///
    /// # Examples
    ///
    /// Create a command that sets the current cometing status:
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "client")]
    /// use serenity::client::Context;
    /// # #[cfg(feature = "framework")]
    /// use serenity::framework::standard::{macros::command, Args, CommandResult};
    /// use serenity::model::channel::Message;
    /// use serenity::model::gateway::Activity;
    ///
    /// # #[cfg(feature = "framework")]
    /// #[command]
    /// async fn compete(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    ///     let name = args.message();
    ///     ctx.set_activity(Activity::competing(&name)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn competing<N>(name: N) -> Activity
    where
        N: ToString,
    {
        Activity::new(name.to_string(), ActivityType::Competing)
    }
}

/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-buttons).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityButton {
    /// The text shown on the button.
    pub label: String,
    /// The url opened when clicking the button.
    ///
    /// **Note**: Bots cannot access activity button URL.
    #[serde(default)]
    pub url: String,
}

/// The assets for an activity.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ActivityAssets {
    /// The ID for a large asset of the activity, usually a snowflake.
    pub large_image: Option<String>,
    /// Text displayed when hovering over the large image of the activity.
    pub large_text: Option<String>,
    /// The ID for a small asset of the activity, usually a snowflake.
    pub small_image: Option<String>,
    /// Text displayed when hovering over the small image of the activity.
    pub small_text: Option<String>,
}

bitflags! {
    /// A set of flags defining what is in an activity's payload.
    ///
    /// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-flags).
    #[derive(Default)]
    pub struct ActivityFlags: u64 {
        /// Whether the activity is an instance activity.
        const INSTANCE = 1 << 0;
        /// Whether the activity is joinable.
        const JOIN = 1 << 1;
        /// Whether the activity can be spectated.
        const SPECTATE = 1 << 2;
        /// Whether a request can be sent to join the user's party.
        const JOIN_REQUEST = 1 << 3;
        /// Whether the activity can be synced.
        const SYNC = 1 << 4;
        /// Whether the activity can be played.
        const PLAY = 1 << 5;
        /// Whether the activity party is friend only.
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        /// Whether the activity party is in a voice channel.
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        /// Whether the activity can be embedded.
        const EMBEDDED = 1 << 8;
    }
}

/// Information about an activity's party.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-party).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ActivityParty {
    /// The ID of the party.
    pub id: Option<String>,
    /// Used to show the party's current and maximum size.
    pub size: Option<[u64; 2]>,
}

/// Secrets for an activity.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-secrets).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ActivitySecrets {
    /// The secret for joining a party.
    pub join: Option<String>,
    /// The secret for a specific instanced match.
    #[serde(rename = "match")]
    pub match_: Option<String>,
    /// The secret for spectating an activity.
    pub spectate: Option<String>,
}

/// Representation of an emoji used in a custom status
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-emoji).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityEmoji {
    /// The name of the emoji.
    pub name: String,
    /// The id of the emoji.
    pub id: Option<EmojiId>,
    /// Whether this emoji is animated.
    pub animated: Option<bool>,
}

/// [Discord docs](https://discord.com/developers/docs/topics/gateway#activity-object-activity-types).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ActivityType {
    /// An indicator that the user is playing a game.
    Playing = 0,
    /// An indicator that the user is streaming to a service.
    Streaming = 1,
    /// An indicator that the user is listening to something.
    Listening = 2,
    /// An indicator that the user is watching something.
    Watching = 3,
    /// An indicator that the user uses custom statuses
    Custom = 4,
    /// An indicator that the user is competing somewhere.
    Competing = 5,
    /// An indicator that the activity is of unknown type.
    Unknown = !0,
}

enum_number!(ActivityType {
    Playing,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing
});

impl Default for ActivityType {
    fn default() -> Self {
        ActivityType::Playing
    }
}

/// A representation of the data retrieved from the gateway endpoint.
///
/// For the bot-specific gateway, refer to [`BotGateway`].
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#get-gateway-example-response).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Gateway {
    /// The gateway to connect to.
    pub url: String,
}

/// Information detailing the current active status of a [`User`].
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#client-status-object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientStatus {
    pub desktop: Option<OnlineStatus>,
    pub mobile: Option<OnlineStatus>,
    pub web: Option<OnlineStatus>,
}

/// Information about the user of a [`Presence`] event.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway-events#presence-update).
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(default)]
pub struct PresenceUser {
    pub id: UserId,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "discriminator::option")]
    pub discriminator: Option<u16>,
    pub email: Option<String>,
    pub mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub name: Option<String>,
    pub verified: Option<bool>,
    pub public_flags: Option<UserPublicFlags>,
}

impl PresenceUser {
    /// Attempts to convert this [`PresenceUser`] instance into a [`User`].
    ///
    /// If one of [`User`]'s required fields is None in `self`, None is returned.
    #[must_use]
    pub fn into_user(self) -> Option<User> {
        Some(User {
            avatar: self.avatar,
            bot: self.bot?,
            discriminator: self.discriminator?,
            id: self.id,
            name: self.name?,
            public_flags: self.public_flags,
            banner: None,
            accent_colour: None,
            member: None,
        })
    }

    /// Attempts to convert this [`PresenceUser`] instance into a [`User`].
    ///
    /// Will clone individual fields if needed.
    ///
    /// If one of [`User`]'s required fields is None in `self`, None is returned.
    #[must_use]
    pub fn to_user(&self) -> Option<User> {
        Some(User {
            avatar: self.avatar.clone(),
            bot: self.bot?,
            discriminator: self.discriminator?,
            id: self.id,
            name: self.name.clone()?,
            public_flags: self.public_flags,
            banner: None,
            accent_colour: None,
            member: None,
        })
    }

    #[cfg(feature = "cache")] // method is only used with the cache feature enabled
    pub(crate) fn update_with_user(&mut self, user: User) {
        self.id = user.id;
        if let Some(avatar) = user.avatar {
            self.avatar = Some(avatar);
        }
        self.bot = Some(user.bot);
        self.discriminator = Some(user.discriminator);
        self.name = Some(user.name);
        if let Some(public_flags) = user.public_flags {
            self.public_flags = Some(public_flags);
        }
    }
}

/// Information detailing the current online status of a [`User`].
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#presence-update-presence-update-event-fields).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Presence {
    /// [`User`]'s current activities.
    #[serde(default)]
    pub activities: Vec<Activity>,
    /// The devices a user are currently active on, if available.
    #[serde(default)]
    pub client_status: Option<ClientStatus>,
    /// The `GuildId` the presence update is coming from.
    pub guild_id: Option<GuildId>,
    /// The user's online status.
    pub status: OnlineStatus,
    /// Data about the associated user.
    pub user: PresenceUser,
}

/// An initial set of information given after IDENTIFYing to the gateway.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#ready-ready-event-fields).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Ready {
    pub application: PartialCurrentApplicationInfo,
    pub guilds: Vec<UnavailableGuild>,
    #[serde(default, with = "presences")]
    pub presences: HashMap<UserId, Presence>,
    #[serde(default, with = "private_channels")]
    pub private_channels: HashMap<ChannelId, Channel>,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    #[serde(default, rename = "_trace")]
    pub trace: Vec<String>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}

/// Information describing how many gateway sessions you can initiate within a
/// ratelimit period.
///
/// [Discord docs](https://discord.com/developers/docs/topics/gateway#session-start-limit-object-session-start-limit-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SessionStartLimit {
    /// The number of sessions that you can still initiate within the current
    /// ratelimit period.
    pub remaining: u64,
    /// The number of milliseconds until the ratelimit period resets.
    pub reset_after: u64,
    /// The total number of session starts within the ratelimit period allowed.
    pub total: u64,
    /// The number of identify requests allowed per 5 seconds.
    pub max_concurrency: u64,
}
/// Timestamps of when a user started and/or is ending their activity.
///
/// [Discord docs](https://discord.com/developers/docs/game-sdk/activities#data-models-activitytimestamps-struct).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ActivityTimestamps {
    pub end: Option<u64>,
    pub start: Option<u64>,
}


use base64::encode;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::routing::Route;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum ContextProps {
    None,
    Empty,

    Call,
    ContextMenu,

    Friends,
    /// Add Friend
    AddFriend,
    /// Friend Suggestion
    FriendSuggestion,
    /// Add Friends to DM
    AddFriendsToDM,

    /// Spectate Modal
    SpectateModal,
    /// Desktop Invite Modal
    DesktopInviteModal(InviteMeta),

    /// Group DM
    GroupDM,
    DMSpamModal,
    /// DM Channel
    DMChannel,
    /// New Group DM
    NewGroupDM,
    /// Group DM Invite Create
    GroupDMInviteCreate,

    /// ReportMenuBlockUser-iOS
    ReportMenuBlockUserIOS,

    /// User Profile
    UserProfile,

    /// overlay_locked_activated
    OverlayLockedActivated,
    /// overlay_unlocked
    OverlayUnlocked,
    /// overlay_unlocked_pinned
    OverlayUnlockedPinned,

    RPC,

    /// Markdown Link
    MarkdownLink(InviteMeta),

    /// Application Directory
    ApplicationDirectory,

    /// Invite Button Embed,
    InviteButtonEmbed(InviteMeta),

    /// Application Store Verified Guild Invite - Lurker
    ApplicationStoreVerifiedGuildInviteLurker,
    /// Chat Input Blocker - Lurker Mode
    ChatInputBlockerLurkerMode,
    /// iOS Guild Discovery - Lurker
    IOSGuildDiscoveryLurker,
    /// Notice - Lurker Mode
    NoticeLurkerMode,

    /// Directory Channel Entry
    DirectoryChannelEntry,
    /// Directory Channel Events
    DirectoryChannelEvents,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct InviteMeta {
    pub guild_id: u64,
    pub channel_id: u64,
    pub channel_type: i32,
    pub message_id: u64,
    pub guild_scheduled_event_id: u64,
}

#[derive(Serialize)]
struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl Location {
    pub fn new(location: Option<&str>) -> Self {
        Self {
            location: location.map(ToString::to_string),
        }
    }
}

#[allow(dead_code)]
const LURKING_SOURCES: (&'static str, &'static str) =
    ("Chat Input Blocker - Lurker Mode", "Notice - Lurker Mode");

impl ContextProps {
    pub(crate) fn get_string(self) -> String {
        let location = Location::new(match self {
            Self::Call => Some("Call"),
            Self::ContextMenu => Some("ContextMenu"),
            Self::Friends => Some("Friends"),
            Self::AddFriend => Some("Add Friend"),
            Self::FriendSuggestion => Some("Friend Suggestion"),
            Self::AddFriendsToDM => Some("Add Friends to DM"),
            Self::SpectateModal => Some("Spectate Modal"),
            Self::DesktopInviteModal(..) => Some("Desktop Invite Modal"),
            Self::GroupDM => Some("Group DM"),
            Self::DMSpamModal => Some("DMSpamModal"),
            Self::DMChannel => Some("DM Channel"),
            Self::NewGroupDM => Some("New Group DM"),
            Self::GroupDMInviteCreate => Some("Group DM Invite Create"),
            Self::ReportMenuBlockUserIOS => Some("ReportMenuBlockUser-iOS"),
            Self::UserProfile => Some("User Profile"),
            Self::OverlayLockedActivated => Some("overlay_locked_activated"),
            Self::OverlayUnlocked => Some("overlay_unlocked"),
            Self::OverlayUnlockedPinned => Some("overlay_unlocked_pinned"),
            Self::RPC => Some("RPC"),
            Self::MarkdownLink(..) => Some("Markdown Link"),
            Self::ApplicationDirectory => Some("Application Directory"),
            Self::InviteButtonEmbed(..) => Some("Invite Button Embed"),
            Self::ApplicationStoreVerifiedGuildInviteLurker => {
                Some("Application Store Verified Guild Invite - Lurker")
            }
            Self::ChatInputBlockerLurkerMode => Some("Chat Input Blocker - Lurker Mode"),
            Self::IOSGuildDiscoveryLurker => Some("iOS Guild Discovery - Lurker"),
            Self::NoticeLurkerMode => Some("Notice - Lurker Mode"),
            Self::DirectoryChannelEntry => Some("Directory Channel Entry"),
            Self::DirectoryChannelEvents => Some("Directory Channel Events"),

            _ => None,
        });

        encode(serde_json::to_string::<Location>(&location).expect("Invalid context properties"))
    }

    pub(crate) fn get_context_props(route: &Route<'_>) -> Self {
        let vec = match route {
            Route::CreatePrivateChannel | Route::GetUserPrivateChannels => {
                vec![ContextProps::NewGroupDM]
            }
            Route::CreateMessage { .. } => {
                vec![
                    ContextProps::Empty,
                    ContextProps::OverlayLockedActivated,
                    ContextProps::OverlayUnlocked,
                    ContextProps::OverlayUnlockedPinned,
                ]
            }
            Route::CreateInvite { .. } => {
                vec![ContextProps::RPC, ContextProps::GroupDMInviteCreate]
            }
            Route::DeleteInvite { .. }
            | Route::GetInvite { .. }
            | Route::GetInviteWithExpiration { .. } => {
                // TODO: implement the extra fields required for this endpoint, only important when accepting invites using OAuth2
                vec![ContextProps::Empty]
            }
            Route::UpdateCurrentMember { .. } => {
                vec![
                    ContextProps::ApplicationStoreVerifiedGuildInviteLurker,
                    ContextProps::ChatInputBlockerLurkerMode,
                    ContextProps::ChatInputBlockerLurkerMode,
                    ContextProps::DirectoryChannelEntry,
                    ContextProps::DirectoryChannelEvents,
                    ContextProps::IOSGuildDiscoveryLurker,
                    ContextProps::NoticeLurkerMode,
                ]
            }
            _ => vec![],
        };

        vec.choose(&mut rand::thread_rng())
            .unwrap_or(&ContextProps::None)
            .to_owned()
    }

    pub(crate) fn is_some(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

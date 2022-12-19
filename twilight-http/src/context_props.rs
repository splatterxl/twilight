#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum ContextProps {
    None,
    Empty,
    Friends,
    ContextMenu,
    UserProfile,
    AddFriend,
    GuildHeader,
    GroupDM,
    NewGroupDM,
    DMChannel,
    AddFriendsToDM,
    GroupDMInviteCreate,
    App,
    Login,
    Register,
    VerifyEmail,
    AcceptInvite {
        guild_id: u32,
        channel_id: u32,
        channel_type: i32,
    },
    JoinGuildPopup {
        guild_id: u32,
        channel_id: u32,
        channel_type: i32,
    },
    InviteEmbed {
        guild_id: u32,
        channel_id: u32,
        message_id: u32,
        channel_type: i32,
    },
    Lurking {
        source: Option<String>,
    },
}

#[allow(dead_code)]
const LURKING_SOURCES: (&'static str, &'static str) =
    ("Chat Input Blocker - Lurker Mode", "Notice - Lurker Mode");

impl ContextProps {
    pub(crate) fn get_string(self) -> String {
        String::from(match self {
            Self::Friends => "eyJsb2NhdGlvbiI6IkZyaWVuZHMifQ==",
            Self::ContextMenu => "eyJsb2NhdGlvbiI6IkNvbnRleHRNZW51In0=",

            _ => "e30=",
        })
    }

    pub(crate) fn is_some(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

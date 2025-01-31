use super::AutoModerationKeywordPresetType;
use serde::{Deserialize, Serialize};

/// Additional data used to determine whether a rule should be triggered.
///
/// Different fields are relevant based on the value of [`AutoModerationRule::trigger_type`].
///
/// [`AutoModerationRule::trigger_type`]: super::AutoModerationRule::trigger_type
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationTriggerMetadata {
    /// Substrings that will be exempt from triggering the preset type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    /// Substrings which will be searched for in content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_filter: Option<Vec<String>>,
    /// Internally pre-defined wordsets which will be searched for in content.
    ///
    /// A keyword can be a phrase which contains multiple words. Wildcard
    /// symbols can be used to customize how each keyword will be matched. See
    /// [Discord Docs/Keyword Matching Strategies].
    ///
    /// [Discord Docs/Keyword Matching Strategies]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-matching-strategies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<AutoModerationKeywordPresetType>>,
}

#[cfg(test)]
mod tests {
    use super::{AutoModerationKeywordPresetType, AutoModerationTriggerMetadata};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(AutoModerationTriggerMetadata: keyword_filter, presets);
    assert_impl_all!(
        AutoModerationTriggerMetadata: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn trigger_metadata() {
        let value = AutoModerationTriggerMetadata {
            allow_list: Some(Vec::from(["heck".into()])),
            keyword_filter: Some(Vec::from(["shoot".into(), "darn".into()])),
            presets: Some(Vec::from([
                AutoModerationKeywordPresetType::Profanity,
                AutoModerationKeywordPresetType::SexualContent,
                AutoModerationKeywordPresetType::Slurs,
            ])),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AutoModerationTriggerMetadata",
                    len: 3,
                },
                Token::Str("allow_list"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Str("heck"),
                Token::SeqEnd,
                Token::Str("keyword_filter"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::Str("shoot"),
                Token::Str("darn"),
                Token::SeqEnd,
                Token::Str("presets"),
                Token::Some,
                Token::Seq { len: Some(3) },
                Token::U8(u8::from(AutoModerationKeywordPresetType::Profanity)),
                Token::U8(u8::from(AutoModerationKeywordPresetType::SexualContent)),
                Token::U8(u8::from(AutoModerationKeywordPresetType::Slurs)),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}

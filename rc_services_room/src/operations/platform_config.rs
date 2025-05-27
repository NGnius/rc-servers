use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PLATFORM_CONFIG_KEY: u8 = 197;

pub(super) fn platform_config_provider() -> SimpleFunc<165, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PLATFORM_CONFIG_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Any, // obj
            val_ty: TypePrefix::Any, // obj
            items: vec![
                (Typed::Str("BuyPremiumAvailable".into()), Typed::Bool(false.into())),
                (Typed::Str("MainShopButtonAvailable".into()), Typed::Bool(false.into())),
                (Typed::Str("RoboPassButtonAvailable".into()), Typed::Bool(false.into())),
                (Typed::Str("LanguageSelectionAvailable".into()), Typed::Bool(false.into())),
                (Typed::Str("AutoJoinPublicChatRoom".into()), Typed::Bool(true.into())), // TODO maybe?
                (Typed::Str("CanCreateChatRooms".into()), Typed::Bool(true.into())), // TODO
                (Typed::Str("CurseVoiceEnabled".into()), Typed::Bool(false.into())),
                (Typed::Str("DeltaDNAEnabled".into()), Typed::Bool(false.into())),
                (Typed::Str("UseDecimalSystem".into()), Typed::Bool(true.into())),
                (Typed::Str("FeedbackURL".into()), Typed::Str("https://mstdn.ca/@ngram".into())),
                (Typed::Str("SupportURL".into()), Typed::Str("https://git.ngni.us/OpenJam/servers".into())),
                (Typed::Str("WikiURL".into()), Typed::Str("https://git.ngram.ca/OpenJam/servers/wiki".into())),
            ].into(),
        }));
        Ok(params.into())
    })
}

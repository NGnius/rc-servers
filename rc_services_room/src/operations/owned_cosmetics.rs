use polariton_server::operations::{SimpleFunc, SimpleOperation, SimpleOpError, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 50;

fn all_emotes() -> Vec<String> {
    vec![
        // for future reference: part of EmotigramsConfigurableData (-2824383771674178305)
        "Craywave".to_owned(),
        "Chicken".to_owned(),
        "Heart".to_owned(),
        "Lol".to_owned(),
        "Thumbsdown".to_owned(),
        "Thumbsup".to_owned(),
        "Facepalm".to_owned(),
    ]
}

pub(super) fn owned_cosmetics_provider() -> SimpleFunc<23, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::StrArr(all_emotes().into_iter().map(|x| x.into()).collect::<Vec<_>>().into()));
        Ok(params.into())
    })
}

/*pub(super) fn selected_cosmetics_provider() -> SimpleFunc<21, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Str, // str
            custom_ty: None,
            items: vec![Typed::Str("1".into())],
        }));
        Ok(params.into())
    })
}*/

pub(super) struct EmoteListSelected;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for EmoteListSelected {
    type User = crate::UserTy;
    const CODE: u8 = 21;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let emotes_list = user_info.get_emotes().await?;
        params.insert(PARAM_KEY, Typed::StrArr(emotes_list.into_iter().map(|x| x.into()).collect::<Vec<_>>().into()));
        Ok(params)
    }
}

pub(super) fn selected_cosmetics_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, EmoteListSelected> {
    SimpleOpImpl::new(EmoteListSelected)
}

pub(super) struct EmoteListSaver;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for EmoteListSaver {
    type User = crate::UserTy;
    const CODE: u8 = 22;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(emotes) = params.remove(&PARAM_KEY) {
            if let Typed::StrArr(str_arr) = emotes {
                // unused? code path
                let user_info = user.user()?;
                let emotes_list: Vec<String> = str_arr.vec.into_iter().map(|x| x.string).collect();
                user_info.set_emotes(&emotes_list).await?;
            } else if let Typed::Arr(reg_arr) = emotes {
                let user_info = user.user()?;
                let emotes_list: Vec<String> = reg_arr.items.into_iter().filter_map(|x| {
                    if let Typed::Str(s) = x {
                        Some(s.string)
                    } else {
                        None
                    }
                }).collect();
                user_info.set_emotes(&emotes_list).await?;
            }
        }
        Ok(params)
    }
}

pub(super) fn save_selected_cosmetics_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, EmoteListSaver> {
    SimpleOpImpl::new(EmoteListSaver)
}

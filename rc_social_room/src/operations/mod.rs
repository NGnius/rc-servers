mod more_auth;
mod friend_list;
mod settings;
mod clan_invite;
mod clan_info;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .without_state(more_auth::MoreLobbyAuth)
        //.without_state(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
        .without_state(friend_list::friends_provider()) // TODO friend object parsing Token: 0x0200169C RID: 5788
        .without_state(settings::settings_provider()) // TODO save settings persistently
        .without_state(polariton_server::operations::Ack::<43, _>::default()) // get my clan info (this is equivalent to not being in a clan)
        .without_state(clan_invite::clan_invites_provider())
        .without_state(polariton_server::operations::Ack::<19, _>::default()) // get pending platoon invite (this is equivalent to having no pending invite)
        .without_state(clan_info::clan_info_provider())
}

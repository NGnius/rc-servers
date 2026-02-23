pub mod user;
pub mod user_aux;
pub mod permissions;
pub mod garage;
pub mod campaign;
pub mod campaign_difficulty_completion;
pub mod common_query;
pub mod sanction;
pub mod multiplayer_game;
pub mod multiplayer_game_player;
pub mod game_event;
pub mod multiplayer_game_score;
#[cfg(feature = "factory")]
pub mod factory;
pub mod friend;
pub mod clan;
pub mod clan_member;

pub fn parse_int_csv(s: &str) -> Vec<u32> {
    s.split(',').filter_map(|i_as_s| {
        i_as_s.parse().ok()
    }).collect()
}

pub fn dump_csv<T: std::string::ToString>(slice: &[T]) -> String {
    itertools::Itertools::intersperse(slice.iter().map(|x| x.to_string()), ",".to_owned()).collect()
}

pub(crate) const QUEUE_BUFFER: usize = 64;
pub(crate) const USERNAME_REGEX: &str = r"^[[:graph:]]{1,16}$";
pub(crate) const GAMENAME_REGEX: &str = r"^[[:print:]]{1,24}$";
pub(crate) const MAX_PLAYERS: usize = 100;
pub(crate) const MAX_GAME_INSTANCES: usize = 1000;
pub(crate) const MIN_TIMEOUT: f64 = 0.1;
pub(crate) const MAX_TIMEOUT: f64 = 600.0;
pub(crate) const INSTANCE_LIFETIME: u64 = 600;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) const MAGIC: &str = "coco";
pub(crate) const VERSION: u64 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Request {
    Handshake {
        magic: String,
        version: u64,
    },
    GameList,
    GameDescription {
        name: String,
    },
    GameNew {
        name: String,
        params: GameParams,
        args: HashMap<String, String>,
        hidden: bool,
    },
    LobbyList,
    LobbySubscribe,
    LobbyUnsubscribe,
    LobbyJoinMatch {
        id: String,
        name: String,
    },
    LobbyLeaveMatch,
    MatchSpectate {
        id: String,
    },
    MatchLeave,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Reply {
    Handshake { magic: String, version: u64 },
    GameList { games: Vec<String> },
    GameDescription { description: Option<String> },
    GameNew { id: Result<String, String> },
    LobbyList { info: Vec<MatchInfo> },
    LobbySubscribed { seed: Vec<MatchInfo> },
    LobbyJoinedMatch { info: Result<MatchInfo, String> },
    LobbyNew { id: String, info: MatchInfo },
    LobbyUpdate { id: String, info: MatchInfo },
    LobbyDelete { id: String },
    LobbyUnsubscribed,
    LobbyLeavedMatch,
    MatchStarted,
    MatchEnded,
    MatchLeaved,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GameParams {
    pub(crate) players: Option<usize>,
    pub(crate) bots: usize,
    pub(crate) timeout: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MatchInfo {
    pub(crate) players: usize,
    pub(crate) bots: usize,
    pub(crate) timeout: f64,
    pub(crate) args: HashMap<String, String>,
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) game: String,
    pub(crate) running: bool,
    pub(crate) time: u64,
    pub(crate) connected: Vec<String>,
    pub(crate) spectators: usize,
}

#[allow(dead_code)]
impl Request {
    pub(crate) fn forge(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(x) => Ok(x),
            Err(x) => Err(format!("Cannot forge request: {}", x)),
        }
    }
    pub(crate) fn parse(req: &str) -> Result<Request, String> {
        match serde_json::from_str(req) {
            Ok(x) => Ok(x),
            Err(x) => Err(format!("Cannot parse request: {}", x)),
        }
    }
}

#[allow(dead_code)]
impl Reply {
    pub(crate) fn forge(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(x) => Ok(x),
            Err(x) => Err(format!("Cannot forge request: {}", x)),
        }
    }
    pub(crate) fn parse(req: &str) -> Result<Reply, String> {
        match serde_json::from_str(req) {
            Ok(x) => Ok(x),
            Err(x) => Err(format!("Cannot parse request: {}", x)),
        }
    }
}

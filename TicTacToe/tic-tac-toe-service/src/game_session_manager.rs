use crate::game_session::GamingSession;
use crate::game_trait::GameTrait;
use crate::models::PlayerInfo;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Manages the Games Sessions.
pub(crate) struct GamingSessionsManager<T: GameTrait + Clone + Send + Sync + 'static> {
    sessions: Arc<Mutex<HashMap<String, Box<GamingSession<T>>>>>,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    /// Creates a new Gaming Session and returns the Invitation ID.
    pub(crate) fn create_session(&mut self, session_owner: PlayerInfo, broker_address: String, broker_port: u16) -> GamingSession<T> {
        let session = GamingSession::new(session_owner, broker_address, broker_port);
        self.upsert_session(&session);
        session
    }

    pub(crate) fn get_game_by_id(&self, game_id: &String) -> Option<T> {
        match self.get_session_containing_game(game_id) {
            None => None,
            Some(session) => session.current_game.clone(),
        }
    }

    pub(crate) fn get_game_by_invitation_code(&self, invitation_code: &String) -> Option<T> {
        match self.get_session(invitation_code) {
            None => None,
            Some(session) => {
                session.current_game
            }
        }
    }

    pub(crate) fn get_session(&self, invitation_code: &String) -> Option<Box<GamingSession<T>>> {
        self.sessions.lock().unwrap().get(invitation_code).cloned()
    }

    pub(crate) fn get_session_and_game_by_game_id(&self, game_id: &String) -> Option<(GamingSession<T>, T)> {
        match self.get_session_containing_game(game_id) {
            None => None,
            Some(session) => {
                session.current_game.as_ref().map(|game| ((*session).clone(), game.clone()))
            }
        }
    }

    fn get_session_containing_game(&self, game_id: &String) -> Option<Box<GamingSession<T>>> {
        for session in self.sessions.lock().unwrap().iter() {
            if let Some(game) = session.1.current_game.clone() {
                if game.get_id().as_str() == game_id {
                    return Some(session.1.clone());
                }
            }
        }
        None
    }

    /// Creates a new Gaming Session instance.
    pub(crate) fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn remove_game(&mut self, game_id: &String) -> bool {
        match self.get_session_containing_game(game_id) {
            None => false,
            Some(mut session) => {
                session.clear_game();
                self.upsert_session(&session);
                true
            }
        }
    }

    /// Removes a new Gaming Session.
    pub(crate) fn remove_session(&mut self, invitation_code: &String) {
        self.sessions.lock().unwrap().remove(invitation_code);
    }

    pub(crate) fn upsert_game(&self, game: &T) -> bool {
        match self.get_session_containing_game(&game.get_id()) {
            Some(mut session) => {
                session.set_game(game);
                true
            }
            None => false,
        }
    }

    /// Upserts a Gaming Session.
    fn upsert_session(&mut self, session: &GamingSession<T>) {
        self.sessions.lock().unwrap().insert(session.invitation_code.clone(), Box::new(session.clone()));
    }
}
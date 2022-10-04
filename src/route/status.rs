pub mod status {
    use crate::domain::state::switchbot::SwitchBotState;
    use rocket::{serde::json::Json, Route, State};
    use std::sync::{Arc, Mutex};

    #[get("/status")]
    fn status(state: &State<Arc<Mutex<SwitchBotState>>>) -> Json<SwitchBotState> {
        let s = state.lock().unwrap();
        Json(s.clone())
    }

    pub fn build() -> Vec<Route> {
        routes![status]
    }
}

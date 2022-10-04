pub mod status {
    use crate::domain::state::switchbot::SwitchBotState;
    use rocket::{serde::json::Json, Route, State};
    use std::sync::{Arc, Mutex};

    #[get("/")]
    fn get(state: &State<Arc<Mutex<SwitchBotState>>>) -> Json<SwitchBotState> {
        let state = state.lock().unwrap();
        Json(state.clone())
    }

    pub fn build() -> Vec<Route> {
        routes![get]
    }
}

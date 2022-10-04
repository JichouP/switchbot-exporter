pub mod exporter {
    use crate::{
        domain::state::switchbot::SwitchBotState, infrastructure::prometheus::SwitchBotMetrics,
    };
    use rocket::{Route, State};
    use std::sync::{Arc, Mutex};

    #[get("/")]
    fn get(state: &State<Arc<Mutex<SwitchBotState>>>) -> String {
        let state = state.lock().unwrap();
        let metrics = SwitchBotMetrics::from(&*state);
        metrics.0
    }

    pub fn build() -> Vec<Route> {
        routes![get]
    }
}

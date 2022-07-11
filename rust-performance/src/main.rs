mod app;
mod components;
mod model;
mod extensions;

use app::App;
use cfg_if::cfg_if;

// configure logging
cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    yew::start_app::<App>();
}

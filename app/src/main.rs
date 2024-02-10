#![allow(non_snake_case)]

use dioxus::prelude::*;
use dist::{Distribution, NoncentralT};

#[cfg(feature = "debug")]
use log::{info, LevelFilter};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    #[cfg(feature = "debug")]
    {
        // init logger for Dioxus
        dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    }
    // launch the web app
    #[cfg(feature = "debug")]
    info!("Launching PowerAnalyses app");
    cx.render(rsx! {
        div { "Hello, world!" }
        div { "{NoncentralT::new(0.5, 0.4).cdf(0.3, true)}" }
    })
}

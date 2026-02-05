use log::{warn};

mod app;
mod components;
mod content;
mod pages;
mod routes;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    warn!("Starting App");
    yew::Renderer::<app::App>::new().render();
}

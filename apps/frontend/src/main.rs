mod app;
mod components;
mod content;
mod pages;
mod routes;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

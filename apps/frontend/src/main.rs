mod app;
mod components;
mod content;
mod pages;
fn main() {
    yew::Renderer::<app::App>::new().render();
}

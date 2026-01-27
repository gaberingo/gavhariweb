use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Projects => html! { <ProjectsPage /> },
        Route::NotFound => html! { <NotFound/> },
    }
}

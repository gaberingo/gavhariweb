use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{Route, switch};

/// Root app with router.
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

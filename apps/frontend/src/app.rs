use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{Route, switch};
use log::{info};

/// Root app with router.
#[function_component]
pub fn App() -> Html {
    info!("App mounted");
    html! {
        <div class="mx-auto w-full max-w-[1440px] px-4 md:px-10">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

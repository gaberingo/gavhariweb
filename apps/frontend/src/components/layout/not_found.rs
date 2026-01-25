use yew::prelude::*;
use web_sys::window;
use yew_router::prelude::*;
use crate::routes::*;

#[function_component]
pub fn NotFound() -> Html {
    let on_retry = Callback::from(|_| {
        if let Some(win) = window() {
            let _ = win.location().reload();
        }
    });
    html! {
        <div class="relative min-h-screen overflow-hidden bg-[#181311] text-white">
            <div class="pointer-events-none absolute inset-0">
                <div class="absolute -left-24 top-16 h-48 w-48 rounded-full bg-[#6b3e2e]/40 blur-3xl"></div>
                <div class="absolute -right-24 bottom-24 h-56 w-56 rounded-full bg-[#2b4b5a]/40 blur-3xl"></div>
                <div class="absolute left-1/2 top-1/3 h-24 w-24 -translate-x-1/2 rounded-full border border-white/10"></div>
            </div>
            <div class="mx-auto flex min-h-screen max-w-[960px] flex-col items-center justify-center gap-6 px-4 text-center">
                <span class="text-xs font-semibold uppercase tracking-[0.45em] text-white/50">
                    {"Sinyal Hilang"}
                </span>
                <div class="relative">
                    <div class="absolute -top-6 left-1/2 -translate-x-1/2 text-[10px] tracking-[0.5em] text-white/30">
                        {"GAVHARI"}
                    </div>
                    <h1 class="text-[96px] font-black leading-none sm:text-[128px]">
                        {"404"}
                    </h1>
                    <div class="mx-auto mt-1 h-1 w-28 bg-white/70"></div>
                </div>
                <h2 class="text-2xl font-semibold sm:text-3xl">
                    {"Tersesat di jalur yang belum dibuat"}
                </h2>
                <p class="max-w-[520px] text-sm text-white/60">
                    {"Kami belum membangun halaman itu. Coba kembali ke beranda atau jelajahi lagi."}
                </p>
                <div class="flex flex-wrap items-center justify-center gap-3">
                    <Link<Route>
                        to={Route::Home}
                        classes="rounded-full bg-white px-5 py-2 text-sm font-semibold text-[#181311] transition hover:bg-white/90"
                    >
                        {"Kembali ke Beranda"}
                    </Link<Route>>
                    <button
                        onclick={on_retry}
                        class="rounded-full border border-white/20 px-5 py-2 text-sm font-semibold text-white/80 transition hover:border-white/40 hover:text-white"
                    >
                        {"Coba Lagi"}
                    </button>
                </div>
            </div>
        </div>
    }
}
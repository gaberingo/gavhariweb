use yew::prelude::*;

/// Navbar cyberpunk dengan menu utama, CTA, dan menu mobile.
#[function_component]
pub fn Navbar() -> Html {
    html! {
        <header
            class="sticky top-0 z-50 w-full border-b border-[#392c28] bg-[#181311]/80 backdrop-blur-md"
            data-doc="Navbar wrapper: branding + nav + actions."
        >
            <div class="mx-auto flex max-w-[1440px] items-center justify-between px-4 py-3 md:px-10">
                <a class="group flex items-center gap-3" href="#" data-doc="Branding block: logo + name.">
                    <div class="relative flex h-9 w-9 items-center justify-center">
                        <div class="absolute inset-0 rounded-[10px] bg-gradient-to-br from-[#ff7a18] via-[#ec4913] to-[#9e2a12] opacity-90 transition-transform duration-300 group-hover:rotate-6 group-hover:scale-105"></div>
                        <div class="absolute inset-[3px] rounded-[8px] border border-white/20 bg-[#1b1412] transition-all duration-300 group-hover:border-[#ffb08a]/60"></div>
                        <span class="relative text-[11px] font-extrabold uppercase tracking-[0.2em] text-white">{"GV"}</span>
                    </div>
                    <div class="flex flex-col leading-none">
                        <span class="text-[10px] font-semibold uppercase tracking-[0.25em] text-white/60 transition-colors group-hover:text-[#ffb08a]">{"Studio"}</span>
                        <h2 class="text-lg font-bold tracking-tight text-white transition-colors group-hover:text-[#ffb08a]">{"Gavhari"}</h2>
                    </div>
                </a>

                <div class="flex items-center gap-6">
                    <nav class="hidden items-center gap-8 md:flex" data-doc="Primary navigation links.">
                        <a class="text-sm font-medium text-white/70 transition-colors hover:text-[#ec4913]" href="/projects">{"Projects"}</a>
                        <a class="text-sm font-medium text-white/70 transition-colors hover:text-[#ec4913]" href="#">{"Writing"}</a>
                        <a class="text-sm font-medium text-white/70 transition-colors hover:text-[#ec4913]" href="#">{"About"}</a>
                    </nav>
                    <div class="flex items-center gap-3 pl-0 md:pl-4 md:border-l md:border-[#392c28]" data-doc="Actions: theme toggle, GitHub, mobile menu.">
                        <button class="hidden h-10 w-10 items-center justify-center rounded-lg border border-[#392c28] bg-[#271e1c] text-white transition-colors hover:border-[#ec4913]/50 md:flex">
                            <span class="text-xs font-semibold">{"LT"}</span>
                        </button>
                        <a class="flex h-10 items-center justify-center gap-2 rounded-lg border border-[#392c28] bg-[#271e1c] px-4 text-sm font-bold text-white transition-colors hover:bg-[#392c28]" href="https://github.com" target="_blank">
                            <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                <path d="M12 1.5c-5.8 0-10.5 4.7-10.5 10.5 0 4.6 3 8.5 7.2 9.9.5.1.7-.2.7-.5v-1.9c-3 .7-3.6-1.3-3.6-1.3-.5-1.3-1.2-1.6-1.2-1.6-1-.7.1-.7.1-.7 1.1.1 1.7 1.1 1.7 1.1 1 .1 1.6.7 1.9 1 .1-.7.4-1.1.7-1.4-2.4-.3-4.9-1.2-4.9-5.4 0-1.2.4-2.1 1.1-2.9-.1-.3-.5-1.4.1-2.9 0 0 .9-.3 2.9 1.1.8-.2 1.7-.3 2.6-.3.9 0 1.8.1 2.6.3 2-1.4 2.9-1.1 2.9-1.1.6 1.5.2 2.6.1 2.9.7.8 1.1 1.8 1.1 2.9 0 4.2-2.5 5.1-4.9 5.4.4.3.7.9.7 1.9v2.8c0 .3.2.6.7.5 4.2-1.4 7.2-5.3 7.2-9.9C22.5 6.2 17.8 1.5 12 1.5z"/>
                            </svg>
                            <span class="hidden sm:inline">{"GitHub"}</span>
                        </a>
                        <details class="relative md:hidden" data-doc="Mobile menu dropdown.">
                            <summary class="list-none text-white">
                                <span class="text-2xl">{"="}</span>
                            </summary>
                            <div class="absolute right-0 mt-2 flex w-52 flex-col gap-2 rounded-lg border border-[#392c28] bg-[#181311] p-3 text-sm text-white/70 shadow-[0_16px_40px_rgba(0,0,0,0.4)]">
                                <a class="transition hover:text-[#ec4913]" href="/projects">{"Projects"}</a>
                                <a class="transition hover:text-[#ec4913]" href="#">{"Writing"}</a>
                                <a class="transition hover:text-[#ec4913]" href="#">{"About"}</a>
                            </div>
                        </details>
                    </div>
                </div>
            </div>
        </header>
    }
}

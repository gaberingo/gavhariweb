use yew::prelude::*;

/// Value proposition dengan poin utama layanan.
#[function_component]
pub fn ValuePropsSection() -> Html {
    html! {
        <section class="relative overflow-hidden border-t border-[#392c28] bg-[#120e0c] py-20" data-doc="Value props section.">
            <div class="pointer-events-none absolute inset-0">
                <div class="absolute -left-20 top-10 h-64 w-64 rounded-full bg-[#ec4913]/10 blur-[110px]"></div>
                <div class="absolute right-6 top-1/2 h-72 w-72 -translate-y-1/2 rounded-full bg-[#38bdf8]/10 blur-[130px]"></div>
            </div>
            <div class="relative mx-auto flex max-w-[1280px] flex-col gap-12 px-4 md:flex-row md:gap-16 md:px-10">
            <div class="md:w-1/3 md:self-start md:sticky md:top-24">
                <div class="inline-flex items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.3em] text-white/60">
                    <span class="h-1.5 w-1.5 rounded-full bg-[#ec4913]"></span>
                    {"Correctness by design"}
                </div>
                <h2 class="mt-4 text-3xl font-bold leading-tight sm:text-4xl">{"Engineering for systems that stay reliable."}</h2>
                <p class="mt-4 text-base text-white/60">
                    {"Setiap modul fokus pada keamanan memori, konsistensi, dan performa. Layout ini siap untuk profil, promo, dan store."}
                </p>
                <div class="mt-6 flex items-center gap-3 text-xs text-white/50">
                    <span class="h-1 w-10 rounded-full bg-[#ec4913]"></span>
                    {"Stability, clarity, and velocity"}
                </div>
            </div>
            <div class="grid gap-4 md:w-2/3 sm:grid-cols-2">
                <div class="neon-border group relative overflow-hidden rounded-2xl border border-[#392c28] bg-[#17110f] p-6 transition hover:border-[#ec4913]/40">
                    <div class="absolute -right-12 top-6 h-20 w-20 rounded-full bg-[#ec4913]/10 blur-[45px]"></div>
                    <p class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"Systems"}</p>
                    <p class="mt-4 text-lg font-bold">{"Systems Mindset"}</p>
                    <p class="mt-3 text-sm text-white/60">{"Berangkat dari constraint low-level sampai observability."}</p>
                </div>
                <div class="neon-border group relative overflow-hidden rounded-2xl border border-[#392c28] bg-[#17110f] p-6 transition hover:border-[#ec4913]/40">
                    <div class="absolute -right-12 top-6 h-20 w-20 rounded-full bg-[#22c55e]/10 blur-[45px]"></div>
                    <p class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"DX"}</p>
                    <p class="mt-4 text-lg font-bold">{"DX-Friendly APIs"}</p>
                    <p class="mt-3 text-sm text-white/60">{"API ringkas, dokumentasi jelas, dan tooling yang rapi."}</p>
                </div>
                <div class="neon-border neon-border-cool group relative overflow-hidden rounded-2xl border border-[#392c28] bg-[#17110f] p-6 transition hover:border-[#ec4913]/40 sm:col-span-2">
                    <div class="absolute -right-12 top-6 h-20 w-20 rounded-full bg-[#38bdf8]/10 blur-[45px]"></div>
                    <p class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"Automation"}</p>
                    <p class="mt-4 text-lg font-bold">{"Reliable Automation"}</p>
                    <p class="mt-3 text-sm text-white/60">{"CI/CD stabil dengan guardrail performa dan quality gate."}</p>
                </div>
            </div>
            </div>
        </section>
    }
}

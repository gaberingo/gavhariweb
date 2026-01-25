use yew::prelude::*;

/// Grid proyek unggulan.
#[function_component]
pub fn FeaturedProjectsSection() -> Html {
    html! {
        <section class="relative overflow-hidden border-t border-[#392c28] bg-[#120e0c] py-20" data-doc="Featured projects grid.">
            <div class="pointer-events-none absolute inset-0">
                <div class="absolute -left-24 top-10 h-64 w-64 rounded-full bg-[#ec4913]/10 blur-[110px]"></div>
                <div class="absolute bottom-0 right-0 h-72 w-72 rounded-full bg-[#14b8a6]/10 blur-[130px]"></div>
            </div>
            <div class="relative mx-auto max-w-[1280px] px-4 md:px-10">
                <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
                    <div>
                        <div class="inline-flex items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.3em] text-white/60">
                            <span class="h-1.5 w-1.5 rounded-full bg-[#ec4913]"></span>
                            {"Featured Projects"}
                        </div>
                        <h2 class="mt-4 text-3xl font-bold sm:text-4xl">{"Highlighted builds"}</h2>
                        <p class="mt-3 max-w-xl text-sm text-white/60">
                            {"Eksperimen produk yang fokus pada low-latency, UX, dan reliability untuk aplikasi modern."}
                        </p>
                    </div>
                    <button class="hidden items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-4 py-2 text-sm font-medium text-white/70 transition hover:border-[#ec4913]/60 hover:text-white md:flex">
                        {"View all repositories"}
                    </button>
                </div>
                <div class="mt-12 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                <article class="group relative flex flex-col overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] transition-all duration-300 hover:-translate-y-1 hover:border-[#ec4913]/50">
                    <div class="absolute -right-16 top-6 h-24 w-24 rounded-full bg-[#ec4913]/10 blur-[45px]"></div>
                    <div class="h-48 w-full bg-[linear-gradient(130deg,_#1c232a,_#0f141c)]">
                        <div class="flex h-full items-end justify-between p-6 text-xs uppercase tracking-[0.3em] text-white/50">
                            <span>{"Realtime"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[10px]">{"Fintech"}</span>
                        </div>
                    </div>
                    <div class="flex flex-1 flex-col p-6">
                        <div class="flex items-start justify-between">
                            <h3 class="text-xl font-bold text-white transition group-hover:text-[#ec4913]">{"High-Freq Trading Bot"}</h3>
                            <span class="text-white/40">{"↗"}</span>
                        </div>
                        <p class="mt-3 text-sm text-white/60">
                            {"Latency rendah dan desain arsitektur deterministic untuk market data."}
                        </p>
                        <div class="mt-4 flex flex-wrap gap-2 text-xs font-mono">
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-[#ec4913]">{"Rust"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"Tokio"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"WebSocket"}</span>
                        </div>
                    </div>
                </article>
                <article class="group relative flex flex-col overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] transition-all duration-300 hover:-translate-y-1 hover:border-[#ec4913]/50">
                    <div class="absolute -right-16 top-6 h-24 w-24 rounded-full bg-[#22c55e]/10 blur-[45px]"></div>
                    <div class="h-48 w-full bg-[linear-gradient(130deg,_#111827,_#0b0f1f)]">
                        <div class="flex h-full items-end justify-between p-6 text-xs uppercase tracking-[0.3em] text-white/50">
                            <span>{"Visual"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[10px]">{"Media"}</span>
                        </div>
                    </div>
                    <div class="flex flex-1 flex-col p-6">
                        <div class="flex items-start justify-between">
                            <h3 class="text-xl font-bold text-white transition group-hover:text-[#ec4913]">{"WASM Image Processor"}</h3>
                            <span class="text-white/40">{"↗"}</span>
                        </div>
                        <p class="mt-3 text-sm text-white/60">
                            {"Pipeline WebAssembly untuk komputasi foto real-time."}
                        </p>
                        <div class="mt-4 flex flex-wrap gap-2 text-xs font-mono">
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-[#ec4913]">{"Yew"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"WASM"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"Canvas API"}</span>
                        </div>
                    </div>
                </article>
                <article class="group relative flex flex-col overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] transition-all duration-300 hover:-translate-y-1 hover:border-[#ec4913]/50">
                    <div class="absolute -right-16 top-6 h-24 w-24 rounded-full bg-[#38bdf8]/10 blur-[45px]"></div>
                    <div class="h-48 w-full bg-[linear-gradient(130deg,_#0b2d2f,_#07191a)]">
                        <div class="flex h-full items-end justify-between p-6 text-xs uppercase tracking-[0.3em] text-white/50">
                            <span>{"Infra"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[10px]">{"Storage"}</span>
                        </div>
                    </div>
                    <div class="flex flex-1 flex-col p-6">
                        <div class="flex items-start justify-between">
                            <h3 class="text-xl font-bold text-white transition group-hover:text-[#ec4913]">{"Distributed KV Store"}</h3>
                            <span class="text-white/40">{"↗"}</span>
                        </div>
                        <p class="mt-3 text-sm text-white/60">
                            {"Consensus layer untuk penyimpanan konsisten multi-region."}
                        </p>
                        <div class="mt-4 flex flex-wrap gap-2 text-xs font-mono">
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-[#ec4913]">{"Rust"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"gRPC"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-white/70">{"Raft"}</span>
                        </div>
                    </div>
                </article>
                </div>
                <div class="mt-8 text-center md:hidden">
                    <button class="rounded-full border border-[#392c28] bg-[#1c1513] px-4 py-2 text-sm font-medium text-white/70 transition hover:border-[#ec4913]/60 hover:text-white">
                        {"View all repositories"}
                    </button>
                </div>
            </div>
        </section>
    }
}

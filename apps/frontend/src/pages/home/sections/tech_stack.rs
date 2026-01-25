use yew::prelude::*;

/// Daftar stack teknologi dalam bentuk tag.
#[function_component]
pub fn TechStackSection() -> Html {
    html! {
        <section class="relative overflow-hidden border-t border-[#392c28] bg-[#120e0c] py-16" data-doc="Tech stack tags.">
            <div class="pointer-events-none absolute inset-0">
                <div class="absolute -left-32 top-10 h-64 w-64 rounded-full bg-[#ec4913]/10 blur-[120px]"></div>
                <div class="absolute right-0 top-24 h-80 w-80 rounded-full bg-[#3b82f6]/10 blur-[140px]"></div>
            </div>
            <div class="relative mx-auto max-w-[1280px] px-4 md:px-10">
                <div class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
                    <div>
                        <div class="inline-flex items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.3em] text-white/60">
                            <span class="h-1.5 w-1.5 rounded-full bg-[#ec4913]"></span>
                            {"Tooling"}
                        </div>
                        <h2 class="mt-4 text-3xl font-bold sm:text-4xl">{"Technology Stack"}</h2>
                        <p class="mt-3 max-w-xl text-sm text-white/60">
                            {"Build cepat, rapi, dan siap scale dengan kombinasi bahasa, framework, dan infrastruktur yang solid."}
                        </p>
                    </div>
                    <div class="flex items-center gap-3 text-xs text-white/50">
                        <span class="h-1 w-8 rounded-full bg-[#ec4913]"></span>
                        {"Sorted by domain"}
                    </div>
                </div>
                <div class="mt-10 grid gap-6 md:grid-cols-2">
                    <div class="group relative overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] p-6">
                        <div class="absolute -right-10 top-6 h-16 w-16 rounded-full bg-[#ec4913]/10 blur-[40px]"></div>
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"Languages"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[11px] text-white/60">{"Core"}</span>
                        </div>
                        <div class="mt-6 flex flex-wrap gap-3">
                            <span class="flex items-center gap-2 rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">
                                <span class="h-2 w-2 rounded-full bg-[#ec4913]"></span>
                                {"Rust"}
                            </span>
                            <span class="flex items-center gap-2 rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">
                                <span class="h-2 w-2 rounded-full bg-[#3b82f6]"></span>
                                {"TypeScript"}
                            </span>
                            <span class="flex items-center gap-2 rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">
                                <span class="h-2 w-2 rounded-full bg-[#eab308]"></span>
                                {"SQL"}
                            </span>
                        </div>
                    </div>
                    <div class="group relative overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] p-6">
                        <div class="absolute -right-12 top-8 h-16 w-16 rounded-full bg-[#22c55e]/10 blur-[40px]"></div>
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"Frontend"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[11px] text-white/60">{"Experience"}</span>
                        </div>
                        <div class="mt-6 flex flex-wrap gap-3">
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Yew"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"React"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"WebAssembly"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Tailwind CSS"}</span>
                        </div>
                    </div>
                    <div class="group relative overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] p-6">
                        <div class="absolute -right-10 top-8 h-16 w-16 rounded-full bg-[#f97316]/10 blur-[40px]"></div>
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"Backend"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[11px] text-white/60">{"Systems"}</span>
                        </div>
                        <div class="mt-6 flex flex-wrap gap-3">
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Axum"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Tokio"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"PostgreSQL"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Redis"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"gRPC"}</span>
                        </div>
                    </div>
                    <div class="group relative overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] p-6">
                        <div class="absolute -right-10 top-8 h-16 w-16 rounded-full bg-[#38bdf8]/10 blur-[40px]"></div>
                        <div class="flex items-center justify-between">
                            <span class="text-xs font-semibold uppercase tracking-[0.3em] text-white/50">{"DevOps"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[11px] text-white/60">{"Delivery"}</span>
                        </div>
                        <div class="mt-6 flex flex-wrap gap-3">
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"Docker"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"GitHub Actions"}</span>
                            <span class="rounded-full border border-[#392c28] bg-[#271e1c] px-4 py-2 text-sm font-medium">{"AWS"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

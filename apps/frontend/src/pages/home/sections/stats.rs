use yew::prelude::*;

/// Statistik ringkas dengan highlight metrik.
#[function_component]
pub fn StatsSection() -> Html {
    html! {
        <section class="border-y border-[#392c28] bg-[#120e0c] py-10" data-doc="Stats strip.">
            <div class="mx-auto grid max-w-[1280px] grid-cols-2 gap-6 px-4 md:grid-cols-4 md:px-10">
                <div class="flex flex-col gap-1 rounded-lg border border-transparent p-4 transition hover:border-[#392c28] hover:bg-[#271e1c]">
                    <span class="text-3xl font-bold tracking-tight text-white md:text-4xl">{"40%"}</span>
                    <span class="font-mono text-xs uppercase tracking-[0.22em] text-white/50">{"Latency Cut"}</span>
                </div>
                <div class="flex flex-col gap-1 rounded-lg border border-transparent p-4 transition hover:border-[#392c28] hover:bg-[#271e1c]">
                    <span class="text-3xl font-bold tracking-tight text-white md:text-4xl">{"100%"}</span>
                    <span class="font-mono text-xs uppercase tracking-[0.22em] text-white/50">{"Memory Safety"}</span>
                </div>
                <div class="flex flex-col gap-1 rounded-lg border border-transparent p-4 transition hover:border-[#392c28] hover:bg-[#271e1c]">
                    <span class="text-3xl font-bold tracking-tight text-white md:text-4xl">{"5+"}</span>
                    <span class="font-mono text-xs uppercase tracking-[0.22em] text-white/50">{"OSS Crates"}</span>
                </div>
                <div class="flex flex-col gap-1 rounded-lg border border-transparent p-4 transition hover:border-[#392c28] hover:bg-[#271e1c]">
                    <span class="text-3xl font-bold tracking-tight text-white md:text-4xl">{"99.9%"}</span>
                    <span class="font-mono text-xs uppercase tracking-[0.22em] text-white/50">{"Uptime Delivered"}</span>
                </div>
            </div>
        </section>
    }
}

use yew::prelude::*;
use crate::content::StatItem;

#[derive(Properties, PartialEq)]
pub struct StatsProps {
    pub data: Vec<StatItem>
}
/// Statistik ringkas dengan highlight metrik.
#[function_component]
pub fn StatsSection(props: &StatsProps) -> Html {
    let stats = &props.data;
    html! {
        <section class="border-y border-[#392c28] bg-[#120e0c] py-10" data-doc="Stats strip.">
            <div class="mx-auto grid max-w-[1440px] grid-cols-2 gap-6 px-4 md:grid-cols-4 md:px-10">
                {for stats.iter().map(|stat| {
                    html!{
                        <div class="flex flex-col gap-1 rounded-lg border border-transparent p-4 transition hover:border-[#392c28] hover:bg-[#271e1c]">
                            <span class="text-3xl font-bold tracking-tight text-white md:text-4xl">{stat.value.clone()}</span>
                            <span class="font-mono text-xs uppercase tracking-[0.22em] text-white/50">{stat.label.clone()}</span>
                        </div>
                    }
                })}
            </div>
        </section>
    }
}

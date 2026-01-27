use yew::prelude::*;

use crate::content::ProjectsFilters;

#[derive(Properties, PartialEq)]
pub struct ProjectsFilterBarProps {
    pub data: ProjectsFilters,
}

#[function_component]
pub fn ProjectsFilterBar(props: &ProjectsFilterBarProps) -> Html {
    let data = &props.data;
    html! {
        <section class="sticky top-[72px] z-40 -mx-4 px-4 md:-mx-10 md:px-10" data-doc="Projects filter bar.">
            <div class="flex flex-col gap-4 rounded-xl border border-[#392c28] bg-[#221c18]/90 p-3 shadow-lg backdrop-blur-md md:flex-row md:items-center md:justify-between">
                <div class="flex flex-wrap items-center gap-2">
                    <span class="hidden text-xs font-mono uppercase tracking-wider text-white/50 md:block">{"Category:"}</span>
                    {for data.categories.iter().map(|label| {
                        let is_active = label == "All";
                        let class = if is_active {
                            "rounded-lg bg-[#ec4913] px-4 py-1.5 text-sm font-medium text-white"
                        } else {
                            "rounded-lg px-4 py-1.5 text-sm font-medium text-white/60 transition hover:bg-[#392c28] hover:text-white"
                        };
                        html! { <button class={class}>{label.clone()}</button> }
                    })}
                </div>
                <div class="hidden h-px w-full bg-[#392c28] md:block md:h-6 md:w-px"></div>
                <div class="flex w-full flex-wrap items-center gap-2 md:w-auto">
                    <span class="hidden text-xs font-mono uppercase tracking-wider text-white/50 md:block">{"Stack:"}</span>
                    <div class="flex flex-wrap gap-2">
                        {for data.stack.iter().map(|label| {
                            html! {
                                <span class="inline-flex items-center rounded border border-[#392c28] bg-[#181311] px-3 py-1 text-xs font-mono text-white/70">
                                    {label.clone()}
                                </span>
                            }
                        })}
                    </div>
                </div>
                <div class="ml-auto flex items-center">
                    <button class="flex items-center gap-2 rounded border border-[#392c28] bg-[#181311] px-3 py-1.5 text-sm font-mono text-white/60 transition hover:border-[#ec4913] hover:text-white">
                        <span>{format!("Year: {}", data.years.first().cloned().unwrap_or_else(|| "All".to_string()))}</span>
                        <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 0 1 1.06.02L10 10.94l3.71-3.71a.75.75 0 1 1 1.06 1.06l-4.24 4.24a.75.75 0 0 1-1.06 0L5.21 8.29a.75.75 0 0 1 .02-1.08z" clip-rule="evenodd" />
                        </svg>
                    </button>
                </div>
            </div>
        </section>
    }
}

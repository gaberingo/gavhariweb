use yew::prelude::*;

use crate::content::ProjectCard;

#[derive(Properties, PartialEq)]
pub struct ProjectsGridProps {
    pub cards: Vec<ProjectCard>,
}

fn card_icon(name: &str) -> Html {
    let icon_class = "h-12 w-12 text-[#392c28] transition-colors group-hover:text-[#ec4913]/60";
    match name {
        "monitor" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <rect x="3.5" y="4.5" width="17" height="12" rx="2"></rect>
                <path d="M8 20h8M12 16.5v3.5"></path>
            </svg>
        },
        "dns" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <ellipse cx="12" cy="6" rx="7" ry="3.5"></ellipse>
                <path d="M5 6v6c0 2 3.1 3.5 7 3.5s7-1.5 7-3.5V6"></path>
                <path d="M5 12v6c0 2 3.1 3.5 7 3.5s7-1.5 7-3.5v-6"></path>
            </svg>
        },
        "image" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <rect x="4" y="5" width="16" height="14" rx="2"></rect>
                <path d="m8 13 2.5-2.5 3 3 2-2 2.5 2.5"></path>
                <circle cx="9" cy="9" r="1.5"></circle>
            </svg>
        },
        "terminal" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <rect x="3.5" y="5" width="17" height="14" rx="2"></rect>
                <path d="m7.5 10 2 2-2 2"></path>
                <path d="M11.5 14h4"></path>
            </svg>
        },
        "hub" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <circle cx="12" cy="12" r="2.5"></circle>
                <circle cx="5" cy="7" r="2"></circle>
                <circle cx="19" cy="7" r="2"></circle>
                <circle cx="5" cy="17" r="2"></circle>
                <circle cx="19" cy="17" r="2"></circle>
                <path d="M7 8.5 10 11M17 8.5 14 11M7 15.5 10 13M17 15.5 14 13"></path>
            </svg>
        },
        "memory" => html! {
            <svg class={icon_class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <rect x="6" y="6" width="12" height="12" rx="2"></rect>
                <path d="M9 6V4M12 6V4M15 6V4M9 20v-2M12 20v-2M15 20v-2M6 9H4M6 12H4M6 15H4M20 9h-2M20 12h-2M20 15h-2"></path>
            </svg>
        },
        _ => html! { <span class="text-3xl text-[#392c28]">{"[]"}</span> },
    }
}

#[function_component]
pub fn ProjectsGrid(props: &ProjectsGridProps) -> Html {
    html! {
        <section class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3" data-doc="Projects grid.">
            {for props.cards.iter().map(|card| {
                let badge = card.badge.clone();
                html! {
                    <article class="group relative flex h-full flex-col overflow-hidden rounded-xl border border-[#392c28] bg-[#221c18] transition-all duration-300 hover:-translate-y-1 hover:border-[#ec4913]/50 hover:shadow-xl hover:shadow-[#ec4913]/10">
                        <div class="relative flex aspect-video w-full items-center justify-center overflow-hidden bg-gradient-to-br from-[#2a241f] to-[#181411]">
                            <div class="absolute inset-0 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                                <div class="absolute -right-10 -top-8 h-40 w-40 rounded-full bg-[#ec4913]/10 blur-[80px]"></div>
                            </div>
                            {card_icon(&card.icon)}
                            {badge.map(|label| {
                                html! {
                                    <div class="absolute right-3 top-3 rounded border border-white/10 bg-black/50 px-2 py-1 text-[10px] font-mono uppercase tracking-wider text-white/60">
                                        {label}
                                    </div>
                                }
                            })}
                        </div>
                        <div class="flex flex-1 flex-col gap-3 p-5">
                            <div class="flex items-start justify-between gap-4">
                                <h3 class="text-xl font-bold text-white transition-colors group-hover:text-[#ec4913]">{card.title.clone()}</h3>
                                <svg class="mt-1 h-5 w-5 text-white/40 transition-all group-hover:translate-x-1 group-hover:text-white" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path fill-rule="evenodd" d="M3 10a.75.75 0 0 1 .75-.75h10.69l-3.22-3.22a.75.75 0 1 1 1.06-1.06l4.5 4.5a.75.75 0 0 1 0 1.06l-4.5 4.5a.75.75 0 1 1-1.06-1.06l3.22-3.22H3.75A.75.75 0 0 1 3 10z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <p class="text-sm leading-relaxed text-white/60">{card.description.clone()}</p>
                            <div class="mt-auto flex flex-wrap gap-2">
                                {for card.tags.iter().enumerate().map(|(idx, tag)| {
                                    let class = if idx == 0 {
                                        "rounded border border-[#ec4913]/30 bg-[#181311] px-2 py-1 text-xs font-mono text-[#ec4913]"
                                    } else {
                                        "rounded border border-[#392c28] bg-[#181311] px-2 py-1 text-xs font-mono text-white/60"
                                    };
                                    html! { <span class={class}>{tag.clone()}</span> }
                                })}
                            </div>
                        </div>
                    </article>
                }
            })}
        </section>
    }
}

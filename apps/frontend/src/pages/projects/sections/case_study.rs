use yew::prelude::*;

use crate::content::CaseStudyContent;

#[derive(Properties, PartialEq)]
pub struct CaseStudySectionProps {
    pub data: CaseStudyContent,
}

fn stat_class(accent: &Option<String>) -> &'static str {
    match accent.as_deref() {
        Some("primary") => "text-[#ec4913]",
        Some("green") => "text-[#4caf50]",
        _ => "text-white",
    }
}

#[function_component]
pub fn CaseStudySection(props: &CaseStudySectionProps) -> Html {
    let data = &props.data;
    html! {
        <section class="mt-10" data-doc="Projects case study highlight.">
            <div class="mb-8 flex items-center gap-4">
                <div class="h-px flex-1 bg-[#392c28]"></div>
                <span class="text-sm font-mono uppercase tracking-widest text-white/70">{"Case Study Highlight"}</span>
                <div class="h-px flex-1 bg-[#392c28]"></div>
            </div>
            <div class="flex flex-col overflow-hidden rounded-2xl border border-[#392c28] bg-[#221c18] lg:flex-row">
                <div class="flex flex-1 flex-col gap-8 p-8 lg:p-12">
                    <div>
                        <div class="mb-4 flex flex-wrap gap-3">
                            {for data.badges.iter().enumerate().map(|(idx, label)| {
                                let class = if idx == 0 {
                                    "rounded-full bg-[#ec4913]/20 px-3 py-1 text-xs font-bold uppercase tracking-wider text-[#ec4913]"
                                } else {
                                    "rounded-full bg-[#392c28] px-3 py-1 text-xs font-bold uppercase tracking-wider text-white/70"
                                };
                                html! { <span class={class}>{label.clone()}</span> }
                            })}
                        </div>
                        <h2 class="mb-4 text-3xl font-bold leading-tight text-white lg:text-4xl">{data.title.clone()}</h2>
                        <p class="text-lg text-white/60">{data.description.clone()}</p>
                    </div>
                    <div class="grid grid-cols-2 gap-6 border-t border-[#392c28] pt-6">
                        {for data.stats.iter().map(|stat| {
                            let accent_class = stat_class(&stat.accent);
                            html! {
                                <div>
                                    <p class={classes!("text-3xl", "font-mono", "font-bold", accent_class)}>{stat.value.clone()}</p>
                                    <p class="text-xs uppercase tracking-wide text-white/50">{stat.label.clone()}</p>
                                </div>
                            }
                        })}
                    </div>
                    <button class="group flex w-fit items-center gap-2 font-bold text-white">
                        <span>{"Read Full Story"}</span>
                        <svg class="h-5 w-5 transition-transform group-hover:translate-x-1" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M3 10a.75.75 0 0 1 .75-.75h10.69l-3.22-3.22a.75.75 0 1 1 1.06-1.06l4.5 4.5a.75.75 0 0 1 0 1.06l-4.5 4.5a.75.75 0 1 1-1.06-1.06l3.22-3.22H3.75A.75.75 0 0 1 3 10z" clip-rule="evenodd" />
                        </svg>
                    </button>
                </div>
                <div class="relative min-h-[280px] border-t border-[#392c28] bg-[#181311] lg:min-h-full lg:w-1/2 lg:border-l lg:border-t-0">
                    <div class="absolute inset-0 bg-gradient-to-br from-[#221c18] to-black opacity-80"></div>
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="w-4/5 max-w-sm rounded-lg border border-[#392c28] bg-[#181311] p-6 shadow-2xl">
                            <div class="mb-4 flex gap-2">
                                <div class="h-3 w-3 rounded-full bg-[#f87171]"></div>
                                <div class="h-3 w-3 rounded-full bg-[#facc15]"></div>
                                <div class="h-3 w-3 rounded-full bg-[#4ade80]"></div>
                            </div>
                            <div class="space-y-2">
                                <div class="h-2 w-3/4 rounded bg-[#392c28]"></div>
                                <div class="h-2 w-full rounded bg-[#392c28]"></div>
                                <div class="h-2 w-5/6 rounded bg-[#392c28]"></div>
                                <div class="mt-4 flex gap-2">
                                    <div class="relative h-20 w-full overflow-hidden rounded border border-white/5 bg-[#1e1e1e]">
                                        <div class="absolute bottom-0 left-0 h-[40%] w-full bg-[#ec4913]/20"></div>
                                        <div class="absolute bottom-0 left-0 h-[20%] w-[10%] bg-[#ec4913]"></div>
                                        <div class="absolute bottom-0 left-[15%] h-[50%] w-[10%] bg-[#ec4913]"></div>
                                        <div class="absolute bottom-0 left-[30%] h-[35%] w-[10%] bg-[#ec4913]"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

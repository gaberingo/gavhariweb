use yew::prelude::*;

use crate::content::ProjectsFooterCta;

#[derive(Properties, PartialEq)]
pub struct ProjectsFooterCtaProps {
    pub data: ProjectsFooterCta,
}

#[function_component]
pub fn ProjectsFooterCtaSection(props: &ProjectsFooterCtaProps) -> Html {
    let data = &props.data;
    html! {
        <footer class="mt-14 border-t border-[#392c28] bg-[#221c18] py-16" data-doc="Projects footer CTA.">
            <div class="mx-auto flex max-w-[960px] flex-col items-center gap-6 px-4 text-center">
                <div class="flex h-12 w-12 items-center justify-center rounded-full bg-[#392c28]">
                    <svg class="h-6 w-6 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                        <path d="M9 12h6"></path>
                        <path d="M9 16h6"></path>
                        <path d="M8 4h8a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z"></path>
                    </svg>
                </div>
                <h2 class="text-3xl font-bold text-white md:text-4xl">{data.title.clone()}</h2>
                <p class="max-w-lg text-white/60">{data.description.clone()}</p>
                <div class="mt-4 flex w-full flex-col gap-4 sm:w-auto sm:flex-row">
                    <button class="h-12 w-full rounded-lg bg-[#ec4913] px-8 font-bold text-white transition hover:bg-[#c53b0e] sm:w-auto">
                        {data.primary_label.clone()}
                    </button>
                    <button class="flex h-12 w-full items-center justify-center gap-2 rounded-lg border border-[#392c28] bg-transparent px-8 font-medium text-white transition hover:border-white sm:w-auto">
                        <span>{data.secondary_label.clone()}</span>
                        <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path d="M12.5 3.5a.75.75 0 0 1 .75-.75h3.5a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0V5.31l-7.72 7.72a.75.75 0 1 1-1.06-1.06l7.72-7.72H13.25a.75.75 0 0 1-.75-.75z"></path>
                            <path d="M5 5.75A2.75 2.75 0 0 1 7.75 3h2.5a.75.75 0 0 1 0 1.5h-2.5A1.25 1.25 0 0 0 6.5 5.75v6.5A1.25 1.25 0 0 0 7.75 13.5h6.5A1.25 1.25 0 0 0 15.5 12.25v-2.5a.75.75 0 0 1 1.5 0v2.5A2.75 2.75 0 0 1 14.25 15h-6.5A2.75 2.75 0 0 1 5 12.25v-6.5z"></path>
                        </svg>
                    </button>
                </div>
                <p class="mt-10 text-sm font-mono text-white/40">{data.footer_note.clone()}</p>
            </div>
        </footer>
    }
}

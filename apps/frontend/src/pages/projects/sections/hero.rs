use yew::prelude::*;

use crate::content::ProjectsHero;

#[derive(Properties, PartialEq)]
pub struct ProjectsHeroProps {
    pub data: ProjectsHero,
}

#[function_component]
pub fn ProjectsHeroSection(props: &ProjectsHeroProps) -> Html {
    let data = &props.data;
    html! {
        <section class="flex flex-col gap-4 py-6" data-doc="Projects hero.">
            <h1 class="text-4xl font-black leading-tight tracking-[-0.03em] text-white sm:text-5xl md:text-6xl">
                {data.title.clone()}{" "}
                <span class="text-[#ec4913]">{data.highlight.clone()}</span>
            </h1>
            <p class="max-w-2xl text-base text-white/70 sm:text-lg">
                {data.subtitle.clone()}
            </p>
        </section>
    }
}

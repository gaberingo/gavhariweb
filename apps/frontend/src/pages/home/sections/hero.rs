use yew::prelude::*;

use crate::content::HeroContent;

/// Hero utama: headline, CTA, dan kartu kode.
#[derive(Properties, PartialEq)]
pub struct HeroSectionProps {
    pub data: HeroContent,
}

#[function_component]
pub fn HeroSection(props: &HeroSectionProps) -> Html {
    let data = &props.data;
    let code_snippet = Html::from_html_unchecked(AttrValue::from(data.code_snippet.clone()));
    html! {
        <section class="relative py-16 md:py-24" data-doc="Hero section: headline + code card.">
            <div class="pointer-events-none absolute right-0 top-0 hidden h-[520px] w-[520px] -translate-y-1/3 translate-x-1/4 rounded-full bg-[#ec4913]/10 blur-[120px] md:block"></div>
            <div class="mx-auto grid max-w-[1440px] gap-10 px-4 md:px-10 lg:grid-cols-[1.1fr_0.9fr] lg:items-center lg:gap-16">
                <div class="min-w-0">
                    <p class="text-xs font-semibold uppercase tracking-[0.2em] text-white/60">{data.kicker.clone()}</p>
                    <h1 class="mt-4 text-4xl font-bold leading-tight sm:text-5xl lg:text-6xl">
                        {data.title.clone()}
                        {" "}
                        <span class="text-[#ec4913]">{data.highlight.clone()}</span>
                        {" "}
                        {data.suffix.clone()}
                    </h1>
                    <p class="mt-6 max-w-xl text-base font-light text-white/70 sm:text-lg">{data.subtitle.clone()}</p>
                    <div class="mt-8 flex flex-wrap gap-4">
                        <a
                            href={data.primary_cta.url.clone()}
                            class="w-full rounded-lg bg-[#ec4913] px-6 py-3 text-center text-base font-bold text-white shadow-[0_18px_40px_rgba(236,73,19,0.35)] transition hover:bg-[#c53b0e] sm:w-auto"
                        >
                            {data.primary_cta.label.clone()}
                        </a>
                        <a
                            href={data.secondary_cta.url.clone()}
                            class="neon-border w-full rounded-lg border border-[#392c28] bg-[#271e1c] px-6 py-3 text-center text-base font-bold text-white/90 transition hover:bg-[#392c28] sm:w-auto"
                        >
                            {data.secondary_cta.label.clone()}
                        </a>
                    </div>
                </div>
                <div class="neon-border min-w-0 w-full rounded-xl border border-[#392c28] bg-[#0d0d0d] shadow-[0_24px_60px_rgba(0,0,0,0.5)]">
                    <div class="flex items-center justify-between border-b border-[#392c28] bg-[#1a1a1a] px-4 py-3 text-xs text-white/50">
                        <span class="h-2 w-2 rounded-full bg-[#f87171]"></span>
                        <span class="h-2 w-2 rounded-full bg-[#facc15]"></span>
                        <span class="h-2 w-2 rounded-full bg-[#4ade80]"></span>
                        <span class="ml-auto font-mono text-xs uppercase tracking-[0.2em]">{"src/main.rs"}</span>
                    </div>
                    <pre class="custom-scrollbar overflow-x-auto px-4 py-4 font-mono text-sm leading-relaxed sm:px-6 sm:py-5 sm:text-base">
                        <code class="code-snippet">{code_snippet}</code>
                    </pre>
                </div>
            </div>
        </section>
    }
}

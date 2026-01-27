use yew::prelude::*;

use crate::content::CtaContent;

/// CTA penutup untuk mengajak kontak.
#[derive(Properties, PartialEq)]
pub struct CtaSectionProps {
    pub data: CtaContent,
}

#[function_component]
pub fn CtaSection(props: &CtaSectionProps) -> Html {
    let data = &props.data;
    html! {
        <section class="relative mt-16 overflow-hidden border-t border-[#392c28] bg-[#ec4913] py-20" data-doc="CTA banner.">
            <div class="absolute inset-0 opacity-10 [background-image:radial-gradient(#000_1px,transparent_1px)] [background-size:20px_20px]"></div>
            <div class="relative mx-auto flex max-w-[960px] flex-col items-center gap-6 px-4 text-center text-white sm:px-6">
                <h2 class="text-3xl font-bold sm:text-5xl">{data.title.clone()}</h2>
                <p class="max-w-2xl text-lg text-white/80">{data.subtitle.clone()}</p>
                <div class="mt-2 flex w-full flex-wrap justify-center gap-4">
                    <button class="w-full rounded-lg bg-[#181311] px-8 py-4 text-lg font-bold text-white shadow-xl transition hover:bg-black sm:w-auto">
                        {data.primary_cta.clone()}
                    </button>
                    <button class="w-full rounded-lg border border-white/30 bg-white/20 px-8 py-4 text-lg font-bold text-white backdrop-blur transition hover:bg-white/30 sm:w-auto">
                        {data.secondary_cta.clone()}
                    </button>
                </div>
            </div>
        </section>
    }
}

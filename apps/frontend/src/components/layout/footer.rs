use yew::prelude::*;

use crate::content::FooterContent;

/// Footer ringkas dengan info identitas.
#[derive(Properties, PartialEq)]
pub struct SiteFooterProps {
    pub data: FooterContent,
}

#[function_component]
pub fn SiteFooter(props: &SiteFooterProps) -> Html {
    let data = &props.data;
    html! {
        <footer class="border-t border-[#392c28] bg-[#181311] py-12" data-doc="Footer info.">
            <div class="mx-auto flex max-w-[1280px] flex-col items-center justify-between gap-6 px-4 text-sm text-white/60 md:flex-row md:px-10">
                <div class="text-center md:text-left">
                    <p class="text-lg font-bold text-white">{data.brand.clone()}</p>
                    <p class="mt-2 text-sm text-white/50">{data.copyright.clone()}</p>
                </div>
                <div class="flex items-center gap-6 text-white/60">
                    {for data.links.iter().map(|label| html! { <span>{label.clone()}</span> })}
                </div>
                <div class="text-center md:text-right">
                    <p class="text-sm text-white/50">{data.timezone.clone()}</p>
                    <p class="text-xs font-mono text-white/40">{data.local_time.clone()}</p>
                </div>
            </div>
        </footer>
    }
}

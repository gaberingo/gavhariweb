use yew::prelude::*;

use crate::components::{CtaSection, HeroSection, Navbar, SiteFooter};
use crate::content::load_site_content;
use crate::pages::home::sections::{
    FeaturedProjectsSection, StatsSection, TechStackSection, ValuePropsSection,
};

/// Home page layout (static for now).
#[function_component]
pub fn HomePage() -> Html {
    let content = load_site_content();
    html! {
        <div class="min-h-screen bg-[#181311] text-white" data-doc="App shell: global theme.">
            <Navbar />
            <main class="pb-16" data-doc="Main content wrapper.">
                <HeroSection data={content.hero.clone()} />
                <StatsSection />
                <ValuePropsSection />
                <TechStackSection />
                <FeaturedProjectsSection />
            </main>
            <CtaSection data={content.cta.clone()} />
            <SiteFooter data={content.footer.clone()} />
        </div>
    }
}

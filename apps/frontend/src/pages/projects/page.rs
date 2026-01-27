use yew::prelude::*;

use crate::components::Navbar;
use crate::content::load_site_content;
use crate::pages::projects::sections::{
    CaseStudySection, ProjectsFilterBar, ProjectsFooterCtaSection, ProjectsGrid,
    ProjectsHeroSection,
};

#[function_component]
pub fn ProjectsPage() -> Html {
    let content = load_site_content();
    let projects = content.projects;
    html! {
        <div class="min-h-screen bg-[#181311] text-white" data-doc="Projects page shell.">
            <Navbar />
            <main class="mx-auto flex w-full max-w-[1440px] flex-col gap-12 px-4 py-10 md:px-10">
                <ProjectsHeroSection data={projects.hero.clone()} />
                <ProjectsFilterBar data={projects.filters.clone()} />
                <ProjectsGrid cards={projects.cards.clone()} />
                <CaseStudySection data={projects.case_study.clone()} />
            </main>
            <ProjectsFooterCtaSection data={projects.footer_cta.clone()} />
        </div>
    }
}

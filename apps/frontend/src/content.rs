use serde::Deserialize;
use web_sys::console;

#[derive(Debug, Deserialize, Clone)]
pub struct SiteContent {
    pub hero: HeroContent,
    pub cta: CtaContent,
    pub footer: FooterContent,
    pub stats: Vec<StatItem>,
    pub projects: ProjectsContent,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HeroContent {
    pub kicker: String,
    pub title: String,
    pub highlight: String,
    pub suffix: String,
    pub subtitle: String,
    pub primary_cta: HeroCta,
    pub secondary_cta: HeroCta,
    pub code_snippet: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HeroCta {
    pub label: String,
    pub url: String,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CtaContent {
    pub title: String,
    pub subtitle: String,
    pub primary_cta: String,
    pub secondary_cta: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FooterContent {
    pub brand: String,
    pub copyright: String,
    pub timezone: String,
    pub local_time: String,
    pub links: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StatItem {
    pub value: String,
    pub label: String
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ProjectsContent {
    pub hero: ProjectsHero,
    pub filters: ProjectsFilters,
    pub cards: Vec<ProjectCard>,
    pub case_study: CaseStudyContent,
    pub footer_cta: ProjectsFooterCta,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ProjectsHero {
    pub title: String,
    pub highlight: String,
    pub subtitle: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ProjectsFilters {
    pub categories: Vec<String>,
    pub stack: Vec<String>,
    pub years: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ProjectCard {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub badge: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CaseStudyContent {
    pub badges: Vec<String>,
    pub title: String,
    pub description: String,
    pub stats: Vec<CaseStudyStat>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CaseStudyStat {
    pub value: String,
    pub label: String,
    pub accent: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ProjectsFooterCta {
    pub title: String,
    pub description: String,
    pub primary_label: String,
    pub secondary_label: String,
    pub footer_note: String,
}

pub fn load_site_content() -> SiteContent {
    let raw = include_str!("../assets/content.yaml");
    match serde_yaml::from_str(raw) {
        Ok(content) => content,
        Err(err) => {
            console::error_1(&format!("Failed to parse content.yaml: {err}").into());
            panic!("content.yaml must be valid");
        }
    }
}

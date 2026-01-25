use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SiteContent {
    pub hero: HeroContent,
    pub cta: CtaContent,
    pub footer: FooterContent,
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

pub fn load_site_content() -> SiteContent {
    let raw = include_str!("../assets/content.yaml");
    serde_yaml::from_str(raw).expect("content.yaml must be valid")
}

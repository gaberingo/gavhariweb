use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ProjectCardProps {
    badge: &'static str,
    category: &'static str,
    title: &'static str,
    cta_icon: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
    #[prop_or("")]
    image_url: &'static str,
    #[prop_or("130deg,_#1c232a,_#0f141c")]
    gradient: &'static str
}

#[function_component]
fn ProjectsCard(props: &ProjectCardProps) -> Html {
    html! {
        <article class="group relative flex flex-col overflow-hidden rounded-3xl border border-[#392c28] bg-[#17110f] transition-all duration-300 hover:-translate-y-1 hover:border-[#ec4913]/50">
            <div class={format!("h-2 w-full bg-[linear-gradient({})]", props.gradient)}></div>
            <div class="flex flex-1 flex-col gap-4 px-6 pb-6 pt-5">
                <div class="flex flex-wrap items-center gap-3 text-[11px] uppercase tracking-[0.3em] text-white/50">
                    <span>{props.badge}</span>
                    <span class="rounded-full border border-[#392c28] bg-[#1f1715] px-3 py-1 text-[10px]">{props.category}</span>
                </div>
                <h3 class="text-xl font-bold text-white transition group-hover:text-[#ec4913]">{props.title}</h3>
                {
                    if props.image_url.is_empty() {
                        html! {
                            <div class="h-56 w-full rounded-2xl border border-[#392c28] bg-[#1a1311]">
                                <div class="h-full w-full rounded-2xl bg-[linear-gradient(110deg,_rgba(236,73,19,0.08),_rgba(0,0,0,0),_rgba(236,73,19,0.1))] opacity-70 animate-pulse"></div>
                            </div>
                        }
                    } else {
                        html! {
                            <img
                                class="h-56 w-full rounded-2xl border border-[#392c28] object-cover"
                                src={props.image_url}
                                alt={props.title}
                                loading="lazy"
                                decoding="async"
                            />
                        }
                    }
                }
                <p class="text-sm text-white/60 line-clamp-3">
                    {props.description}
                </p>
                <div class="flex flex-wrap gap-2 text-xs font-mono">
                    {for props.tags.iter().map(|tag| {
                        html!{<span class="rounded-full border border-[#392c28] bg-[#181311] px-3 py-1 text-[#ec4913]">{*tag}</span>}
                    })}
                </div>
                <div class="flex items-center justify-between text-xs text-white/40">
                    <span>{"Detail build & architecture"}</span>
                    <span class="inline-flex items-center gap-2 text-white/60">
                        {"Explore"}
                        <span class="flex h-8 w-8 items-center justify-center rounded-full border border-white/10 bg-black/30 text-white/70 transition group-hover:border-[#ec4913]/60 group-hover:text-white">
                            {props.cta_icon}
                        </span>
                    </span>
                </div>
            </div>
        </article>
    }
}

/// Grid proyek unggulan.
#[function_component]
pub fn FeaturedProjectsSection() -> Html {
    let dummy_data: Vec<ProjectCardProps> = vec![
        ProjectCardProps {
            badge: "Realtime",
            category: "Fintech",
            title: "High-Freq Trading Bot",
            cta_icon: "↗",
            description: "Latency rendah dan desain arsitektur deterministic untuk market data.",
            tags: vec!["Rust", "Tokio", "WebSocket"],
            image_url: "https://images.unsplash.com/photo-1518770660439-4636190af475?auto=format&fit=crop&w=800&q=80",
            gradient: "130deg,_#1c232a,_#0f141c",
        },
        ProjectCardProps {
            badge: "Visual",
            category: "Media",
            title: "WASM Image Processor",
            cta_icon: "↗",
            description: "Pipeline WebAssembly untuk komputasi foto real-time.",
            tags: vec!["Yew", "WASM", "Canvas API"],
            image_url: "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=800&q=80",
            gradient: "130deg,_#111827,_#0b0f1f",
        },
        ProjectCardProps {
            badge: "Infra",
            category: "Storage",
            title: "Distributed KV Store",
            cta_icon: "↗",
            description: "Consensus layer untuk penyimpanan konsisten multi-region.",
            tags: vec!["Rust", "gRPC", "Raft"],
            image_url: "https://images.unsplash.com/photo-1451187580459-43490279c0fa?auto=format&fit=crop&w=800&q=80",
            gradient: "130deg,_#0b2d2f,_#07191a",
        },
    ];
    html! {
        <section class="relative overflow-hidden border-t border-[#392c28] bg-[#120e0c] py-20" data-doc="Featured projects grid.">
            <div class="pointer-events-none absolute inset-0">
                <div class="absolute -left-24 top-10 h-64 w-64 rounded-full bg-[#ec4913]/10 blur-[110px]"></div>
                <div class="absolute bottom-0 right-0 h-72 w-72 rounded-full bg-[#14b8a6]/10 blur-[130px]"></div>
            </div>
            <div class="relative mx-auto max-w-[1280px] px-4 md:px-10">
                <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
                    <div>
                        <div class="inline-flex items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.3em] text-white/60">
                            <span class="h-1.5 w-1.5 rounded-full bg-[#ec4913]"></span>
                            {"Featured Projects"}
                        </div>
                        <h2 class="mt-4 text-3xl font-bold sm:text-4xl">{"Highlighted builds"}</h2>
                        <p class="mt-3 max-w-xl text-sm text-white/60">
                            {"Eksperimen produk yang fokus pada low-latency, UX, dan reliability untuk aplikasi modern."}
                        </p>
                    </div>
                    <button class="hidden items-center gap-2 rounded-full border border-[#392c28] bg-[#1c1513] px-4 py-2 text-sm font-medium text-white/70 transition hover:border-[#ec4913]/60 hover:text-white md:flex">
                        {"View all repositories"}
                    </button>
                </div>
                <div class="mt-12 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                    {for dummy_data.iter().map(|project| {
                        html! {
                            <ProjectsCard
                                badge={project.badge}
                                category={project.category}
                                title={project.title}
                                cta_icon={project.cta_icon}
                                description={project.description}
                                tags={project.tags.clone()}
                                gradient={project.gradient}
                            />
                        }
                    })}
                </div>
                <div class="mt-8 text-center md:hidden">
                    <button class="rounded-full border border-[#392c28] bg-[#1c1513] px-4 py-2 text-sm font-medium text-white/70 transition hover:border-[#ec4913]/60 hover:text-white">
                        {"View all repositories"}
                    </button>
                </div>
            </div>
        </section>
    }
}

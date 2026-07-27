use leptos::prelude::*;
use leptos_meta::Title;

use crate::{
    components::animations::animate_in::AnimateIn,
    data::{AreaInfo, AREAS},
    layouts::main::MainLayout,
};

#[component]
pub fn AreasPage() -> impl IntoView {
    view! {
        <MainLayout>
            <Title text="Areas"/>
            <AreasHero/>
            <AreasGrid/>
        </MainLayout>
    }
}

#[component]
fn AreasHero() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden bg-base-300 px-6 pb-20 pt-36">
            <div
                class="pointer-events-none absolute inset-0 opacity-40"
                style="background-image: radial-gradient(circle, rgba(255,255,255,0.06) 1px, transparent 1px); background-size: 28px 28px;"
            />
            <div
                class="pointer-events-none absolute inset-0"
                style="background-image: radial-gradient(ellipse at 15% 60%, rgba(16,185,129,0.12) 0%, transparent 55%), radial-gradient(ellipse at 85% 40%, rgba(6,182,212,0.08) 0%, transparent 55%);"
            />

            <div class="relative mx-auto max-w-6xl">
                <a
                    href="/"
                    class="mb-8 inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-base-content/70 backdrop-blur-sm transition hover:border-white/20 hover:bg-white/10 hover:text-base-content"
                >
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                    </svg>
                    "Home"
                </a>

                <div class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
                    <div>
                        <span class="mb-4 inline-block rounded-full bg-emerald-500/10 px-4 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300">
                            "Explore"
                        </span>
                        <h1 class="text-6xl font-extrabold tracking-tight md:text-8xl">
                            "Town "
                            <span class="bg-gradient-to-r from-emerald-400 via-teal-300 to-cyan-400 bg-clip-text text-transparent">
                                "Areas"
                            </span>
                        </h1>
                    </div>
                    <p class="mb-2 max-w-xs text-right text-base-content/50 md:text-right">
                        {format!("{} unique districts, each with its own identity, atmosphere, and story.", AREAS.len())}
                    </p>
                </div>

                <div class="mt-8 h-px w-full bg-gradient-to-r from-transparent via-white/10 to-transparent"/>
            </div>
        </section>
    }
}

#[component]
fn AreasGrid() -> impl IntoView {
    view! {
        <section class="bg-base-200 px-6 py-16">
            <div class="mx-auto max-w-6xl">
                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    {AREAS.iter().copied().map(|area| view! {
                        <AreaCard area=area/>
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn AreaCard(area: AreaInfo) -> impl IntoView {
    let href = format!("/areas/{}", area.slug);
    let badge_bg = format!("background-color: rgba({}, 0.12);", area.accent_rgb);
    let warp_bg = format!(
        "background-color: rgba({}, 0.08); color: rgba({}, 0.9);",
        area.accent_rgb, area.accent_rgb
    );

    view! {
        <AnimateIn>
            <a
                href=href
                class="group relative flex flex-col overflow-hidden rounded-2xl border border-white/6 bg-base-300 shadow-lg transition-all duration-300 hover:-translate-y-1 hover:border-white/12 hover:shadow-2xl hover:shadow-black/40"
            >
                <div class=format!("h-[3px] w-full shrink-0 {}", area.accent_bar)/>

                <div class="relative aspect-[4/3] overflow-hidden">
                    <div
                        class="absolute inset-0 bg-cover bg-center transition-transform duration-500 ease-out group-hover:scale-105"
                        style=format!("background-image: url('{}')", area.image)
                    />
                    <div class="absolute inset-0 bg-gradient-to-t from-base-300/60 via-transparent to-transparent"/>
                </div>

                <div class="flex flex-1 flex-col p-5">
                    <div class="mb-3">
                        <span
                            class=format!("rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-widest {}", area.accent_text)
                            style=badge_bg
                        >
                            {area.theme}
                        </span>
                    </div>

                    <h3 class="mb-2 text-xl font-bold text-white">{area.name}</h3>
                    <p class="mb-5 line-clamp-2 flex-1 text-sm leading-relaxed text-base-content/55">
                        {area.description}
                    </p>

                    <div class="flex items-center justify-between border-t border-white/5 pt-4">
                        <code
                            class="rounded-lg px-3 py-1.5 font-mono text-xs"
                            style=warp_bg
                        >
                            {area.warp_cmd}
                        </code>
                        <span class=format!("flex items-center gap-1 text-sm font-semibold transition-all duration-200 group-hover:gap-2.5 {}", area.accent_text)>
                            "Explore"
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                            </svg>
                        </span>
                    </div>
                </div>
            </a>
        </AnimateIn>
    }
}

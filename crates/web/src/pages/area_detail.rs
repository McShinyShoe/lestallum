// SPDX-License-Identifier: GPL-3.0-or-later

use leptos::prelude::*;
use leptos::web_sys;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::{
    data::{AreaInfo, AREAS},
    layouts::main::MainLayout,
};

#[derive(Clone, Copy)]
struct LightboxState {
    src: &'static str,
    cx: i32,
    cy: i32,
    closing: bool,
}

#[component]
pub fn AreaDetailPage() -> impl IntoView {
    let params = use_params_map();
    let area = move || {
        let slug = params.with(|p| p.get("area").unwrap_or_default().to_owned());
        AREAS.iter().copied().find(|a| a.slug == slug)
    };

    view! {
        <MainLayout>
            {move || match area() {
                Some(a) => view! { <AreaDetailView area=a/> }.into_any(),
                None => view! { <AreaNotFound/> }.into_any(),
            }}
        </MainLayout>
    }
}

#[component]
fn AreaDetailView(area: AreaInfo) -> impl IntoView {
    let lightbox: RwSignal<Option<LightboxState>> = RwSignal::new(None);
    let title = area.name;
    let copy_js = format!(
        "navigator.clipboard.writeText('{}').then(()=>{{let t=this;t.textContent='Copied!';setTimeout(()=>t.textContent='Copy',2000)}}).catch(()=>{{}})",
        area.warp_cmd
    );
    let hero_gradient = format!(
        "background-image: linear-gradient(to bottom, rgba({rgb},0.25) 0%, rgba(0,0,0,0.55) 60%, var(--color-base-100) 100%), url('{img}'); background-size: cover; background-position: center;",
        rgb = area.accent_rgb,
        img = area.image,
    );
    let badge_style = format!(
        "background-color: rgba({}, 0.15); border: 1px solid rgba({}, 0.3);",
        area.accent_rgb, area.accent_rgb
    );
    let warp_glow = format!(
        "box-shadow: 0 0 0 1px rgba({}, 0.25), 0 4px 24px rgba({}, 0.12);",
        area.accent_rgb, area.accent_rgb
    );
    let copy_btn_style = format!(
        "background-color: rgba({}, 0.15); color: rgba({}, 0.9);",
        area.accent_rgb, area.accent_rgb
    );
    let accent_bar = format!(
        "background: linear-gradient(to right, transparent, rgba({rgb}, 0.9) 50%, transparent);",
        rgb = area.accent_rgb
    );
    let screenshot_border = format!("border-color: rgba({}, 0.2);", area.accent_rgb);

    view! {
        <Title text=title/>

        <section class="relative h-[70vh] min-h-[480px]" style=hero_gradient>
            <div class="absolute inset-x-0 top-0 h-32 bg-gradient-to-b from-black/30 to-transparent"/>

            <div class="relative flex h-full flex-col justify-end px-6 pb-16 pt-32">
                <div class="mx-auto w-full max-w-6xl">
                    <a
                        href="/areas"
                        class="mb-8 inline-flex items-center gap-2 rounded-xl border border-white/15 bg-black/30 px-4 py-2 text-sm font-medium text-white/70 backdrop-blur-sm transition hover:border-white/30 hover:bg-black/50 hover:text-white"
                    >
                        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        "All Areas"
                    </a>

                    <br/>

                    <span
                        class=format!("mb-4 inline-block rounded-full px-4 py-1 text-xs font-semibold uppercase tracking-widest {}", area.accent_text)
                        style=badge_style
                    >
                        {area.theme}
                    </span>

                    <h1 class="text-6xl font-extrabold tracking-tight text-white drop-shadow-2xl md:text-8xl">
                        {area.name}
                    </h1>

                    <div class="mt-4 h-[3px] w-72 rounded-full mx-auto" style=accent_bar/>
                </div>
            </div>
        </section>

        <section class="bg-base-100 px-6 py-10">
            <div class="mx-auto max-w-6xl">
                <div
                    class="flex flex-col items-start justify-between gap-4 rounded-2xl border border-white/5 bg-base-300 p-6 sm:flex-row sm:items-center"
                    style=warp_glow
                >
                    <div>
                        <p class="mb-1 text-xs font-semibold uppercase tracking-widest text-base-content/40">
                            "Teleport Command"
                        </p>
                        <code class=format!("font-mono text-2xl font-bold {}", area.accent_text)>
                            {area.warp_cmd}
                        </code>
                    </div>
                    <button
                        onclick=copy_js
                        class="rounded-xl px-5 py-2.5 text-sm font-semibold transition hover:brightness-110 active:scale-95"
                        style=copy_btn_style
                    >
                        "Copy"
                    </button>
                </div>
            </div>
        </section>

        <section class="bg-base-200 px-6 py-16">
            <div class="mx-auto max-w-6xl">
                <div class="grid gap-12 md:grid-cols-[1fr_2fr]">
                    <div>
                        <span class="mb-3 block text-xs font-semibold uppercase tracking-widest text-base-content/40">
                            "About"
                        </span>
                        <h2 class="text-4xl font-bold leading-tight">
                            {area.name}
                        </h2>
                    </div>
                    <p class="text-lg leading-relaxed text-base-content/70 md:pt-8">
                        {area.long_description}
                    </p>
                </div>
            </div>
        </section>

        <section class="bg-base-100 px-6 py-16">
            <div class="mx-auto max-w-6xl">
                <div class="mb-8 flex items-baseline justify-between">
                    <h2 class="text-3xl font-bold">"Screenshots"</h2>
                    <span class="text-sm text-base-content/40">"More coming soon"</span>
                </div>
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                    {area.screenshots.iter().copied().map(|src| {
                        let img_style = format!("background-image: url('{}');", src);
                        let border = screenshot_border.clone();
                        view! {
                            <div
                                class="group relative aspect-video cursor-pointer overflow-hidden rounded-xl border"
                                style=border
                                on:click=move |e: web_sys::MouseEvent| lightbox.set(Some(LightboxState { src, cx: e.client_x(), cy: e.client_y(), closing: false }))
                            >
                                <div
                                    class="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-105"
                                    style=img_style
                                />
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>

        {move || lightbox.get().map(|state| {
            let close_lightbox = move || {
                lightbox.update(|opt| {
                    if let Some(s) = opt { s.closing = true; }
                });
            };
            let backdrop_style = if state.closing {
                "animation: lb-backdrop-out 0.3s ease forwards;"
            } else {
                "animation: lb-backdrop-in 0.38s ease forwards;"
            };
            let content_style = if state.closing {
                "animation: lb-out 0.3s ease forwards;".to_string()
            } else {
                format!(
                    "transform-origin: {}px {}px; animation: lb-in 0.38s cubic-bezier(0.4,0,0.2,1) forwards;",
                    state.cx, state.cy
                )
            };
            view! {
                <div
                    class="fixed inset-0 z-40 bg-black/80 backdrop-blur-sm"
                    style=backdrop_style
                />
                <div
                    class="fixed inset-0 z-50 flex items-center justify-center"
                    style=content_style
                    on:click=move |_| close_lightbox()
                    on:animationend=move |_| {
                        if lightbox.get().map(|s| s.closing).unwrap_or(false) {
                            lightbox.set(None);
                        }
                    }
                >
                    <div
                        class="relative mx-4 w-full max-w-5xl"
                        on:click=move |e: web_sys::MouseEvent| e.stop_propagation()
                    >
                        <button
                            class="absolute -top-10 left-0 flex items-center gap-2 text-sm font-medium text-white/60 transition hover:text-white"
                            on:click=move |_| close_lightbox()
                        >
                            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                            "Close"
                        </button>
                        <img
                            src=state.src
                            class="max-h-[85vh] w-full rounded-xl object-contain shadow-2xl"
                        />
                    </div>
                </div>
            }
        })}
    }
}

#[component]
fn AreaNotFound() -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col items-center justify-center bg-base-100 px-6 text-center">
            <p class="mb-2 text-sm uppercase tracking-[0.4em] text-emerald-300/80">"Unknown district"</p>
            <h1 class="bg-gradient-to-br from-white via-slate-100 to-emerald-200 bg-clip-text text-8xl font-extrabold tracking-tight text-transparent drop-shadow-2xl">
                "404"
            </h1>
            <div class="mt-4 h-px w-32 bg-gradient-to-r from-transparent via-emerald-400 to-transparent"/>
            <p class="mt-6 max-w-md text-base-content/60">"This area doesn't exist."</p>
            <div class="mt-10 flex gap-4">
                <a
                    href="/areas"
                    class="btn btn-outline btn-lg rounded-full px-8"
                >
                    "All Areas"
                </a>
                <a
                    href="/"
                    class="btn btn-primary btn-lg rounded-full px-8 shadow-xl shadow-emerald-500/20 transition hover:scale-105"
                >
                    "Home"
                </a>
            </div>
        </div>
    }
}

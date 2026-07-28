// SPDX-License-Identifier: GPL-3.0-or-later

use leptos::ev;
use leptos::prelude::*;

use crate::data::DISCORD_URL;

const FLOAT_THRESHOLD_PX: f64 = 8.0;

#[derive(Clone, Copy)]
pub struct NavMenus {
    pub profile_open: RwSignal<bool>,
    pub mobile_open: RwSignal<bool>,
    pub info_open: RwSignal<bool>,
}

impl Default for NavMenus {
    fn default() -> Self {
        Self {
            profile_open: RwSignal::new(false),
            mobile_open: RwSignal::new(false),
            info_open: RwSignal::new(false),
        }
    }
}

impl NavMenus {
    pub fn any_open(self) -> bool {
        self.profile_open.get() || self.mobile_open.get() || self.info_open.get()
    }

    pub fn close_all(self) {
        self.profile_open.set(false);
        self.mobile_open.set(false);
        self.info_open.set(false);
    }
}

fn is_scrolled_past_top() -> bool {
    window().scroll_y().unwrap_or(0.0) > FLOAT_THRESHOLD_PX
}

#[component]
pub fn Navbar() -> impl IntoView {
    let menus = use_context::<NavMenus>().unwrap_or_default();
    let floating = RwSignal::new(false);

    Effect::new(move |_| {
        floating.set(is_scrolled_past_top());

        let handle = window_event_listener(ev::scroll, move |_| {
            floating.set(is_scrolled_past_top());
        });

        on_cleanup(move || handle.remove());
    });

    view! {
        <nav class=move || format!(
            "fixed inset-x-0 top-0 z-50 px-4 transition-all duration-300 ease-out {}",
            if floating.get() { "pt-4" } else { "pt-0" }
        )>
            <div class="relative">

                <div class=move || format!(
                    "absolute inset-0 border border-white/10 bg-base-200/80 backdrop-blur-md pointer-events-none transition-all duration-300 ease-out {}",
                    if floating.get() { "rounded-2xl shadow-2xl" } else { "rounded-t-none rounded-b-2xl shadow-lg" }
                ) />

                <div class="relative flex items-center justify-between px-6 py-3">

                    <div class="flex items-center gap-2">
                        <button
                            class="lg:hidden flex h-8 w-8 items-center justify-center rounded-lg text-base-content/60 transition hover:bg-white/10 hover:text-base-content"
                            on:click=move |_| menus.mobile_open.update(|v| *v = !*v)
                            aria-label="Open menu"
                        >
                            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                            </svg>
                        </button>

                        <a href="/" class="flex items-center gap-2 text-lg font-bold tracking-tight">
                            <img src="/favicon-32x32.png" alt="✦" class="w-5 h-5" />
                            "Lestallum"
                        </a>
                    </div>

                    <ul class="hidden lg:flex items-center gap-1 text-sm font-medium">
                        <NavLink href="/">"Home"</NavLink>
                        <NavLink href="/areas">"Areas"</NavLink>
                        <NavLink href="/map">"Map"</NavLink>
                        <InfoDropdown info_open=menus.info_open/>
                        <li>
                            <a
                                href=DISCORD_URL
                                target="_blank"
                                rel="noopener"
                                class="rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
                            >
                                "Discord"
                            </a>
                        </li>
                    </ul>

                    <button
                        on:click=move |_| menus.profile_open.update(|v| *v = !*v)
                        class="flex h-10 w-10 items-center justify-center rounded-full border border-white/10 bg-base-300 text-base-content/60 transition hover:border-emerald-500/40 hover:text-emerald-400"
                        aria-label="Profile options"
                    >
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                            />
                        </svg>
                    </button>

                </div>

                <Show when=move || menus.mobile_open.get()>
                    <div
                        on:click=|ev| ev.stop_propagation()
                        class="absolute left-0 top-full mt-2 w-56 overflow-hidden rounded-xl border border-white/10 bg-base-200/80 shadow-2xl backdrop-blur-md"
                    >
                        <ul class="p-2 text-sm font-medium">
                            <MobileNavLink href="/">"Home"</MobileNavLink>
                            <MobileNavLink href="/areas">"Areas"</MobileNavLink>
                            <MobileNavLink href="/map">"Map"</MobileNavLink>
                            <li class="px-1 pb-1 pt-3 text-xs font-semibold uppercase tracking-widest text-base-content/40">
                                "Info"
                            </li>
                            <MobileNavLink href="/rules">"Rules"</MobileNavLink>
                            <MobileNavLink href="/lore">"Lore"</MobileNavLink>
                            <MobileNavLink href="/gallery">"Gallery"</MobileNavLink>
                            <li class="my-1 border-t border-white/5"/>
                            <li>
                                <a
                                    href=DISCORD_URL
                                    target="_blank"
                                    rel="noopener"
                                    class="flex w-full rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
                                >
                                    "Discord"
                                </a>
                            </li>
                        </ul>
                    </div>
                </Show>

                <Show when=move || menus.profile_open.get()>
                    <div
                        on:click=|ev| ev.stop_propagation()
                        on:contextmenu=|ev| ev.prevent_default()
                        class="absolute right-0 top-full mt-2 w-52 overflow-hidden rounded-xl border border-white/10 bg-base-200/80 shadow-2xl backdrop-blur-md"
                    >
                        <ProfileMenuSection label="Account">
                            <ProfileMenuItem icon="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1">
                                "Login"
                            </ProfileMenuItem>
                        </ProfileMenuSection>
                    </div>
                </Show>

            </div>
        </nav>
    }
}

#[component]
fn InfoDropdown(info_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <li>
            <button
                on:click=move |ev| {
                    ev.stop_propagation();
                    info_open.update(|v| *v = !*v);
                }
                class="flex items-center gap-1 rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content cursor-pointer select-none"
            >
                "Info"
                <svg
                    class=move || format!(
                        "h-3 w-3 transition-transform duration-200{}",
                        if info_open.get() { "" } else { " -rotate-90" }
                    )
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                </svg>
            </button>
            <Show when=move || info_open.get()>
                <ul
                    on:click=|ev| ev.stop_propagation()
                    class="absolute top-full mt-2 w-40 rounded-xl border border-white/10 bg-base-200/80 shadow-2xl backdrop-blur-md"
                >
                    <InfoLink href="/rules">"Rules"</InfoLink>
                    <InfoLink href="/lore">"Lore"</InfoLink>
                    <InfoLink href="/gallery">"Gallery"</InfoLink>
                </ul>
            </Show>
        </li>
    }
}

#[component]
fn InfoLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="flex px-3 py-2 text-sm text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
            >
                {children()}
            </a>
        </li>
    }
}

#[component]
fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
            >
                {children()}
            </a>
        </li>
    }
}

#[component]
fn MobileNavLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="flex w-full rounded-lg px-3 py-2 text-base-content/70 transition hover:bg-white/10 hover:text-base-content"
            >
                {children()}
            </a>
        </li>
    }
}

#[component]
pub fn ProfileMenuSection(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="border-b border-white/5 last:border-b-0">
            <p class="px-4 pb-1 pt-3 text-xs font-semibold uppercase tracking-widest text-base-content/40">
                {label}
            </p>
            <ul class="p-1.5">
                {children()}
            </ul>
        </div>
    }
}

#[component]
pub fn ProfileMenuItem(icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <button class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm text-base-content/80 transition hover:bg-white/10 hover:text-base-content">
                <svg class="h-4 w-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon/>
                </svg>
                {children()}
            </button>
        </li>
    }
}

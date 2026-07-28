// SPDX-License-Identifier: GPL-3.0-or-later

use leptos::prelude::*;

use crate::data::{DISCORD_URL, LICENSE_URL, SOURCE_URL};

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="border-t border-white/5 bg-base-300 px-6 py-16">
            <div class="mx-auto max-w-6xl">
                <div class="grid gap-10 md:grid-cols-4">
                    <div class="md:col-span-2">
                        <h3 class="mb-3 text-2xl font-bold tracking-tight">"Lestallum Town"</h3>
                        <p class="text-sm leading-relaxed text-base-content/60">
                            "A Minecraft town built by a community of dedicated players.
                            Come visit, build with us, and stay for the company."
                        </p>
                    </div>

                    <div>
                        <h4 class="mb-4 text-sm font-semibold uppercase tracking-widest text-emerald-400">
                            "Contact"
                        </h4>
                        <ul class="space-y-2 text-sm text-base-content/70">
                            <li>"Server: mc.thecavern.net"</li>
                            <li>"Email: lestallum@shinyshoe.net"</li>
                            <li>"Discord: lestallum"</li>
                        </ul>
                    </div>

                    <div>
                        <h4 class="mb-4 text-sm font-semibold uppercase tracking-widest text-emerald-400">
                            "Links"
                        </h4>
                        <ul class="space-y-2 text-sm">
                            <li>
                                <a href=DISCORD_URL class="text-base-content/70 transition hover:text-emerald-400">
                                    "Discord Server"
                                </a>
                            </li>
                            <li>
                                <a href="/rules" class="text-base-content/70 transition hover:text-emerald-400">
                                    "Town Rules"
                                </a>
                            </li>
                            <li>
                                <a href="/map" class="text-base-content/70 transition hover:text-emerald-400">
                                    "Live Map"
                                </a>
                            </li>
                            <li>
                                <a href="/wiki" class="text-base-content/70 transition hover:text-emerald-400">
                                    "Wiki"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="mt-12 flex flex-col items-center justify-between gap-4 border-t border-white/5 pt-8 text-sm text-base-content/50 sm:flex-row">
                    <p>"© 2026 ShinyShoe. Not affiliated with Mojang or Microsoft."</p>
                    <p>"Created with Leptos :3"</p>
                </div>

                <div class="mt-6 space-y-2 text-xs leading-relaxed text-base-content/40">
                    <p>
                        "Lestallum is free software: you may redistribute and modify it under the terms of the "
                        <a
                            href=LICENSE_URL
                            target="_blank"
                            rel="noopener noreferrer"
                            class="underline underline-offset-2 transition hover:text-emerald-400"
                        >
                            "GNU General Public License, version 3 or later"
                        </a>
                        ". It comes with ABSOLUTELY NO WARRANTY, to the extent permitted by law."
                    </p>
                    <p>
                        <a
                            href=SOURCE_URL
                            target="_blank"
                            rel="noopener noreferrer"
                            class="underline underline-offset-2 transition hover:text-emerald-400"
                        >
                            "Source code"
                        </a>
                    </p>
                </div>
            </div>
        </footer>
    }
}

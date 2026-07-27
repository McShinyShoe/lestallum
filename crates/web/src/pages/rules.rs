use leptos::prelude::*;
use leptos_meta::Title;

use crate::layouts::main::MainLayout;

#[component]
pub fn RulesPage() -> impl IntoView {
    view! {
        <MainLayout>
            <Title text="Rules"/>
            <RulesHero/>
            <RulesContent/>
        </MainLayout>
    }
}

#[component]
fn RulesHero() -> impl IntoView {
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

            <div class="relative mx-auto max-w-4xl">
                <a
                    href="/"
                    class="mb-8 inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-base-content/70 backdrop-blur-sm transition hover:border-white/20 hover:bg-white/10 hover:text-base-content"
                >
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                    </svg>
                    "Home"
                </a>

                <br/>

                <span class="mb-4 inline-block rounded-full bg-emerald-500/10 px-4 py-1 text-xs font-semibold uppercase tracking-widest text-emerald-300">
                    "Town Guidelines"
                </span>
                <h1 class="text-6xl font-extrabold tracking-tight md:text-8xl">
                    "Town "
                    <span class="bg-gradient-to-r from-emerald-400 via-teal-300 to-cyan-400 bg-clip-text text-transparent">
                        "Rules"
                    </span>
                </h1>
                <p class="mt-4 mx-auto max-w-xl text-center text-base-content/60">
                    "Guidelines and information for living in Lestallum. Please read everything carefully."
                </p>

                <div class="mt-8 h-px w-full bg-gradient-to-r from-transparent via-white/10 to-transparent"/>
            </div>
        </section>
    }
}

#[component]
fn RulesContent() -> impl IntoView {
    view! {
        <section class="bg-base-200 px-6 py-16">
            <div class="mx-auto max-w-4xl space-y-8">

                <RulesCard
                    icon="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                    title="Rules"
                    accent="emerald"
                >
                    <RuleItem>"Do NOT harass or grief other people in and outside of our town."</RuleItem>
                    <RuleItem>"Do NOT spam on townchat."</RuleItem>
                    <RuleItem>"Be respectful to others and follow town staff."</RuleItem>
                    <RuleItem>"Please don't beg."</RuleItem>
                    <RuleItem>"Don't grief the road."</RuleItem>
                    <RuleItem>"Be nice :D"</RuleItem>
                </RulesCard>

                <RulesCard
                    icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    title="Taxes"
                    accent="teal"
                >
                    <p class="text-base-content/70 leading-relaxed">
                        "We have "
                        <span class="font-bold text-emerald-400">"no tax"</span>
                        ". Our tax is 0%, which means you will not pay tax no matter how many plots you have or how long you are in our town."
                    </p>
                </RulesCard>

                <RulesCard
                    icon="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                    title="Plots"
                    accent="cyan"
                >
                    <div class="space-y-4 text-base-content/70 leading-relaxed">
                        <p>
                            "If you need a plot, ask a town Assistant to claim you one it will cost "
                            <span class="font-bold text-emerald-400">"$0"</span>
                            ". To claim a plot yourself, use "
                            <Code>"/plot claim"</Code>
                            "."
                        </p>
                        <p>
                            "You "
                            <span class="font-semibold text-red-400">"CANNOT"</span>
                            " claim a $10k plot. A $10k plot is open to claim but requires town staff permission."
                        </p>
                        <div>
                            <p class="mb-2 font-semibold text-base-content/90">"Your plot expires if:"</p>
                            <ul class="space-y-1 pl-4">
                                <ExpireItem>"You're offline for 60+ days."</ExpireItem>
                                <ExpireItem>"Your chunk is flat and you're offline for 10+ days."</ExpireItem>
                                <ExpireItem>"You quit the town."</ExpireItem>
                            </ul>
                        </div>
                        <p>
                            "When that happens, some of your items will be moved to Town Storage at "
                            <Code>"/pw Storage"</Code>
                            "."
                        </p>
                    </div>
                </RulesCard>

                <RulesCard
                    icon="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"
                    title="Moving Chunk"
                    accent="emerald"
                >
                    <div class="space-y-2 text-base-content/70 leading-relaxed">
                        <p>"If you ask town staff to move your chunk, you will have "
                            <span class="font-bold text-emerald-400">"10 days"</span>
                            " to move your items."
                        </p>
                        <p>"Tell staff as soon as you are done moving. Moving is "
                            <span class="font-bold text-emerald-400">"free"</span>
                            "."
                        </p>
                    </div>
                </RulesCard>

                <RulesCard
                    icon="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
                    title="Town Storage"
                    accent="teal"
                >
                    <div class="space-y-3 text-base-content/70 leading-relaxed">
                        <p>
                            "Town Storage is a private storage accessible by town Residents for storing all kinds of items. Some items are automated such as Basic Building Blocks and SF Materials. Access it with "
                            <Code>"/pw Storage"</Code>
                            "."
                        </p>
                        <div class="grid gap-3 sm:grid-cols-2 mt-4">
                            <PlatformCard platform="Java">
                                "Right-click the note block on the first floor."
                            </PlatformCard>
                            <PlatformCard platform="Bedrock">
                                "Access some items at the Basement Floor by standing on the elevator plate."
                            </PlatformCard>
                        </div>
                    </div>
                </RulesCard>

                <RulesCard
                    icon="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    title="Town Farm"
                    accent="cyan"
                >
                    <div class="space-y-3 text-base-content/70 leading-relaxed">
                        <p>
                            "Town Vanilla Farm is accessible at "
                            <Code>"/t outpost Farm"</Code>
                            ". Every user of the farm "
                            <span class="font-bold text-yellow-400">"MUST replant all crops"</span>
                            " after harvesting."
                        </p>
                        <p>
                            "Our MoFood Farm is accessible at "
                            <Code>"/pw farm"</Code>
                            "."
                        </p>
                    </div>
                </RulesCard>

                <RulesCard
                    icon="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7"
                    title="Free Items"
                    accent="emerald"
                >
                    <div class="space-y-3 text-base-content/70 leading-relaxed">
                        <p>"Our town provides free items to residents:"</p>
                        <div class="grid gap-3 sm:grid-cols-3">
                            <FreeItemBadge>"SF Materials"</FreeItemBadge>
                            <FreeItemBadge>"SF Machines"</FreeItemBadge>
                            <FreeItemBadge>"Building Blocks"</FreeItemBadge>
                        </div>
                        <ul class="space-y-1 mt-2 pl-1">
                            <ExpireItem>
                                <span>"SF Materials and Building Blocks: " <Code>"/pw Storage"</Code></span>
                            </ExpireItem>
                            <ExpireItem>"SF Machines: ask town staff directly."</ExpireItem>
                        </ul>
                    </div>
                </RulesCard>

                <RulesCard
                    icon="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                    title="Town Assistance"
                    accent="teal"
                >
                    <p class="text-base-content/70 leading-relaxed">
                        "Have a question about any plugin? Ask freely in townchat and we'll answer if we can. You can also ask in our Discord. For Slimefun-related questions, many of our town members can help as well."
                    </p>
                </RulesCard>

            </div>
        </section>
    }
}

#[component]
fn RulesCard(
    icon: &'static str,
    title: &'static str,
    accent: &'static str,
    children: Children,
) -> impl IntoView {
    let (bar_class, badge_class) = match accent {
        "teal" => (
            "bg-gradient-to-r from-teal-500 to-cyan-400",
            "bg-teal-500/10 text-teal-300",
        ),
        "cyan" => (
            "bg-gradient-to-r from-cyan-500 to-sky-400",
            "bg-cyan-500/10 text-cyan-300",
        ),
        _ => (
            "bg-gradient-to-r from-emerald-500 to-teal-400",
            "bg-emerald-500/10 text-emerald-300",
        ),
    };

    view! {
        <div class="overflow-hidden rounded-2xl border border-white/6 bg-base-300 shadow-lg">
            <div class=format!("h-[3px] w-full {}", bar_class)/>
            <div class="p-6 md:p-8">
                <div class="mb-5 flex items-center gap-3">
                    <span class=format!("flex h-10 w-10 items-center justify-center rounded-xl {}", badge_class)>
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon/>
                        </svg>
                    </span>
                    <h2 class="text-xl font-bold">{title}</h2>
                </div>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn RuleItem(children: Children) -> impl IntoView {
    view! {
        <li class="flex items-start gap-3 py-2 border-b border-white/5 last:border-b-0">
            <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-emerald-500/20 text-emerald-400">
                <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"/>
                </svg>
            </span>
            <span class="text-base-content/75 leading-relaxed">{children()}</span>
        </li>
    }
}

#[component]
fn ExpireItem(children: Children) -> impl IntoView {
    view! {
        <li class="flex items-start gap-2">
            <span class="mt-1.5 h-1.5 w-1.5 shrink-0 rounded-full bg-red-400/70"/>
            <span class="text-base-content/70">{children()}</span>
        </li>
    }
}

#[component]
fn Code(children: Children) -> impl IntoView {
    view! {
        <code class="rounded-md bg-base-100 px-1.5 py-0.5 font-mono text-xs text-emerald-300">
            {children()}
        </code>
    }
}

#[component]
fn PlatformCard(platform: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-white/8 bg-base-200 p-4">
            <p class="mb-1 text-xs font-semibold uppercase tracking-widest text-emerald-400">{platform}</p>
            <p class="text-sm text-base-content/70">{children()}</p>
        </div>
    }
}

#[component]
fn FreeItemBadge(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center rounded-xl border border-emerald-500/20 bg-emerald-500/10 px-3 py-2 text-sm font-semibold text-emerald-300">
            {children()}
        </div>
    }
}

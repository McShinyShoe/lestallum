use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="404 Not Found"/>
        <div class="flex min-h-screen flex-col items-center justify-center bg-base-100 px-6 text-center">
            <p class="mb-2 text-sm uppercase tracking-[0.4em] text-emerald-300/80">"Aint nuthin here"</p>
            <h1 class="bg-gradient-to-br from-white via-slate-100 to-emerald-200 bg-clip-text text-8xl font-extrabold tracking-tight text-transparent drop-shadow-2xl sm:text-9xl">
                "404"
            </h1>
            <div class="mt-4 h-px w-32 bg-gradient-to-r from-transparent via-emerald-400 to-transparent"></div>
            <p class="mt-6 max-w-md text-lg text-base-content/60">
                "This page doesn't exist."
            </p>
            <div class="mt-10 flex flex-wrap items-center justify-center gap-4">
                <button
                    onclick="history.back()"
                    class="btn btn-outline btn-lg rounded-full px-8"
                >
                    "Back"
                </button>
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

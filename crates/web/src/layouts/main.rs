use leptos::prelude::*;

use crate::components::animations::overlay_group::{OverlayAnimation, OverlayAnimationGroup};
use crate::components::animations::random_video::VideoPosition;
use crate::components::sections::footer::SiteFooter;
use crate::components::sections::navbar::{NavMenus, Navbar};

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let menus = NavMenus::default();
    provide_context(menus);

    view! {
        <Show when=move || menus.any_open()>
            <div class="fixed inset-0 z-40" on:click=move |_| menus.close_all()/>
        </Show>

        <Navbar/>
        <OverlayAnimationGroup frequency_secs=5>
            <OverlayAnimation src="animation/right_hi_shiny.webm" width="400px" position=VideoPosition::Right/>
        </OverlayAnimationGroup>
        {children()}
        <SiteFooter/>
    }
}

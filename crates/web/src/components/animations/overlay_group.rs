use std::time::Duration;

use leptos::prelude::*;

use crate::components::animations::random_video::VideoPosition;

#[derive(Clone, Copy)]
struct AnimationGroupCtx {
    active_id: RwSignal<Option<usize>>,
    child_count: RwSignal<usize>,
    frequency_ms: u32,
}

impl AnimationGroupCtx {
    fn schedule_next(self) {
        let jitter = js_sys::Math::random() * 0.5 + 0.75;
        let delay = Duration::from_millis((f64::from(self.frequency_ms) * jitter) as u64);

        set_timeout(
            move || {
                let count = self.child_count.get_untracked();
                if count > 0 {
                    let index = ((js_sys::Math::random() * count as f64) as usize).min(count - 1);
                    self.active_id.set(Some(index));
                }
            },
            delay,
        );
    }
}

#[component]
pub fn OverlayAnimationGroup(frequency_secs: u32, children: Children) -> impl IntoView {
    let ctx = AnimationGroupCtx {
        active_id: RwSignal::new(None),
        child_count: RwSignal::new(0),
        frequency_ms: frequency_secs.saturating_mul(1000),
    };
    provide_context(ctx);

    Effect::new(move |_| ctx.schedule_next());

    view! { {children()} }
}

#[component]
pub fn OverlayAnimation(
    src: &'static str,
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
    #[prop(optional)] position: VideoPosition,
) -> impl IntoView {
    let Some(ctx) = use_context::<AnimationGroupCtx>() else {
        return ().into_any();
    };

    let my_id = ctx.child_count.get_untracked();
    ctx.child_count.update(|n| *n += 1);

    let video_ref = NodeRef::<leptos::html::Video>::new();

    Effect::new(move |_| {
        if ctx.active_id.get() == Some(my_id) {
            if let Some(video) = video_ref.get() {
                video.set_current_time(0.0);
                let _ = video.play();
            }
        }
    });

    let pos_css = position.to_css();

    view! {
        <div style=move || {
            let base = format!("position:fixed;{pos_css}z-index:50;pointer-events:none;");
            if ctx.active_id.get() == Some(my_id) {
                base
            } else {
                format!("{base}visibility:hidden;")
            }
        }>
            <video
                node_ref=video_ref
                src=src
                muted=true
                playsinline=true
                style=move || {
                    let mut s = String::new();
                    if let Some(w) = width  { s.push_str(&format!("width:{w};")); }
                    if let Some(h) = height { s.push_str(&format!("height:{h};")); }
                    s
                }
                on:ended=move |_| {
                    ctx.active_id.set(None);
                    ctx.schedule_next();
                }
            />
        </div>
    }
    .into_any()
}

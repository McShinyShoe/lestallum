use std::time::Duration;

use leptos::prelude::*;

#[derive(Clone, Copy, Default)]
pub enum VideoPosition {
    Top,
    TopRight,
    #[default]
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl VideoPosition {
    pub fn to_css(self) -> &'static str {
        match self {
            Self::Top => "top:0;left:50%;transform:translateX(-50%);",
            Self::TopRight => "top:0;right:0;",
            Self::Right => "right:0;top:50%;transform:translateY(-50%);",
            Self::BottomRight => "bottom:0;right:0;",
            Self::Bottom => "bottom:0;left:50%;transform:translateX(-50%);",
            Self::BottomLeft => "bottom:0;left:0;",
            Self::Left => "left:0;top:50%;transform:translateY(-50%);",
            Self::TopLeft => "top:0;left:0;",
        }
    }
}

#[component]
pub fn RandomVideoAnimation(
    src: &'static str,
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
    #[prop(optional)] position: VideoPosition,
) -> impl IntoView {
    let visible = RwSignal::new(false);
    let video_ref = NodeRef::<leptos::html::Video>::new();

    Effect::new(move |_| schedule_next(visible));

    Effect::new(move |_| {
        if visible.get() {
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
            if visible.get() { base } else { format!("{base}visibility:hidden;") }
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
                    visible.set(false);
                    schedule_next(visible);
                }
            />
        </div>
    }
}

fn schedule_next(visible: RwSignal<bool>) {
    let delay = Duration::from_millis((js_sys::Math::random() * 25_000.0 + 5_000.0) as u64);
    set_timeout(move || visible.set(true), delay);
}

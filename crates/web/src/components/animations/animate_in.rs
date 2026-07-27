use leptos::prelude::*;
use leptos::web_sys::{IntersectionObserver, IntersectionObserverEntry};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn AnimateIn(children: Children) -> impl IntoView {
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(element) = wrapper_ref.get() else {
            return;
        };

        let on_intersect = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _: IntersectionObserver| {
                let entry = entries.get(0).unchecked_into::<IntersectionObserverEntry>();
                if entry.is_intersecting() {
                    visible.set(true);
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

        if let Ok(observer) = IntersectionObserver::new(on_intersect.as_ref().unchecked_ref()) {
            observer.observe(element.as_ref());
            on_intersect.forget();
        }
    });

    view! {
        <div
            node_ref=wrapper_ref
            class=move || if visible.get() {
                "animate-in fade-in zoom-in slide-in-from-bottom duration-500"
            } else {
                "opacity-0"
            }
        >
            {children()}
        </div>
    }
}

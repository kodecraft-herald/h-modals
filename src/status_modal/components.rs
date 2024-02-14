use leptos::{RwSignal, IntoView, Show, view, component, SignalGet, SignalSet};
use crate::attributes::enums::ComponentStatus;

#[component]
pub fn StatusModal(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    #[prop(optional)]
    status: Option<ComponentStatus>,
) -> impl IntoView
{
    let status_class = match status {
        Some(ComponentStatus::Info) => "info-modal-box",
        Some(ComponentStatus::Success) => "success-modal-box",
        Some(ComponentStatus::Neutral) => "neutral-modal-box",
        Some(ComponentStatus::Warning) => "warning-modal-box",
        Some(ComponentStatus::Error) => "error-modal-box",
        _ => "neutral-modal-box",
    };

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class="modal-cust-top blur-bg">
                <div class={status_class}>
                    <h3 class="font-bold text-2xl text-black">{title.clone()}</h3>
                    <p class="py-4 text-black">{description.clone()}</p>
                    <div class="modal-action">
                        <button class="btn btn-sm rounded" title="Close" on:click = move |_| signal.set(false)>Close</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
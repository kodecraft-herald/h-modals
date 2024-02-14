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
        Some(ComponentStatus::Info) => "modal-box bg-info rounded-box",
        Some(ComponentStatus::Success) => "modal-box bg-success rounded-box",
        Some(ComponentStatus::Neutral) => "modal-box bg-neutral rounded-box",
        Some(ComponentStatus::Warning) => "modal-box bg-warning rounded-box",
        Some(ComponentStatus::Error) => "modal-box bg-error rounded-box",
        _ => "modal-box bg-neutral rounded-box",
    };

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class="modal-top">
                    <div class={status_class}>
                        <h3 class="font-bold text-2xl text-black">{title.clone()}</h3>
                        <p class="py-4 text-black">{description.clone()}</p>
                        <div class="modal-action">
                            <button class="btn btn-sm rounded" title="Close" on:click = move |_| signal.set(false)>Close</button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
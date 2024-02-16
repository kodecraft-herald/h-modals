use leptos::{RwSignal, IntoView, Show, view, component, SignalGet, SignalSet};
use crate::attributes::enums::{ComponentStatus, Position};

#[component]
pub fn StatusModal(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    #[prop(optional)]
    position: Option<Position>,
    #[prop(optional)]
    status: Option<ComponentStatus>,
    // #[prop(optional)]
    // custom_class: Option<String>,
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

    let position_class = match position {
        Some(Position::TopLeft) => "modal-top",
        Some(Position::Middle) => "modal-middle",
        Some(Position::BottomMiddle) => "modal-bottom-middle",
        _ => "modal-top-middle",
    };

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class={position_class}>
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
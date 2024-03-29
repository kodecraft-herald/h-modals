use crate::attributes::enums::{ComponentStatus, Position};
use leptos::{component, view, IntoView, RwSignal, Show, SignalGet};

/// Component for displaying a status modal that calls a function when closed.
/// Accepts the following parameters:
/// - signal: A `RwSignal<bool>` to control the visibility of the modal
/// - title: The title of the modal
/// - description: The description of the modal
/// - position: The [`Position`] of the modal (default is [`Position::TopMiddle`])
/// - status: The [`ComponentStatus`] of the modal (default is [`ComponentStatus::Neutral`])
/// - button_status: The [`ComponentStatus`] of the button (default is `No Status`)
/// - text_color: The [`ComponentStatus`] of the text (default is `Black Text`)
/// - function: The closure that will be called after the modal is closed (Can be anything as long as the closure doesn't have parameters).

#[allow(non_snake_case)]
#[component]
pub fn StatusModalWithFunction<F>(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    mut function: F,
    #[prop(optional)] position: Option<Position>,
    #[prop(optional)] status: Option<ComponentStatus>,
    #[prop(optional)] button_status: Option<ComponentStatus>,
    #[prop(optional)] text_color: Option<ComponentStatus>,
    // #[prop(optional)]
    // custom_class: Option<String>,
) -> impl IntoView
where
    F: FnMut() + Clone + 'static,
{
    let status_class = match status {
        Some(ComponentStatus::Info) => "modal-box bg-info rounded-box",
        Some(ComponentStatus::Success) => "modal-box bg-success rounded-box",
        Some(ComponentStatus::Neutral) => "modal-box bg-neutral rounded-box",
        Some(ComponentStatus::Warning) => "modal-box bg-warning rounded-box",
        Some(ComponentStatus::Error) => "modal-box bg-error rounded-box",
        _ => "modal-box bg-neutral rounded-box",
    };

    let text_header_class = match text_color {
        Some(ComponentStatus::Info) => "text-info",
        Some(ComponentStatus::Success) => "text-success",
        Some(ComponentStatus::Neutral) => "text-neutral",
        Some(ComponentStatus::Warning) => "text-warning",
        Some(ComponentStatus::Error) => "text-error",
        _ => "text-black",
    };

    let text_desc_class = match text_color {
        Some(ComponentStatus::Info) => "text-info-content",
        Some(ComponentStatus::Success) => "text-success-content",
        Some(ComponentStatus::Neutral) => "text-neutral-content",
        Some(ComponentStatus::Warning) => "text-warning-content",
        Some(ComponentStatus::Error) => "text-error-content",
        _ => "text-black",
    };

    let button_class = match button_status {
        Some(ComponentStatus::Info) => "btn btn-sm rounded btn-info",
        Some(ComponentStatus::Success) => "btn btn-sm rounded btn-success",
        Some(ComponentStatus::Neutral) => "btn btn-sm rounded",
        Some(ComponentStatus::Warning) => "btn btn-sm rounded btn-warning",
        Some(ComponentStatus::Error) => "btn btn-sm rounded btn-error",
        _ => "btn btn-sm rounded",
    };

    let position_class = match position {
        Some(Position::TopLeft) => "modal-top-left",
        Some(Position::TopMiddle) => "modal-top-middle",
        Some(Position::TopRight) => "modal-top-right",
        Some(Position::Middle) => "modal-middle",
        Some(Position::BottomLeft) => "modal-bottom-left",
        Some(Position::BottomMiddle) => "modal-bottom-middle",
        Some(Position::BottomRight) => "modal-bottom-right",
        _ => "modal-top-middle",
    };

    let on_click = move |_| {
        function();
    };

    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class={position_class}>
                    <div class={status_class}>
                        <h3 class=format!("font-bold text-2xl {}", text_header_class)>{title.clone()}</h3>
                        <p class=format!("py-4 {}", text_desc_class)>{description.clone()}</p>
                        <div class="modal-action">
                            <button class={button_class} title="Close" on:click = on_click.clone()>Close</button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

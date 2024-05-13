use crate::attributes::enums::{Colors, ComponentStatus, Position};
use leptos::{component, view, IntoView, RwSignal, Show, SignalGet, SignalSet};

/// Component for displaying a status modal
/// Accepts the following parameters:
/// - signal: A `RwSignal<bool>` to control the visibility of the modal
/// - title: The title of the modal
/// - description: The description of the modal
/// - position: The [`Position`] of the modal (default is [`Position::TopMiddle`])
/// - status: The [`ComponentStatus`] of the modal (default is [`ComponentStatus::Neutral`])
/// - button_status: The [`ComponentStatus`] of the button (default is `No Status`)
/// - text_color: The [`ComponentStatus`] of the text (default is `Black Text`)

#[allow(non_snake_case)]
#[component]
pub fn StatusModal(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    #[prop(optional)] position: Option<Position>,
    #[prop(optional)] status: Option<ComponentStatus>,
    #[prop(optional)] button_status: Option<ComponentStatus>,
    #[prop(optional)] header_text_color: Option<ComponentStatus>,
    #[prop(optional)] desc_text_color: Option<Colors>,
    #[prop(optional)] custom_position_class: Option<String>,
) -> impl IntoView {
    let status_class = match status {
        Some(ComponentStatus::Info) => "bg-info",
        Some(ComponentStatus::Success) => "bg-success",
        Some(ComponentStatus::Neutral) => "bg-neutral",
        Some(ComponentStatus::Warning) => "bg-warning",
        Some(ComponentStatus::Error) => "bg-error",
        _ => "",
    };

    let text_header_class = match header_text_color {
        Some(ComponentStatus::Info) => "text-info",
        Some(ComponentStatus::Success) => "text-success",
        Some(ComponentStatus::Neutral) => "text-neutral",
        Some(ComponentStatus::Warning) => "text-warning",
        Some(ComponentStatus::Error) => "text-error",
        _ => "",
    };

    let text_desc_class = match desc_text_color {
        Some(Colors::White) => "text-white",
        Some(Colors::Black) => "text-black",
        Some(Colors::Gray) => "text-gray",
        _ => "",
    };

    let button_class = match button_status {
        Some(ComponentStatus::Info) => "btn btn-sm rounded btn-info",
        Some(ComponentStatus::Success) => "btn btn-sm rounded btn-success",
        Some(ComponentStatus::Neutral) => "btn btn-sm rounded",
        Some(ComponentStatus::Warning) => "btn btn-sm rounded btn-warning",
        Some(ComponentStatus::Error) => "btn btn-sm rounded btn-error",
        _ => "btn btn-sm rounded",
    };

    let position_class = match custom_position_class {
        None => {
            match position {
                Some(Position::TopLeft) => "items-start justify-start".to_string(),
                Some(Position::TopCenter) => "items-start justify-center".to_string(),
                Some(Position::TopRight) => "items-start justify-end".to_string(),
                Some(Position::Center) => "items-center justify-center".to_string(),
                Some(Position::BottomLeft) => "items-end justify-start".to_string(),
                Some(Position::BottomCenter) => "items-end justify-center".to_string(),
                Some(Position::BottomRight) => "items-end justify-end".to_string(),
                _ => "items-center justify-center".to_string(),
            }
        },
        Some(custom_class) => custom_class,
    };


    view! {

        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class=format!("sticky top-0 flex h-screen {}", position_class.clone())>
                    <div class=format!("flex flex-col gap-4 m-2 modal-box {}", status_class)>
                        <h3 class=format!("font-bold text-2xl {}", text_header_class)>{title.clone()}</h3>
                        <p class=format!("py-4 text-lg {}", text_desc_class)>{description.clone()}</p>
                        <div class="modal-action">
                            <button class={button_class} title="Close" on:click = move |_| signal.set(false)>Close</button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

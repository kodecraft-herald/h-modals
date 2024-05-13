use leptos::{RwSignal, ReadSignal, IntoView, Show, view, component, SignalGet, SignalSet};

use crate::attributes::enums::Position;

/// Component for displaying a confirmation modal
/// Accepts the following parameters:
/// - signal: A `RwSignal<bool>` to control the visibility of the modal
/// - title: The title of the modal
/// - description: The description of the modal
/// - function: A closure that will be called when the confirm button is clicked
/// - pending_signal: A `ReadSignal<bool>` to control the visibility of the loading spinner
/// - position[optional]: The [`Position`] of the modal (default is [`Position::Center`])

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModal<C>(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    mut function: C,
    pending_signal: ReadSignal<bool>,
    #[prop(optional)]
    position: Option<Position>,
    #[prop(optional)] custom_position_class: Option<String>,
) -> impl IntoView
where C: FnMut() + Clone + 'static,
{
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

    let on_click = move |_| function();
    view! {
        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class=format!("sticky top-0 flex h-screen {}", position_class.clone())>
                    <div class="flex flex-col gap-4 m-2 modal-box">
                        <h3 class="font-bold text-2xl">{title.clone()}</h3>
                        <p class="py-4 text-lg">{description.clone()}</p>
                        <div class="modal-action">
                            <button class="btn btn-error btn-sm rounded" prop:disabled=pending_signal title="Cancel" on:click = move |_| signal.set(false)>Cancel</button>
                            {
                                match pending_signal.get() {
                                    true => view! {
                                        <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                    }.into_any(),
                                    false => view! {
                                        <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = on_click.clone()>Confirm</button>
                                    }.into_any(),
                                }
                            }

                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
use leptos::{RwSignal, ReadSignal, IntoView, Show, view, component, SignalGet, SignalSet};

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModal<C>(
    signal: RwSignal<bool>,
    title: String,
    description: String,
    mut function: C,
    pending_signal: ReadSignal<bool>,
) -> impl IntoView
where C: FnMut() + Clone + 'static,
{
    let on_click = move |_| function();
    view! {
        <Show when=move || signal.get() fallback=|| ()>
            <div class = "blur-bg">
                <div class="modal-top">
                    <div class="modal-box rounded-box">
                        <h3 class="font-bold text-2xl">{title.clone()}</h3>
                        <p class="py-4">{description.clone()}</p>
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
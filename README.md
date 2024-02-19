## status modal usage:
```rust
let modal = create_rw_signal(false);

view! {
    <button class = "btn btn-sm" on:click = move |_| modal.set(true)>Open Modal</button>

    <StatusModal
        signal=modal
        title="TITLE!".to_string()
        description="The description goes here.".to_string()
        position=Position::TopMiddle
        status=ComponentStatus::Success
    />
}

```

## confirm modal usage:
```rust
let signal = create_rw_signal(false);

let action = create_action(move |_input: &()| async move {
    let result = server_function().await;
    match result {
        Ok(result) => {
            log::info!("Action Successful!");
            signal.set(false); // Close the modal after successful action.
            return result
        },
        Err(err) => {
            log::error!("Server Function Error: {:?}", err);
            signal.set(false); // Close the modal after unsuccessful action.
            return format!("Server Function Error: {:?}", err);
        }
    }
});

let pending_signal = action.pending(); // Renders a spinner when the action is still pending.

let confirm_modal_fn = move || {
    action.dispatch(()); // Calls the actual action inside a closure function.
};

view! {
    <button class = "btn btn-sm" on:click = move |_| signal.set(true)>Open Modal</button>
    <ConfirmModal
        signal = signal
        title = "APPROVE?".to_string()
        description = "Are you sure you want to approve?".to_string()
        function = confirm_modal_fn
        pending_signal = pending_signal
    />
}

pub async fn server_function() -> Result<String, ServerFnError>{
    use gloo_timers::future::sleep;
    use std::time::Duration;

    log::info!("Server Function!");

    sleep(Duration::from_secs(1)).await; // Add a delay to simulate pending data.

    return Ok(String::from("This came from a server function."))

}

```

## what goes inside the confirm modal:
```rust

let position_class = match position {
    Some(Position::TopLeft) => "modal-top-left",
    Some(Position::TopMiddle) => "modal-top-middle",
    Some(Position::TopRight) => "modal-top-right",
    Some(Position::Middle) => "modal-middle",
    Some(Position::BottomLeft) => "modal-bottom-left",
    Some(Position::BottomMiddle) => "modal-bottom-middle",
    Some(Position::BottomRight) => "modal-bottom-right",
    _ => "modal-middle",
};

let on_click = move |_| function();

view! {
    <Show when=move || signal.get() fallback=|| ()>
        <div class = "blur-bg">
            <div class={position_class}>
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

```

## what goes inside the status modal:
```rust

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

let button_component = match function {
    Some(mut function) => {
        let on_click = move |_| {
            function();
        };
        view! {
            <button class={button_class} title="Close" on:click = on_click.clone()>Close</button>
        }
    },
    None => view! {
        <button class={button_class} title="Close" on:click = move |_| signal.set(false)>Close</button>
    }
};

view! {
    <Show when=move || signal.get() fallback=|| ()>
        <div class = "blur-bg">
            <div class={position_class}>
                <div class={status_class}>
                    <h3 class=format!("font-bold text-2xl {}", text_header_class)>{title.clone()}</h3>
                    <p class=format!("py-4 {}", text_desc_class)>{description.clone()}</p>
                    <div class="modal-action">
                        {button_component.clone()}
                    </div>
                </div>
            </div>
        </div>
    </Show>
}

```

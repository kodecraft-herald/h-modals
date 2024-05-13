## status modal usage:
```rust
let modal = create_rw_signal(false);

view! {
    <button class = "btn btn-sm" on:click = move |_| modal.set(true)>Open Modal</button>

    <StatusModal
        signal=modal
        title="TITLE!".to_string()
        description="The description goes here.".to_string()
        position=Position::TopCenter
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
                <div class=format!("flex flex-col gap-4 m-2 modal-box {}", status_class)>
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

```

## what goes inside the status modal:
```rust

let status_class = match status {
    Some(ComponentStatus::Info) => "bg-info",
    Some(ComponentStatus::Success) => "bg-success",
    Some(ComponentStatus::Neutral) => "bg-neutral",
    Some(ComponentStatus::Warning) => "bg-warning",
    Some(ComponentStatus::Error) => "bg-error",
    _ => "",
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
    Some(ComponentStatus::Info) => "text-black",
    Some(ComponentStatus::Success) => "text-black",
    Some(ComponentStatus::Neutral) => "",
    Some(ComponentStatus::Warning) => "text-black",
    Some(ComponentStatus::Error) => "text-black",
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
                    <p class=format!("py-4 text-lg{}", text_desc_class)>{description.clone()}</p>
                    <div class="modal-action">
                        <button class={button_class} title="Close" on:click = move |_| signal.set(false)>Close</button>
                    </div>
                </div>
            </div>
        </div>
    </Show>
}

```

use std::iter::repeat_with;
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[derive(PartialEq, Clone)]
struct LogEvent {
    uid: u32,
    content: Rc<String>,
}

#[derive(PartialEq, Clone)]
struct Filter {
    filter: String,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let filter = create_signal(
        cx,
        Filter {
            filter: String::new(),
        },
    );

    provide_context_ref(cx, filter);

    log::info!("Create initial");
    let data: &Signal<Vec<_>> = create_signal(
        cx,
        (0..10000)
            .into_iter()
            .map(|i| LogEvent {
                uid: i,
                content: std::rc::Rc::new(repeat_with(fastrand::alphanumeric).take(16).collect()),
            })
            .collect(),
    );
    log::info!("Logs generated");

    let handle_filter = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        filter.modify().filter = target.value();
    };

    view! {cx,
        div(class="log-filter") {
            input(
                class="edit",
                on:input=handle_filter,
            )
        }

        div(class="log-container") {
            Keyed(
                iterable = data,
                view = |cx, log| {
                    let filter = use_context::<Signal<Filter>>(cx);
                    let enabled = create_selector(cx, { let log = log.clone(); move || {
                        log::info!("reeval filter");
                        log.content.contains(&filter.get().filter)
                    }});

                    view! {cx, (
                        if *enabled.get() {
                            log::info!("reeval log");
                            // In the real app, this is a more sophisticated formatting operation
                            let rendered_message = log.content.clone();

                            view! {cx, div (class="log-entry") {
                                (rendered_message)
                            }}
                        } else {
                            view! {cx,}
                        }
                    )}
                },
                key = |x| (x.uid)
            )
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| {
        view! { cx,
            App {}
        }
    });
}

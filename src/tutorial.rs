use leptos::component;
use leptos::ev;
use leptos::ev::MouseEvent;
use leptos::html::button;
use leptos::html::div;
use leptos::html::span;
use leptos::prelude::*;
use leptos::IntoView;
use leptos_router::hooks::use_navigate;

use crate::context::auth::get_auth_context;

#[component]
pub fn Tutorial() -> impl IntoView {
    let get_auth = get_auth_context();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if get_auth.read().is_none() {
            navigate("/login", Default::default());
        }
    });

    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let is_odd = move || count.get() & 1 == 1;
    let message = move || is_odd().then(|| "THE NUMBER IS ODD");

    let (toggled, set_toggled) = signal(false);
    provide_context(set_toggled);
    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            style="position: absolute"
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable
            style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>

        <p class=("red", is_odd)>"Double count:" {move || count.get() * 2}</p>
        <p>{message}</p>
        <p>{move || if is_odd() { "Odd" } else { "Even" }}</p>
        <Show when=move || { count.get() > 5 } fallback=|| view! { <p>"SMOL"</p> }>
            <p>"BIG"</p>
        </Show>

        <ProgressBar progress=count />
        <ProgressBar max=50 progress=count />
        <ProgressBar max=100 progress=Signal::derive(double_count) />

        <List />
        <BadDynamicList />
        <DynamicList />

        <ErrorHandling />

        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled />
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value) />
        <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value) />
        <ButtonD />
        <TakesChildren render_prop=|| {
            view! { <p>"Hi, there!"</p> }
        }>"Some text" <span>"A span"</span></TakesChildren>
        <WrapsChildren>"Child 1" "Child 2"</WrapsChildren>
        {counter(1, 1)}
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
        <br />
    }
}

#[component]
fn List() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        <p>{values.clone()}</p>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
    }
}

#[component]
fn BadDynamicList() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

#[component]
fn DynamicList() -> impl IntoView {
    let length = 5;
    let mut next_counter_id = length + 1;
    let initial_counters = (1..=length)
        .map(|id| (id, ArcRwSignal::new(id)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig));
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button on:click=move |_| *count.write() += 1>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .write()
                                        .retain(|(counter_id, _)| { *counter_id != id });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn ErrorHandling() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));
    view! {
        <label>
            "Type an integer (or not!)"
            <input
                type="number"
                on:input:target=move |e| { set_value.set(e.target().value().parse::<i32>()) }
            />
            <ErrorBoundary fallback=|errors| {
                view! {
                    <p>"Not a number! Errors: "</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}
                    </ul>
                }
            }>
                <p>"You entered: "<strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle A"</button> }
}

#[component]
fn ButtonB(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle B"</button> }
}

#[component]
fn ButtonC() -> impl IntoView {
    view! { <button>"Toggle C"</button> }
}

#[component]
fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter proivder");
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle D"</button> }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1>
            <code>"<TakesChildren/>"</code>
        </h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr />
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();
    view! {
        <h1>
            <code>"<WrapsChildren/>"</code>
        </h1>
        <ul>{children}</ul>
    }
}

pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            // typed events found in leptos::ev
            // 1) prevent typos in event names
            // 2) allow for correct type inference in callbacks
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
    ))
}

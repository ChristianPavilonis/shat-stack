#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn ShatButton(count: u16) -> Component {
    let url = format!("/shat/{}", count);
    html! {
        <button hx-post=url hx-swap="outerHTML" hx-target="#shat-stack" class="px-4 py-1 bg-[#865D0A] rounded-full text-white text-3xl">
            Shat me
        </button>
    }
}

pub fn ShatStack(count: u16) -> Component {
    let shats = (0..count)
        .into_iter()
        .map(|_| {
            html! {
                <span>
                    "💩"
                </span>
            }
        })
        .collect::<Vec<Component>>();

    html! {
        <div id="shat-stack">
            <ShatButton count=count + 1></ShatButton>
            <div class="flex flex-col gap-1 text-3xl">
                {shats}
            </div>
        </div>
    }
}

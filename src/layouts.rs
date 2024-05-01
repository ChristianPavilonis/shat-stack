#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn Layout(elements: Elements) -> Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width,initial-scale=1.0" />
                <link rel="stylesheet" href="/app.css">
                <script src="https://unpkg.com/htmx.org@1.9.12"></script>
                <title>SHAT STACK</title>
            </head>
            <body>
                <main class="mx-auto max-w-[900px]">
                    {elements}
                </main>
            </body>
        </html>
    }
}

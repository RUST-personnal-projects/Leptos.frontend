use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <html>
            <head>
                <title>My Leptos App</title>
            </head>
            <body>
                <p>"Hello, world!"</p>
                <script type="module" src="simple_script.js"></script>
                <script type="module" src="bevy_loader.js"></script>
            </body>
        </html>
    }
}

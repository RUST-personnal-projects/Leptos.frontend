use yew::prelude::*;
use yew_router::prelude::*;

mod video;

use video::videos::Videos;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/video")]
    Video,
    #[at("/*path")]
    Misc { path: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Video => html! { <Videos /> },
        Route::Misc { path } => html! {
                <>
                    <p>{format!("Matched some other path: {}", path)}</p>
                    <MyComponent />
                </>
            },
    }
}


#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <button {onclick}>{"Click to go home"}</button>
        </>
    }
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
    
}

fn main() {
    yew::Renderer::<App>::new().render();
}
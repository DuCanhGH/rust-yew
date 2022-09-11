use stylist::{yew::styled_component, style};
use yew::prelude::{function_component, html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Routes) -> yew::Html {
    match routes {
        Routes::Home => html! {
            <Test />
        },
        Routes::About => html! {
            <p>{"About this page: ehe"}</p>
        },
        Routes::NotFound => html! {
            <p>{"Page not found"}</p>
        },
    }
}

#[styled_component[Test]]
fn test_com() -> Html {
    let style = style!(
        r#"
            color: black;
        "#
    )
    .expect("Failed to mount style");
    html! {
        <>
            <p class={yew::classes!("bg-red-100", style.get_class_name().to_owned())}>{"Test!"}</p>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Hello World"}</h1>
            <h2>{"Fuk yu"}</h2>
            <BrowserRouter>
                <Link<Routes> to={Routes::Home}>{"To /"}</Link<Routes>>
                <Link<Routes> to={Routes::About}>{"To /about"}</Link<Routes>>
                <Switch<Routes> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

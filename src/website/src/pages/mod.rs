mod client;
mod home;
mod login;
mod product;
mod products;

use home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use login::Home;
use product::Home;
use products::Home;

// Pull in the schema we registered in build.rs
#[cynic::schema("schema")]
mod schema {}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

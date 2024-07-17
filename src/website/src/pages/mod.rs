mod client;
mod home;
mod login;
mod product;
mod products;

use home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use login::Login;
use product::Product;
use products::Products;

pub use products::query::MyQuery;

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
                <Route path="/login" view=  move || view! { <Login/> }/>
                <Route path="/product/:id" view=  move || view! { <Product/> }/>
                <Route path="/products" view=  move || view! { <Products/> }/>
            </Routes>
        </Router>
    }
}

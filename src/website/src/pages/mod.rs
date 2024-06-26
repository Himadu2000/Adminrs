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

// Pull in the schema we registered in build.rs
#[cynic::schema("schema")]
mod schema {}

#[component]
pub fn App() -> impl IntoView {
    let (store_id, _) = create_signal(var("STORE_ID"));
    let (_, set_store_id, _) = use_local_storage::<String, FromToStringCodec>("store_id");
    provide_meta_context();

    //? Set environment variable to local storage for reqwest client
    // "Store ID not provided...!"
    create_effect(move |_| {
        let store_id_env = store_id
            .get_untracked()
            .unwrap_or("store:obgsketriakxn1wh3q2e".to_owned());
        set_store_id.set(store_id_env);
    });

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

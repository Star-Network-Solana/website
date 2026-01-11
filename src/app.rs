use crate::components::features::{AudienceGrid, FeaturesGrid, Hero, MissionStatement};
use crate::components::layout::{Footer, Header};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <Header />
            <Hero />
            <FeaturesGrid />
            <MissionStatement />
            <AudienceGrid />
            <Footer />
        </main>
    }
}

use crate::components::layout::{Footer, Header};
use crate::pages::content_pages::*;
use crate::pages::home::Home;
use crate::pages::registry::Registry;
use crate::pages::trainer_profile::TrainerProfile;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/website">
            <main>
                <Header />
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/registry" view=Registry />
                    <Route path="/registry/:id" view=TrainerProfile />
                    <Route path="/how-it-works" view=HowItWorks />
                    <Route path="/standards" view=Standards />
                    <Route path="/educators" view=Educators />
                    <Route path="/enterprises" view=Enterprises />
                    <Route path="/insights" view=Insights />
                    <Route path="/about" view=About />
                </Routes>
                <Footer />
            </main>
        </Router>
    }
}

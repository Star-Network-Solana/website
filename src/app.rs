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
        <Router>
            <main>
                <Header />
                <Routes>
                    <Route path="/website/" view=Home />
                    <Route path="/website/registry" view=Registry />
                    <Route path="/website/registry/:id" view=TrainerProfile />
                    <Route path="/website/how-it-works" view=HowItWorks />
                    <Route path="/website/standards" view=Standards />
                    <Route path="/website/educators" view=Educators />
                    <Route path="/website/enterprises" view=Enterprises />
                    <Route path="/website/insights" view=Insights />
                    <Route path="/website/about" view=About />
                </Routes>
                <Footer />
            </main>
        </Router>
    }
}

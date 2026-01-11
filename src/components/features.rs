use crate::components::ui::{Badge, Button, Card, SectionBox};
use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section style="
            padding: 8rem 0 6rem;
            background: var(--gradient-hero);
            position: relative;
            overflow: hidden;
        ">
            // Glow effect
            <div style="
                position: absolute;
                top: -50%;
                left: 50%;
                transform: translateX(-50%);
                width: 1000px;
                height: 1000px;
                background: var(--gradient-glow);
                pointer-events: none;
                z-index: 0;
            "></div>

            <div class="container" style="text-align: center; position: relative; z-index: 10;">
                <Badge text="Official Ecosystem Standard" />
                <h1 style="
                    font-size: 3.5rem; 
                    margin-bottom: 1.5rem; 
                    letter-spacing: -0.03em;
                    text-wrap: balance;
                ">
                    "The Trust Layer for " <span class="text-gradient">"Rust Education"</span>
                </h1>
                <p style="
                    font-size: 1.25rem; 
                    max-width: 600px; 
                    margin: 0 auto 3rem;
                    color: var(--color-text-muted);
                    text-wrap: balance;
                ">
                    "Accreditation, verification, and open standards that make Rust training credible, transparent, and reliable for the entire ecosystem."
                </p>
                <div style="display: flex; gap: 1rem; justify-content: center;">
                    <Button href="#standards" text="Explore Standards" variant="primary" />
                    <Button href="#registry-view" text="View Registry" variant="secondary" />
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn FeaturesGrid() -> impl IntoView {
    view! {
        <SectionBox id="capabilities".to_string() class="".to_string()>
            <div class="text-center" style="margin-bottom: 4rem;">
                <h2>"What STAR Network Provides"</h2>
                <p>"Infrastructure for a high-quality education market."</p>
            </div>
            <div style="
                display: grid; 
                grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); 
                gap: 2rem;
            ">
                <Card>
                    <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem; color: var(--color-secondary);">"Accreditation"</h3>
                    <p>"Clear criteria, evaluation rubrics, and review processes that define what credible Rust training looks like."</p>
                </Card>
                <Card>
                    <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem; color: var(--color-secondary);">"Trainer Registry"</h3>
                    <p>"A transparent, searchable registry of trainers whose credentials and experience have been reviewed."</p>
                </Card>
                <Card>
                    <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem; color: var(--color-secondary);">"Learning Pathways"</h3>
                    <p>"Structured, production-aligned skill journeys that reflect real-world Rust usage and industry needs."</p>
                </Card>
                <Card>
                    <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem; color: var(--color-secondary);">"Governance"</h3>
                    <p>"Outcome tracking and continuous updates to ensure standards stay relevant as the ecosystem evolves."</p>
                </Card>
            </div>
        </SectionBox>
    }
}

#[component]
pub fn MissionStatement() -> impl IntoView {
    view! {
        <SectionBox id="mission".to_string() class="".to_string()>
            <div style="
                background: linear-gradient(180deg, rgba(30, 41, 59, 0.5) 0%, rgba(15, 23, 42, 0.5) 100%);
                border: 1px solid var(--color-border);
                border-radius: 20px;
                padding: 4rem 2rem;
                text-align: center;
                max-width: 900px;
                margin: 0 auto;
            ">
                <h2 style="margin-bottom: 2rem;">"Why STAR Exists"</h2>
                <p style="font-size: 1.25rem; color: var(--color-text); max-width: 700px; margin: 0 auto;">
                    "Rust adoption depends on education quality. We ensure that those who teach Rust are trusted, prepared, and aligned with real-world needs—so ecosystems can scale responsibly."
                </p>
            </div>
        </SectionBox>
    }
}

#[component]
pub fn AudienceGrid() -> impl IntoView {
    view! {
        <SectionBox id="audience".to_string() class="".to_string()>
            <h2 class="text-center" style="margin-bottom: 3rem;">"Who STAR Network Is For"</h2>
            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 2rem;">
                <Card>
                    <h4>"Foundations"</h4>
                    <p>"Reduce ecosystem risk and ensure consistent, high-quality training outcomes."</p>
                </Card>
                <Card>
                    <h4>"Training Organizations"</h4>
                    <p>"Move from informal workshops to credible, trusted programs."</p>
                </Card>
                <Card>
                    <h4>"Trainers"</h4>
                    <p>"Gain fair recognition, visibility, and access to better opportunities."</p>
                </Card>
                <Card>
                    <h4>"Learners"</h4>
                    <p>"Learn with confidence through consistent, production-ready education."</p>
                </Card>
            </div>
        </SectionBox>
    }
}

use crate::components::ui::Button;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    // Determine overlapping class for transparent header if needed (not active here, consistent sticky)
    view! {
        <header>
            <div class="container nav-container">
                <div class="logo">
                    <a href="/website/" style="
                        font-weight: 800; 
                        font-size: 1.5rem; 
                        letter-spacing: -0.02em; 
                        color: white;
                        display: flex;
                        align-items: center;
                        gap: 0.5rem;
                    ">
                        <span style="color: var(--color-secondary)">"✦"</span>
                        "STAR Network"
                    </a>
                </div>

                <nav class="desktop-nav" style="display: flex; gap: 2rem;">
                    <a href="#standards">"Standards"</a>
                    <a href="#registry">"Registry"</a>
                    <a href="#governance">"Governance"</a>
                    <a href="https://github.com/Star-Network-Solana" target="_blank">"GitHub"</a>
                </nav>

                <div class="cta-nav">
                    <Button href="#apply" text="Apply as Trainer" variant="primary" />
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer style="
            background: var(--color-surface); 
            padding: 4rem 0; 
            margin-top: 4rem; 
            border-top: 1px solid var(--color-border);
        ">
            <div class="container" style="display: grid; grid-template-columns: 1fr 1fr; gap: 4rem;">
                <div>
                    <h4 style="margin-bottom: 1rem;">"STAR Network"</h4>
                    <p style="max-width: 300px;">
                        "The Trust Layer for Rust Education. Building open standards, verification, and accreditation infrastructure for the ecosystem."
                    </p>
                </div>
                <div style="display: flex; gap: 4rem;">
                    <div>
                        <h4 style="font-size: 0.9rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--color-text-muted);">"Protocol"</h4>
                        <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                            <a href="#standards">"Standards"</a>
                            <a href="#registry">"Registry"</a>
                            <a href="#governance">"Governance"</a>
                        </div>
                    </div>
                </div>
            </div>
            <div class="container" style="margin-top: 4rem; padding-top: 2rem; border-top: 1px solid rgba(255,255,255,0.05); text-align: center; color: var(--color-text-muted); font-size: 0.9rem;">
                "© 2024 STAR Network. Open Source & Community Governance."
            </div>
        </footer>
    }
}

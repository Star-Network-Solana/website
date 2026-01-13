use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        // Section 1: Hero
        <section class="hero" style="min-height: 80vh; display: flex; align-items: center; padding-top: 4rem;">
            <div class="container">
                <h1 style="font-size: 3.5rem; max-width: 900px; margin-bottom: 1.5rem; letter-spacing: -0.02em;">
                    "The Trust Layer for Solana (Rust) Education"
                </h1>
                <h2 style="font-size: 2rem; color: var(--color-accent); font-weight: 500; margin-bottom: 1.5rem;">
                    "Visible Truth. Verifiable Talent."
                </h2>
                <p style="font-size: 1.25rem; max-width: 720px; color: var(--color-text-sub); margin-bottom: 3rem;">
                    "An independent initiative building registry-based trust and verification frameworks for Solana and Rust educators."
                </p>
                <div style="display: flex; gap: 1rem;">
                    <a href="/registry" class="btn btn-muted">"Explore the Registry"</a>
                    <a href="/how-it-works" class="btn btn-muted">"How STAR Works"</a>
                </div>
            </div>
        </section>

        // Section 2: The Broken Reality
        <section style="background: var(--color-surface);">
            <div class="container">
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem;">
                    <div style="padding: 2rem; border-left: 2px solid var(--color-border-dark);">
                        <p style="font-size: 1.1rem; color: white; margin-bottom: 0;">
                            "No neutral way to verify Solana educators under existing fragmentation."
                        </p>
                    </div>
                    <div style="padding: 2rem; border-left: 2px solid var(--color-border-dark);">
                        <p style="font-size: 1.1rem; color: white; margin-bottom: 0;">
                            "Enterprises hire without proof of teaching quality or curriculum standards."
                        </p>
                    </div>
                    <div style="padding: 2rem; border-left: 2px solid var(--color-border-dark);">
                        <p style="font-size: 1.1rem; color: white; margin-bottom: 0;">
                            "No shared standard for mastery or pedagogy across the ecosystem."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        // Section 3: What STAR Is / Is Not
        <section>
            <div class="container" style="display: grid; grid-template-columns: 1fr 1fr; gap: 6rem;">
                // STAR IS
                <div>
                    <h3 style="color: var(--color-accent); font-size: 1rem; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 2rem;">"STAR IS"</h3>
                    <ul style="list-style: none; padding: 0; color: white; font-size: 1.25rem; display: flex; flex-direction: column; gap: 1.5rem;">
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span style="color: var(--color-accent)">"✔"</span> "A registry & verification framework"
                        </li>
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span style="color: var(--color-accent)">"✔"</span> "An independent professional association"
                        </li>
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span style="color: var(--color-accent)">"✔"</span> "A neutral coordination layer"
                        </li>
                    </ul>
                </div>

                // STAR IS NOT
                <div>
                    <h3 style="color: var(--color-text-sub); font-size: 1rem; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 2rem;">"STAR IS NOT"</h3>
                    <ul style="list-style: none; padding: 0; color: var(--color-text-sub); font-size: 1.25rem; display: flex; flex-direction: column; gap: 1.5rem;">
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span>"✕"</span> "A bootcamp"
                        </li>
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span>"✕"</span> "A content platform"
                        </li>
                        <li style="display: flex; align-items: center; gap: 1rem;">
                            <span>"✕"</span> "A recruiter"
                        </li>
                    </ul>
                </div>
            </div>
        </section>

        // Section 4: Four-Phase Model
        <section style="background: rgba(255, 255, 255, 0.02); border-top: 1px solid var(--color-border-dark); border-bottom: 1px solid var(--color-border-dark);">
            <div class="container">
                <h2 style="margin-bottom: 4rem; text-align: left;">"The Trust Protocol"</h2>
                <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 2rem;">

                    <div>
                        <div style="font-size: 0.9rem; color: var(--color-accent); margin-bottom: 0.5rem; font-family: var(--font-mono);">"01"</div>
                        <h4 style="font-size: 1.25rem; margin-bottom: 1rem;">"Truth"</h4>
                        <p style="font-size: 0.95rem; color: var(--color-text-sub);">"Registry of verified actors."</p>
                    </div>

                    <div>
                        <div style="font-size: 0.9rem; color: var(--color-accent); margin-bottom: 0.5rem; font-family: var(--font-mono);">"02"</div>
                        <h4 style="font-size: 1.25rem; margin-bottom: 1rem;">"Discovery"</h4>
                        <p style="font-size: 0.95rem; color: var(--color-text-sub);">"Marketplace for talent."</p>
                    </div>

                    <div>
                        <div style="font-size: 0.9rem; color: var(--color-accent); margin-bottom: 0.5rem; font-family: var(--font-mono);">"03"</div>
                        <h4 style="font-size: 1.25rem; margin-bottom: 1rem;">"Standards"</h4>
                        <p style="font-size: 0.95rem; color: var(--color-text-sub);">"Certification benchmarks."</p>
                    </div>

                    <div>
                        <div style="font-size: 0.9rem; color: var(--color-accent); margin-bottom: 0.5rem; font-family: var(--font-mono);">"04"</div>
                        <h4 style="font-size: 1.25rem; margin-bottom: 1rem;">"Coordination"</h4>
                        <p style="font-size: 0.95rem; color: var(--color-text-sub);">"Blockchain registry."</p>
                    </div>

                </div>
            </div>
        </section>
    }
}

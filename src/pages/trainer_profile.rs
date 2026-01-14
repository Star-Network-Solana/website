use leptos::*;
use leptos_router::*;

#[component]
pub fn TrainerProfile() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // Mock Data for display (would normally fetch based on ID)
    let trainer_name = "Dr. Elena Vance";
    let focus = "Rust Pedagogy & Safety";

    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                <a href="/website/registry" style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 2rem; color: var(--color-text-sub); font-size: 0.9rem;">
                    "← Back to Registry"
                </a>

                // Header
                <div style="border-bottom: 1px solid var(--color-border); padding-bottom: 2rem; margin-bottom: 3rem;">
                    <div style="display: flex; justify-content: space-between; align-items: start;">
                        <div>
                            <h1 style="margin-bottom: 0.5rem;">{trainer_name}</h1>
                            <p style="font-size: 1.25rem; color: var(--color-accent); margin-bottom: 0;">{focus}</p>
                        </div>
                        <div style="text-align: right;">
                             <span style="background: rgba(20, 241, 149, 0.1); color: var(--color-accent); padding: 4px 12px; border-radius: 4px; font-size: 0.85rem; border: 1px solid rgba(20, 241, 149, 0.2); font-weight: 600;">"VERIFIED"</span>
                             <div style="margin-top: 0.5rem; font-size: 0.85rem; color: var(--color-text-sub);">"ID: STAR-" {id}</div>
                        </div>
                    </div>
                </div>

                <div class="flex-responsive" style="align-items: start;">
                    // Main Content
                    <div>
                        // 1. Overview
                        <div style="margin-bottom: 4rem;">
                            <h3 style="border-bottom: 1px solid var(--color-border-dark); padding-bottom: 0.5rem; margin-bottom: 1.5rem; font-size: 1.1rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--color-text-sub);">"Overview"</h3>
                            <p style="font-size: 1.05rem; line-height: 1.7;">
                                "Dr. Vance is a systems programming educator with over 15 years of experience in low-level languages. Formal verification specialist focusing on Rust safety guarantees in Solana program development. Previously taught at MIT and currently leads the Advanced Rust curriculum at Starbase Alpha."
                            </p>
                        </div>

                        // 2. Teaching Background
                        <div style="margin-bottom: 4rem;">
                            <h3 style="border-bottom: 1px solid var(--color-border-dark); padding-bottom: 0.5rem; margin-bottom: 1.5rem; font-size: 1.1rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--color-text-sub);">"Teaching Background"</h3>
                            <ul style="list-style: none; padding: 0; display: flex; flex-direction: column; gap: 1rem;">
                                <li style="display: flex; gap: 1rem;">
                                    <span style="color: var(--color-accent); font-weight: bold;">"2023-Present"</span>
                                    <span>"Lead Instructor, Advanced Rust, Starbase Alpha"</span>
                                </li>
                                <li style="display: flex; gap: 1rem;">
                                    <span style="color: var(--color-text-sub); font-weight: bold;">"2018-2023"</span>
                                    <span>"Senior Lecturer, Computer Science, MIT"</span>
                                </li>
                            </ul>
                        </div>

                        // 3. Technical Experience
                        <div style="margin-bottom: 4rem;">
                            <h3 style="border-bottom: 1px solid var(--color-border-dark); padding-bottom: 0.5rem; margin-bottom: 1.5rem; font-size: 1.1rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--color-text-sub);">"Technical Experience"</h3>
                             <p>"Core contributor to the Anchor framework documentation. Published research on formal verification of smart contracts."</p>
                        </div>
                    </div>

                    // Sidebar
                    <aside>
                        <div class="content-card" style="background: var(--color-surface); border: 1px solid var(--color-border-dark); color: white;">
                            <h4 style="color: white; margin-bottom: 1.5rem;">"Verification Summary"</h4>

                            <div style="margin-bottom: 1.5rem;">
                                <div style="font-size: 0.85rem; color: var(--color-text-sub); margin-bottom: 0.25rem;">"Identity Verified"</div>
                                <div style="display: flex; align-items: center; gap: 0.5rem;">
                                    <span style="color: var(--color-accent);">"✔"</span> "Gov ID + Video Interview"
                                </div>
                            </div>

                            <div style="margin-bottom: 1.5rem;">
                                <div style="font-size: 0.85rem; color: var(--color-text-sub); margin-bottom: 0.25rem;">"Pedagogy Review"</div>
                                <div style="display: flex; align-items: center; gap: 0.5rem;">
                                    <span style="color: var(--color-accent);">"✔"</span> "Classroom Audit (Pass)"
                                </div>
                            </div>

                             <div style="margin-bottom: 2rem;">
                                <div style="font-size: 0.85rem; color: var(--color-text-sub); margin-bottom: 0.25rem;">"Last Audit"</div>
                                <div style="font-family: var(--font-mono);">"2026-01-10"</div>
                            </div>

                            <a href="#" class="btn btn-primary" style="width: 100%; text-align: center;">"Contact Trainer"</a>
                        </div>

                        <div style="margin-top: 2rem; padding: 1.5rem; border: 1px solid var(--color-border-dark); border-radius: 8px;">
                            <h5 style="color: var(--color-text-sub); font-size: 0.9rem; margin-bottom: 1rem;">"Governance Notes"</h5>
                            <p style="font-size: 0.85rem; color: var(--color-text-sub); font-style: italic;">
                                "No active disputes. Good standing with the Standards Committee."
                            </p>
                        </div>
                    </aside>
                </div>
            </div>
        </section>
    }
}

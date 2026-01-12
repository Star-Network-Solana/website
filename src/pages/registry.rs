use leptos::*;

#[derive(Clone)]
struct Trainer {
    id: u32,
    name: &'static str,
    focus: &'static str,
    exp_years: u8,
    verified: bool,
    last_review: &'static str,
}

#[component]
pub fn Registry() -> impl IntoView {
    // Mock Data
    let trainers = vec![
        Trainer {
            id: 1,
            name: "Dr. Elena Vance",
            focus: "Rust Pedagogy & Safety",
            exp_years: 8,
            verified: true,
            last_review: "2026-01-10",
        },
        Trainer {
            id: 2,
            name: "Marcus Thorne",
            focus: "Solana Protocol Engineering",
            exp_years: 5,
            verified: true,
            last_review: "2025-12-05",
        },
        Trainer {
            id: 3,
            name: "Sarah Chen",
            focus: "ZK Compression & Math",
            exp_years: 4,
            verified: true,
            last_review: "2026-01-02",
        },
        Trainer {
            id: 4,
            name: "David Okonjo",
            focus: "Enterprise Systems",
            exp_years: 10,
            verified: true,
            last_review: "2025-11-20",
        },
        Trainer {
            id: 5,
            name: "Alex V.",
            focus: "Security Auditing",
            exp_years: 6,
            verified: true,
            last_review: "2025-12-15",
        },
        Trainer {
            id: 6,
            name: "Project Luminous",
            focus: "Bootcamp Provider",
            exp_years: 3,
            verified: false,
            last_review: "Pending",
        },
    ];

    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                // Section 1: Intro
                <div style="margin-bottom: 4rem;">
                    <h1>"Verified Educator Registry"</h1>
                    <p style="font-size: 1.1rem; max-width: 800px; color: var(--color-text-sub);">
                        "The STAR Registry documents verified Solana and Rust educators based on real experience, human review, and continuous signals. Neutral. Transparent. Immutable."
                    </p>
                </div>

                <div style="display: grid; grid-template-columns: 250px 1fr; gap: 4rem; align-items: start;">
                    // Section 2: Filter Panel
                    <aside style="background: var(--color-surface); padding: 1.5rem; border: 1px solid var(--color-border); border-radius: 8px;">
                        <h4 style="margin-bottom: 1.5rem; color: white;">"Filters"</h4>

                        <div style="display: flex; flex-direction: column; gap: 2rem;">

                            // Skill Focus
                            <div>
                                <h5 style="color: var(--color-text-sub); font-size: 0.85rem; text-transform: uppercase; margin-bottom: 0.75rem;">"Skill Focus"</h5>
                                <div style="display: flex; flex-direction: column; gap: 0.5rem; color: var(--color-text-inv);">
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "Rust Core"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "Solana Protocol"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "Security"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "Zk/Math"
                                    </label>
                                </div>
                            </div>

                            // Experience
                            <div>
                                <h5 style="color: var(--color-text-sub); font-size: 0.85rem; text-transform: uppercase; margin-bottom: 0.75rem;">"Experience"</h5>
                                <div style="display: flex; flex-direction: column; gap: 0.5rem; color: var(--color-text-inv);">
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "1-3 Years"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "4-6 Years"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "7+ Years"
                                    </label>
                                </div>
                            </div>

                             // Status
                             <div>
                                <h5 style="color: var(--color-text-sub); font-size: 0.85rem; text-transform: uppercase; margin-bottom: 0.75rem;">"Status"</h5>
                                <div style="display: flex; flex-direction: column; gap: 0.5rem; color: var(--color-text-inv);">
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" checked=true /> "Verified"
                                    </label>
                                    <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
                                        <input type="checkbox" /> "Pending Review"
                                    </label>
                                </div>
                            </div>

                        </div>
                    </aside>

                    // Section 3: Trainer Grid
                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 1.5rem;">
                        {trainers.into_iter().map(|trainer| {
                            view! {
                                <a href=format!("/registry/{}", trainer.id) class="card" style="text-decoration: none; display: block; position: relative; overflow: hidden;">
                                    <div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;">
                                        <h3 style="font-size: 1.25rem; margin: 0; color: white;">{trainer.name}</h3>
                                        {if trainer.verified {
                                            view! { <span style="background: rgba(20, 241, 149, 0.1); color: var(--color-accent); padding: 2px 8px; border-radius: 4px; font-size: 0.75rem; border: 1px solid rgba(20, 241, 149, 0.2);">"VERIFIED"</span> }
                                        } else {
                                            view! { <span style="background: rgba(255, 255, 255, 0.05); color: var(--color-text-sub); padding: 2px 8px; border-radius: 4px; font-size: 0.75rem; border: 1px solid var(--color-border-dark);">"PENDING"</span> }
                                        }}
                                    </div>

                                    <div style="margin-bottom: 1.5rem;">
                                        <div style="font-size: 0.85rem; color: var(--color-text-sub); margin-bottom: 0.25rem;">"Focus"</div>
                                        <div style="color: var(--color-text-inv); font-weight: 500;">{trainer.focus}</div>
                                    </div>

                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; font-size: 0.85rem;">
                                        <div>
                                            <div style="color: var(--color-text-sub); margin-bottom: 0.25rem;">"Experience"</div>
                                            <div style="color: white;">{trainer.exp_years} " Years"</div>
                                        </div>
                                         <div>
                                            <div style="color: var(--color-text-sub); margin-bottom: 0.25rem;">"Last Review"</div>
                                            <div style="color: white; font-family: var(--font-mono); font-size: 0.8rem;">{trainer.last_review}</div>
                                        </div>
                                    </div>

                                    // Hover effect overlay could go here, but using CSS .card:hover
                                </a>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}

use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trainer {
    pub id: u32,
    pub name: String,
    pub focus: String,
    pub exp_years: u8,
    pub verified: bool,
    pub last_review: String,
    pub governance_notes: String,
    pub status: String,
}

#[component]
pub fn Registry() -> impl IntoView {
    // Embedded Static Data (Audit Link: public/trainers.json)
    let trainers_json = include_str!("../../public/trainers.json");
    let trainers: Vec<Trainer> =
        serde_json::from_str(trainers_json).expect("Failed to load static registry data");

    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                // Section 1: Intro
                <div style="margin-bottom: 4rem;">
                    <h1>"Verified Educator Registry"</h1>
                    <p style="font-size: 1.1rem; max-width: 800px; color: var(--color-text-sub);">
                        "The STAR Registry documents Solana and Rust educators based on publicly verifiable experience, human review, and transparent criteria. Registry inclusion reflects observed expertise and teaching background — it is not a license, endorsement, or guarantee."
                    </p>
                    <div style="background: rgba(255, 165, 0, 0.1); border-left: 4px solid orange; padding: 1rem; margin-top: 1rem; margin-bottom: 2rem;">
                         <p style="color: orange; font-weight: bold; margin-bottom: 0.5rem;">"Important:"</p>
                         <p style="color: var(--color-text-sub); margin: 0;">
                            "The STAR Registry is a verification and discovery tool. It does not represent formal accreditation, employment endorsement, or official certification."
                         </p>
                    </div>
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

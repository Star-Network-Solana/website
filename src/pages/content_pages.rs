use leptos::*;

#[component]
pub fn HowItWorks() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                <h1 style="margin-bottom: 2rem;">"How STAR Works"</h1>

                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4rem; margin-bottom: 4rem;">
                    <div>
                        <h3>"Why Trust is Broken"</h3>
                         <p>"STAR Network exists to address a fundamental trust gap in Solana and Rust education: the lack of a neutral, verifiable way to assess educator quality and real-world teaching experience."</p>
                    </div>
                </div>

                <div style="margin-bottom: 4rem;">
                    <h3>"The Verification Process"</h3>
                    <div style="border: 1px solid var(--color-border); padding: 2rem; border-radius: 8px;">
                        <ol style="font-size: 1.1rem; line-height: 2;">
                           <li>"Identity Verification (KYC/Video)"</li>
                           <li>"Pedagogy Audit (Curriculum Review)"</li>
                           <li>"Technical Assessment (Code Review)"</li>
                           <li>"Continuous Signals (Student Feedback/On-chain Reputation)"</li>
                        </ol>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn Standards() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                <div style="margin-bottom: 2rem; background: rgba(255, 255, 255, 0.05); padding: 0.5rem 1rem; display: inline-block; border-radius: 4px;">
                     <span style="color: var(--color-accent); font-weight: 500;">"Status: Frameworks in development — not yet formal standards."</span>
                </div>
                <h1 style="margin-bottom: 2rem;">"Quality Frameworks & Standards (In Development)"</h1>

                <div style="margin-bottom: 4rem;">
                    <h3>"Definition of Quality"</h3>
                    <p>"STAR Network is developing shared quality frameworks to support consistent, transparent evaluation of Solana and Rust education. These frameworks are informed by ongoing registry data, educator feedback, and ecosystem needs, and will evolve over time."</p>
                </div>

                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4rem;">
                    <div style="background: var(--color-surface); padding: 2rem; color: white; border-radius: 8px;">
                        <h4 style="color: var(--color-accent); margin-bottom: 1rem;">"What We Measure"</h4>
                        <ul style="line-height: 1.6;">
                            <li>"Depth of Rust ownership knowledge"</li>
                            <li>"Security best practices (PDA validation, etc.)"</li>
                            <li>"Ability to explain complex concepts clearly"</li>
                        </ul>
                    </div>
                     <div style="background: rgba(255,255,255,0.05); padding: 2rem; border-radius: 8px;">
                        <h4 style="color: var(--color-text-sub); margin-bottom: 1rem;">"What We Do Not Measure"</h4>
                        <ul style="line-height: 1.6;">
                            <li>"Price of courses"</li>
                            <li>"Marketing claims"</li>
                            <li>"Platform popularity"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn Educators() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
             <div class="container">
                <h1 style="margin-bottom: 1rem;">"For Educators"</h1>
                <p style="font-size: 1.2rem; color: var(--color-text-sub); margin-bottom: 2rem; max-width: 800px;">
                    "STAR Network provides educators with a neutral way to make their real teaching and technical experience visible — without exclusivity or loss of independence."
                </p>
                 <div style="background: rgba(100, 149, 237, 0.1); border: 1px solid rgba(100, 149, 237, 0.3); padding: 1.5rem; border-radius: 8px; margin-bottom: 4rem;">
                    <p style="color: white; margin: 0;">"Participation in STAR Network does not limit independent work, pricing, or affiliations."</p>
                </div>

                <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 2rem; margin-bottom: 4rem;">
                   <div style="border: 1px solid var(--color-border); padding: 2rem;">
                       <h3>"High-Signal Visibility"</h3>
                       <p>"Stand out from bootcamps with verified credentials."</p>
                       // DEFERRED: Trainer dashboard and analytics deferred to v2.
                   </div>
                   <div style="border: 1px solid var(--color-border); padding: 2rem;">
                        <h3>"Independence Guarantees"</h3>
                        <p>"We verify your quality, but you own your content and students."</p>
                   </div>
                   <div style="border: 1px solid var(--color-border); padding: 2rem;">
                        <h3>"Enterprise Access"</h3>
                        <p>"Direct pipeline to major ecosystem hiring partners."</p>
                   </div>
                </div>

                <div style="text-align: center; padding: 4rem 0; background: var(--color-surface); border-radius: 8px;">
                     <h3 style="color: white; margin-bottom: 2rem;">"Ready to Verify?"</h3>
                     <a href="#" target="_blank" rel="noopener noreferrer" class="btn btn-primary" style="font-size: 1.1rem; padding: 1rem 2rem;">"Request Registry Review"</a>
                </div>
             </div>
        </section>
    }
}

#[component]
pub fn Enterprises() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                <h1 style="margin-bottom: 2rem;">"For Enterprises"</h1>

                <div style="margin-bottom: 4rem;">
                     <h3>"Why Training Fails"</h3>
                     <p>"STAR Network helps enterprises reduce risk by making educator experience and verification criteria transparent — not by replacing due diligence."</p>
                </div>

                <div style="margin-bottom: 4rem;">
                    <h3>"The STAR Guarantee"</h3>
                    <p>"STAR Network does not guarantee outcomes, performance, or suitability for specific use cases."</p>
                </div>

                <a href="#" target="_blank" rel="noopener noreferrer" class="btn btn-primary">"Request Vetted Educators"</a>
            </div>
        </section>
    }
}

#[component]
pub fn Insights() -> impl IntoView {
    view! {
          <section style="padding-top: 4rem;">
            <div class="container">
                <h1 style="margin-bottom: 2rem;">"Insights"</h1>
                <p style="font-size: 1.1rem; color: var(--color-text-sub); margin-bottom: 4rem; max-width: 800px;">
                    "STAR Network publishes research, analysis, and perspectives on Solana and Rust education, pedagogy, and ecosystem skill gaps."
                </p>

                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 2rem;">
                    <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Pedagogy"</div>
                        <h3 style="margin-bottom: 1rem;">"Teaching Borrow Checker Intuitively"</h3>
                        <p>"A new approach to explaining ownership without fighting the compiler."</p>
                    </div>
                     <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Ecosystem"</div>
                        <h3 style="margin-bottom: 1rem;">"The 2026 Rust Developer Gap"</h3>
                        <p>"Analysis of the current shortage in senior protocol engineers."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
             <div class="container">
                <div style="margin-bottom: 4rem; max-width: 800px;">
                    <h1 style="margin-bottom: 1.5rem;">"About STAR Network"</h1>
                    <p style="font-size: 1.1rem;">"STAR Network is an independent professional association initiative formed to address trust, verification, and quality gaps in Solana and Rust education."</p>

                    <div style="margin-top: 2rem; padding: 2rem; background: rgba(255, 255, 255, 0.03); border-radius: 8px;">
                        <h4 style="margin-bottom: 1rem; color: white;">"Status & Scope"</h4>
                        <ul style="list-style: none; padding: 0; line-height: 1.8; color: var(--color-text-sub);">
                            <li><strong style="color: var(--color-text-inv);">"Current Status:"</strong> " Independent professional association initiative"</li>
                            <li><strong style="color: var(--color-text-inv);">"Legal Structure:"</strong> " In formation"</li>
                            <li><strong style="color: var(--color-text-inv);">"Primary Focus (2026):"</strong> " Trainer registry, verification frameworks, and quality standards development"</li>
                            <li><strong style="color: var(--color-text-inv);">"Accreditation Authority:"</strong> " Emerging (future-state)"</li>
                            <li><strong style="color: var(--color-text-inv);">"Commercial Interest:"</strong> " Neutral, trainer-first, non-exclusive"</li>
                        </ul>
                    </div>
                </div>

                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4rem; margin-bottom: 4rem;">
                    <div>
                        <h3>"Coordination Structure"</h3>
                        <p>"Facilitated by a council of educators and core engineers. Decisions are public and open for transparent review."</p>
                        // DEFERRED: On-chain governance voting modules deferred until institutional maturity.
                    </div>
                    <div>
                         <h3>"Phased Roadmap"</h3>
                         <ul style="line-height: 1.8;">
                            <li>"Phase 1: Registry (Current)"</li>
                            <li>"Phase 2: Accreditation Standard"</li>
                            <li>"Phase 3: Decentralized Verification"</li>
                         </ul>
                    </div>
                </div>

                <div style="border-top: 1px solid var(--color-border); padding-top: 2rem;">
                     <h3>"Conflict of Interest Policy"</h3>
                     <p style="color: var(--color-text-sub);">"STAR Network does not run its own courses. We do not compete with registry members. We exist solely to verify."</p>
                </div>
             </div>
        </section>
    }
}

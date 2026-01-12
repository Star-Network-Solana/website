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
                         <p>"The current education ecosystem is fragmented. Verifying quality requires manual effort, and certificates are easily forged or become outdated."</p>
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
                <h1 style="margin-bottom: 2rem;">"Standards & Accreditation"</h1>

                <div style="margin-bottom: 4rem;">
                    <h3>"Definition of Quality"</h3>
                    <p>"Educational quality is defined by factual accuracy, pedagogical effectiveness, and safety-first coding practices."</p>
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
                <p style="font-size: 1.2rem; color: var(--color-text-sub); margin-bottom: 4rem; max-width: 800px;">
                    "Join the registry to prove your expertise and gain access to enterprise training opportunities."
                </p>

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
                     <p>"Most internal training programs lack structure, and hiring external trainers is a gamble on quality."</p>
                </div>

                <div style="margin-bottom: 4rem;">
                    <h3>"The STAR Guarantee"</h3>
                    <p>"Every verified educator has passed rigorous technical and pedagogical audits. We reduce your risk to zero."</p>
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
                <h1 style="margin-bottom: 4rem;">"Insights"</h1>

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
                    <p style="font-size: 1.1rem;">"We are a non-profit standards body dedicated to the professionalization of the Solana and Rust education sector."</p>
                </div>

                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4rem; margin-bottom: 4rem;">
                    <div>
                        <h3>"Governance Structure"</h3>
                        <p>"Operated by an elected council of educators and core engineers. Decisions are public and open for transparent review."</p>
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

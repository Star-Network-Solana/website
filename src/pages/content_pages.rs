use leptos::*;

#[component]
pub fn HowItWorks() -> impl IntoView {
    view! {
        <section style="padding-top: 4rem;">
            <div class="container">
                <h1 style="margin-bottom: 2rem;">"How STAR Works"</h1>

                <div class="grid-responsive-2" style="margin-bottom: 4rem;">
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

                <div class="grid-responsive-2">
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

                <div class="grid-responsive-3" style="margin-bottom: 4rem;">
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
                     <a href="https://discord.gg/CNHQKDPm" target="_blank" rel="noopener noreferrer" class="btn btn-primary" style="font-size: 1.1rem; padding: 1rem 2rem;">"Request Registry Review"</a>
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

                <a href="https://discord.gg/CNHQKDPm" target="_blank" rel="noopener noreferrer" class="btn btn-primary">"Request Vetted Educators"</a>
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
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Quality Standards"</div>
                        <h3 style="margin-bottom: 1rem;">"Education Quality in Rust"</h3>
                        <p style="font-size: 0.95rem; line-height: 1.6;">"What defines high-quality Rust education, and why consistency matters for long-term ecosystem adoption. A look at common gaps between learning formats and real-world requirements."</p>
                    </div>
                    <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Trust"</div>
                        <h3 style="margin-bottom: 1rem;">"Accreditation & Trust in Technical Training"</h3>
                        <p style="font-size: 0.95rem; line-height: 1.6;">"Why informal reputation is not enough at scale. An introduction to how structured accreditation helps make training credibility visible and verifiable."</p>
                    </div>
                    <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Pedagogy"</div>
                        <h3 style="margin-bottom: 1rem;">"Evaluating Trainer Capability Fairly"</h3>
                        <p style="font-size: 0.95rem; line-height: 1.6;">"Teaching Rust well requires more than technical expertise alone. This topic explores balanced approaches to evaluating educators without gatekeeping."</p>
                    </div>
                    <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Skills"</div>
                        <h3 style="margin-bottom: 1rem;">"Learning Pathways for Production-Ready Skills"</h3>
                        <p style="font-size: 0.95rem; line-height: 1.6;">"How structured learning pathways support progression from fundamentals to real-world Rust use. Focuses on alignment with production environments rather than tutorial completion."</p>
                    </div>
                    <div class="content-card">
                        <div style="font-size: 0.8rem; color: var(--color-text-sub); margin-bottom: 0.5rem; text-transform: uppercase;">"Risk"</div>
                        <h3 style="margin-bottom: 1rem;">"Education as Ecosystem Infrastructure"</h3>
                        <p style="font-size: 0.95rem; line-height: 1.6;">"Why education quality directly affects ecosystem risk, funding efficiency, and adoption speed. Examines training as a foundational layer, not a peripheral activity."</p>
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

                // Founders Profile Section
                <div style="margin-bottom: 4rem;">
                    <h2 style="margin-bottom: 2rem; color: var(--color-accent);">"Founders Profile"</h2>
                    <div class="grid-responsive-2" style="gap: 3rem;">
                        // Amy Thomason
                        <div style="background: rgba(255,255,255,0.02); padding: 2rem; border-radius: 8px; border: 1px solid var(--color-border-dark);">
                            <h3 style="color: white; margin-bottom: 0.5rem;">"Amy Thomason"</h3>
                            <p style="color: var(--color-accent); font-weight: 500; font-size: 0.9rem; margin-bottom: 1.5rem; text-transform: uppercase; letter-spacing: 0.05em;">"Founder & Lead Educator"</p>
                            <p style="font-size: 0.95rem; line-height: 1.7; color: var(--color-text-sub); margin-bottom: 0;">
                                "Amy Thomason is a senior Rust educator and systems programmer with 30+ years of experience across game engines, compilers, and blockchain infrastructure. They have taught Rust to both academic and corporate audiences, led long-running Rust meetups including Oxford ACCU/Rust, and is a published technical author. Their background includes blockchain node development, decentralized systems architecture, and former leadership as MSc faculty in games development. At STAR Network, Amy defines education standards, trainer quality benchmarks, and the Train-the-Trainer pathway."
                            </p>
                        </div>
                        // Gaveesh Jain
                        <div style="background: rgba(255,255,255,0.02); padding: 2rem; border-radius: 8px; border: 1px solid var(--color-border-dark);">
                            <h3 style="color: white; margin-bottom: 0.5rem;">"Gaveesh Jain"</h3>
                            <p style="color: var(--color-accent); font-weight: 500; font-size: 0.9rem; margin-bottom: 1.5rem; text-transform: uppercase; letter-spacing: 0.05em;">"Founder & Community Builder"</p>
                            <p style="font-size: 0.95rem; line-height: 1.7; color: var(--color-text-sub); margin-bottom: 0;">
                                "Gaveesh Jain is a startup founder with 9+ years of experience building and scaling ventures across fintech, healthcare, and Web3. He holds an MBA in Innovation and has worked extensively in business development, partnerships, and platform strategy. Gaveesh has conducted deep, hands-on research across the Solana ecosystem through developer events, community engagement, and direct collaboration with builders and educators. At STAR Network, he focuses on ecosystem partnerships, community growth, and building sustainable infrastructure for Solana education."
                            </p>
                        </div>
                    </div>
                </div>

                <div class="grid-responsive-2" style="margin-bottom: 4rem;">
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
                     <p style="color: var(--color-text-sub); margin-bottom: 1.5rem;">
                        "STAR Network does not operate as a commercial bootcamp or content marketplace and does not compete with educators listed in its registry for independent training work. STAR’s role is to define education standards, verify trainer quality, and coordinate trust within the Solana (Rust) education ecosystem."
                     </p>
                     <p style="color: var(--color-text-sub);">
                        "STAR may deliver Train-the-Trainer programs, benchmarking tests, and certification assessments solely for the purpose of quality assurance and standards alignment. These activities are not substitutes for commercial training programs and are designed to support educators, not replace them. Trainers remain independent and are free to offer their own courses and services outside of STAR."
                     </p>
                </div>
             </div>
        </section>
    }
}

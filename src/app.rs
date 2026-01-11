use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Hero />
            <ProblemSolution />
            <Capabilities />
            <TrustSignals />
            <Audience />
            <Mission />
            <CtaBand />
        </main>
        <Footer />
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <div class="container">
                <div class="logo">
                    <a href="/website/">"STAR Network"</a>
                </div>
                <nav class="primary-nav">
                    <ul>
                        <li><a href="#standards">"Standards"</a></li>
                        <li><a href="#registry">"Registry"</a></li>
                        <li><a href="#how-it-works">"How It Works"</a></li>
                        <li><a href="#audience">"Who It’s For"</a></li>
                        <li><a href="#governance">"Governance"</a></li>
                    </ul>
                </nav>
                <div class="utility-nav">
                    <a href="#apply" class="btn btn-primary" style="padding: 0.5rem 1rem; font-size: 0.9rem;">"Apply as Trainer"</a>
                    <a href="#registry-view" class="btn btn-secondary" style="padding: 0.5rem 1rem; font-size: 0.9rem;">"View Registry"</a>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section id="hero">
            <div class="container">
                <h1>"The Trust Layer for Rust Education"</h1>
                <p class="hero-subtext">"Accreditation, verification, and open standards that make Rust training credible, transparent, and reliable."</p>

                <div class="callout">
                    <p><strong>"Note: "</strong> "Built for ecosystems, institutions, trainers, and learners who need confidence—not guesswork. STAR Network does not provide courses directly; it provides the standards and verification infrastructure."</p>
                </div>

                <div class="cta-group">
                    <a href="#standards" class="btn btn-primary">"Explore Standards"</a>
                    <a href="#registry" class="btn btn-secondary">"View Trainer Registry"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ProblemSolution() -> impl IntoView {
    view! {
        <section id="problem-solution">
            <div class="container">
                <hr />
                <div class="text-grid">
                    <div class="col">
                        <h2>"The Problem"</h2>
                        <ul>
                            <li>"Rust training quality varies widely."</li>
                            <li>"Credentials are unclear."</li>
                            <li>"Organizers struggle to verify expertise, and learners can’t easily judge credibility."</li>
                        </ul>
                    </div>
                    <div class="col">
                        <h2>"The Solution"</h2>
                        <ul>
                            <li>"STAR Network introduces a neutral trust layer for Rust education—making trainer quality visible, verifiable, and repeatable."</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Capabilities() -> impl IntoView {
    view! {
        <section id="capabilities">
            <div class="container">
                <hr />
                <h2>"What STAR Network Provides"</h2>
                <div class="text-grid">
                    <div class="list-item">
                        <h3>"Accreditation Standards"</h3>
                        <p>"Clear criteria, evaluation rubrics, and review processes that define what credible Rust training looks like."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Verified Trainer Registry"</h3>
                        <p>"A transparent, searchable registry of trainers whose credentials and experience have been reviewed."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Learning Pathways"</h3>
                        <p>"Structured, production-aligned skill journeys that reflect real-world Rust usage."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Review & Improvement Loop"</h3>
                        <p>"Outcome tracking and continuous updates to ensure standards stay relevant as the ecosystem evolves."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TrustSignals() -> impl IntoView {
    view! {
        <section id="trust-signals">
            <div class="container">
                <hr />
                <h2>"How Trust Is Established"</h2>
                <ul class="trust-signals-list">
                    <li>"Open, publicly documented standards"</li>
                    <li>"Transparent governance and decision-making"</li>
                    <li>"Neutral, ecosystem-aligned positioning"</li>
                    <li>"Ongoing review and improvement"</li>
                </ul>
            </div>
        </section>
    }
}

#[component]
fn Audience() -> impl IntoView {
    view! {
        <section id="audience">
            <div class="container">
                <hr />
                <h2>"Who STAR Network Is For"</h2>
                <div class="text-grid">
                    <div class="list-item">
                        <h3>"Foundations & Program Owners"</h3>
                        <p>"Reduce ecosystem risk and ensure consistent, high-quality training outcomes."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Training Organizations & Communities"</h3>
                        <p>"Move from informal workshops to credible, trusted programs."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Trainers"</h3>
                        <p>"Gain fair recognition, visibility, and access to better opportunities."</p>
                    </div>
                    <div class="list-item">
                        <h3>"Learners"</h3>
                        <p>"Learn with confidence through consistent, production-ready education."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Mission() -> impl IntoView {
    view! {
        <section id="mission">
            <div class="container" style="text-align: center; margin-top: 4rem;">
                <hr />
                <p style="font-size: 1.2rem; font-weight: 500; color: var(--text-primary);">
                    "Rust adoption depends on education quality. STAR Network was created to ensure that those who teach Rust are trusted, prepared, and aligned with real-world needs—so ecosystems can scale responsibly."
                </p>
            </div>
        </section>
    }
}

#[component]
fn CtaBand() -> impl IntoView {
    view! {
        <section id="cta-band" style="background: var(--bg-color); border-top: 1px solid var(--border-color); margin-top: 6rem; padding: 6rem 1.5rem; text-align: center;">
            <div class="container" style="max-width: 700px;">
                <h2 style="border-bottom: none; margin-bottom: 2.5rem; font-size: 2rem;">"Engage with STAR Network"</h2>
                <div style="display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;">
                    <a href="#apply" class="btn btn-secondary" style="background: transparent; padding: 0.8rem 1.5rem; font-size: 1rem;">"Apply as a Trainer"</a>
                    <a href="#request" class="btn btn-secondary" style="background: transparent; padding: 0.8rem 1.5rem; font-size: 1rem;">"Request an Accredited Trainer"</a>
                    <a href="#standards" class="btn btn-secondary" style="background: transparent; padding: 0.8rem 1.5rem; font-size: 1rem;">"Explore Standards"</a>
                    <a href="#partner" class="btn btn-secondary" style="background: transparent; padding: 0.8rem 1.5rem; font-size: 1rem;">"Join as a Partner"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-row">
                <div class="footer-col" style="max-width: 300px;">
                    <h4>"STAR Network"</h4>
                    <p style="font-size: 0.9rem; margin: 0; color: var(--text-secondary);">"The Trust Layer for Rust Education."</p>
                </div>
                <div class="footer-col">
                    <h4>"Standards"</h4>
                    <ul>
                        <li><a href="#standards">"Protocol"</a></li>
                        <li><a href="#registry">"Registry"</a></li>
                        <li><a href="#governance">"Governance"</a></li>
                    </ul>
                </div>
                <div class="footer-col">
                    <h4>"Connect"</h4>
                    <ul>
                        <li><a href="https://github.com/Star-Network-Solana">"GitHub"</a></li>
                        <li><a href="#">"Contact"</a></li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}

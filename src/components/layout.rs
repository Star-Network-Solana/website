use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header style="
            position: sticky; 
            top: 0; 
            z-index: 50; 
            background: rgba(11, 16, 38, 0.95); /* Deep Space Navy with slight transparency */
            backdrop-filter: blur(8px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            height: var(--header-height);
        ">
            <div class="container" style="height: 100%; display: flex; justify-content: space-between; align-items: center;">
                // Left: Text Logo
                <div class="logo">
                    <a href="/" style="
                        font-family: var(--font-heading);
                        font-weight: 700; 
                        font-size: 1.25rem; 
                        letter-spacing: -0.01em; 
                        color: white;
                        text-decoration: none;
                    ">
                        "STAR NETWORK"
                    </a>
                </div>

                // Center: Navigation
                <nav class="desktop-nav" style="display: flex; gap: 2rem;">
                    <a href="/registry" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Registry"</a>
                    <a href="/how-it-works" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"How STAR Works"</a>
                    <a href="/standards" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Standards"</a>
                    <a href="/educators" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"For Educators"</a>
                    <a href="/enterprises" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"For Enterprises"</a>
                    // Insights and About can be added or prioritized based on space, for now including all as requested
                    <a href="/insights" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Insights"</a>
                    <a href="/about" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"About"</a>
                </nav>

                // Right: Request Access
                <div class="cta-nav">
                    <a href="/educators" class="btn btn-outline" style="text-decoration: none;">
                        "Request Access"
                    </a>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer style="
            background: var(--color-primary); 
            padding: 4rem 0; 
            margin-top: auto; 
            border-top: 1px solid var(--color-border-dark);
            color: var(--color-text-sub);
        ">
            <div class="container">
                <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 4rem;">

                    // Brand Column
                    <div style="flex: 1; min-width: 250px;">
                        <h4 style="color: white; margin-bottom: 1rem;">"STAR Network"</h4>
                        <p style="font-size: 0.9rem; line-height: 1.6; max-width: 320px; color: var(--color-text-sub);">
                            "The institutional trust layer for Solana & Rust education. Establishing neutral verification and open standards for the global ecosystem."
                        </p>
                    </div>

                    // Links Column
                    <div style="display: flex; gap: 4rem;">
                        <div>
                            <h5 style="color: white; font-size: 0.9rem; letter-spacing: 0.05em; margin-bottom: 1.5rem;">"Governance"</h5>
                            <div style="display: flex; flex-direction: column; gap: 0.75rem; font-size: 0.9rem;">
                                <a href="/transparency" style="color: var(--color-text-sub);">"Transparency"</a>
                                <a href="/governance" style="color: var(--color-text-sub);">"Governance Structure"</a>
                                <a href="/ethics" style="color: var(--color-text-sub);">"Code of Ethics"</a>
                            </div>
                        </div>

                        <div>
                            <h5 style="color: white; font-size: 0.9rem; letter-spacing: 0.05em; margin-bottom: 1.5rem;">"Legal & Contact"</h5>
                            <div style="display: flex; flex-direction: column; gap: 0.75rem; font-size: 0.9rem;">
                                <a href="/contact" style="color: var(--color-text-sub);">"Contact"</a>
                                <a href="/legal" style="color: var(--color-text-sub);">"Legal"</a>
                                <a href="/privacy" style="color: var(--color-text-sub);">"Privacy Policy"</a>
                            </div>
                        </div>
                    </div>
                </div>

                <div style="margin-top: 4rem; padding-top: 2rem; border-top: 1px solid rgba(255,255,255,0.05); text-align: left; font-size: 0.85rem;">
                    "© 2026 STAR Network. A Non-Profit Standards Body."
                </div>
            </div>
        </footer>
    }
}

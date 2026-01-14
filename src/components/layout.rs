use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    // Signal for mobile menu state
    let (menu_open, set_menu_open) = create_signal(false);

    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };

    view! {
        <header style="
            position: sticky; 
            top: 0; 
            z-index: 50; 
            background: rgba(11, 16, 38, 0.95); /* Deep Space Navy with slight transparency */
            backdrop-filter: blur(8px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            min-height: var(--header-height);
        ">
            <div class="container" style="height: var(--header-height); display: flex; justify-content: space-between; align-items: center;">
                // Left: Text Logo
                <div class="logo">
                    <a href="/website/" style="
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

                // Mobile: Hamburger Menu Button
                <button
                    class="desktop-hide"
                    on:click=toggle_menu
                    style="
                        background: none;
                        border: none;
                        color: white;
                        font-size: 1.5rem;
                        cursor: pointer;
                        padding: 0.5rem;
                        display: flex;
                        align-items: center;
                        z-index: 60;
                    "
                    aria-label="Toggle menu"
                >
                    {move || if menu_open.get() { "✕" } else { "☰" }}
                </button>

                // Desktop: Navigation
                <nav class="mobile-hide" style="display: flex; gap: 2rem; align-items: center;">
                    <a href="/website/registry" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Registry"</a>
                    <a href="/website/how-it-works" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"How STAR Works"</a>
                    <a href="/website/standards" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Standards"</a>
                    <a href="/website/educators" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"For Educators"</a>
                    <a href="/website/enterprises" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"For Enterprises"</a>
                    <a href="/website/insights" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"Insights"</a>
                    <a href="/website/about" style="color: var(--color-text-inv); font-size: 0.9rem; font-weight: 500;">"About"</a>

                    <a href="/website/educators" class="btn btn-outline" style="text-decoration: none;">
                        "Request Access"
                    </a>
                </nav>
            </div>

            // Mobile: Slide-down Menu
            <div
                class="desktop-hide"
                style=move || format!(
                    "position: absolute; \
                    top: var(--header-height); \
                    left: 0; \
                    right: 0; \
                    background: rgba(11, 16, 38, 0.98); \
                    backdrop-filter: blur(10px); \
                    border-bottom: 1px solid rgba(255, 255, 255, 0.1); \
                    transform: translateY({}); \
                    opacity: {}; \
                    transition: all 0.3s ease; \
                    pointer-events: {}; \
                    max-height: calc(100vh - var(--header-height)); \
                    overflow-y: auto;",
                    if menu_open.get() { "0" } else { "-100%" },
                    if menu_open.get() { "1" } else { "0" },
                    if menu_open.get() { "auto" } else { "none" }
                )
            >
                <nav style="display: flex; flex-direction: column; padding: 2rem 1rem;">
                    <a href="/website/registry" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"Registry"</a>
                    <a href="/website/how-it-works" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"How STAR Works"</a>
                    <a href="/website/standards" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"Standards"</a>
                    <a href="/website/educators" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"For Educators"</a>
                    <a href="/website/enterprises" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"For Enterprises"</a>
                    <a href="/website/insights" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"Insights"</a>
                    <a href="/website/about" style="color: var(--color-text-inv); font-size: 1rem; font-weight: 500; padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05);" on:click=move |_| set_menu_open.set(false)>"About"</a>

                    <a href="/website/educators" class="btn btn-outline" style="text-decoration: none; margin-top: 1rem;" on:click=move |_| set_menu_open.set(false)>
                        "Request Access"
                    </a>
                </nav>
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
                            "An independent professional association initiative building trust and verification frameworks for Solana and Rust education."
                        </p>
                    </div>

                    // Links Column
                    <div style="display: flex; gap: 4rem;">
                        <div>
                            <h5 style="color: white; font-size: 0.9rem; letter-spacing: 0.05em; margin-bottom: 1.5rem;">"Coordination"</h5>
                            <div style="display: flex; flex-direction: column; gap: 0.75rem; font-size: 0.9rem;">
                                <a href="/website/transparency" style="color: var(--color-text-sub);">"Transparency"</a>
                                <a href="/website/governance" style="color: var(--color-text-sub);">"Coordination Structure"</a>
                                <a href="/website/ethics" style="color: var(--color-text-sub);">"Code of Ethics"</a>
                            </div>
                        </div>

                        <div>
                            <h5 style="color: white; font-size: 0.9rem; letter-spacing: 0.05em; margin-bottom: 1.5rem;">"Legal & Contact"</h5>
                            <div style="display: flex; flex-direction: column; gap: 0.75rem; font-size: 0.9rem;">
                                <a href="/website/contact" style="color: var(--color-text-sub);">"Contact"</a>
                                <a href="/website/legal" style="color: var(--color-text-sub);">"Legal"</a>
                                <a href="/website/privacy" style="color: var(--color-text-sub);">"Privacy Policy"</a>
                            </div>
                        </div>
                    </div>
                </div>

                <div style="margin-top: 4rem; padding-top: 2rem; border-top: 1px solid rgba(255,255,255,0.05); text-align: left; font-size: 0.85rem;">
                    "© 2026 STAR Network. Independent Professional Association Initiative."
                </div>
            </div>
        </footer>
    }
}

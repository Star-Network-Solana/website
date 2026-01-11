use leptos::*;

#[component]
pub fn Button(
    #[prop(into)] href: String,
    #[prop(into)] text: String,
    #[prop(optional, into)] variant: String, // "primary" or "secondary"
) -> impl IntoView {
    let class = if variant == "secondary" {
        "btn btn-secondary"
    } else {
        "btn btn-primary"
    };

    view! {
        <a href=href class=class>
            {text}
        </a>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">
            {children()}
        </div>
    }
}

#[component]
pub fn SectionBox(
    #[prop(into)] id: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section id=id class=class>
            <div class="container">
                {children()}
            </div>
        </section>
    }
}

#[component]
pub fn Badge(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <span class="badge" style="
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 9999px;
            font-size: 0.8rem;
            font-weight: 600;
            background: rgba(14, 165, 233, 0.1);
            color: var(--color-secondary);
            margin-bottom: 1rem;
            border: 1px solid rgba(14, 165, 233, 0.2);
        ">
            {text}
        </span>
    }
}

use leptos::*;

#[component]
pub fn CollapsableBox(
    cx: Scope,
    #[prop(into)]
    title: String,
    #[prop(optional)] open: bool,
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class="collapse collapse-arrow rounded-lg".to_owned() + &class.map_or("".to_owned(),|v| format!(" {v}"))>
            <input type="checkbox" class="peer" checked=open />
            <div class="collapse-title text-xl font-medium">
                {title}
            </div>
            <div class="collapse-content">
                {children(cx)}
            </div>
        </div>
    }
}

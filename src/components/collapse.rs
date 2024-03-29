use leptos::*;

#[component]
pub fn CollapsableBox(
    #[prop(into)] title: String,
    #[prop(optional)] open: bool,
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="collapse collapse-arrow rounded-lg".to_owned() + &class.map_or("".to_owned(),|v| format!(" {v}"))>
            <input type="checkbox" class="peer" checked=open />
            <div class="collapse-title text-xl font-medium">
                {title}
            </div>
            <div class="collapse-content">
                {children()}
            </div>
        </div>
    }
}

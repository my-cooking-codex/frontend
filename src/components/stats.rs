use leptos::*;

pub struct Stat {
    pub title: String,
    pub value: String,
    pub description: Option<String>,
}

impl Stat {
    pub fn new(title: &str, value: &str, description: Option<&str>) -> Self {
        Self {
            title: title.to_owned(),
            value: value.to_owned(),
            description: description.map(|s| s.to_owned()),
        }
    }
}

#[component]
pub fn Stats(cx: Scope, stats: Vec<Stat>) -> impl IntoView {
    view! {cx,
        <div class="stats stats-vertical sm:stats-horizontal shadow">
                {stats.into_iter().map(|stat| {
                    view! {cx,
                        <div class="stat place-items-center">
                            <div class="stat-title">{stat.title}</div>
                            <div class="stat-value">{stat.value}</div>
                            {
                                stat.description.as_ref().map(|description| view! {cx, <div class="stat-desc">{description}</div>})
                            }
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}

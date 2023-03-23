use crate::{
    components::{drawer::*, stats::*},
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::{api_error_to_toast, login_redirect_effect, LoginState},
};
use leptos::*;
use mcc_frontend_types::stats::AccountStats;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home", true),
        DrawerLink::new("/recipes/new", "New Recipe", false),
        DrawerLink::new("/recipes", "Recipes", false),
    ];

    let CurrentApi { api, .. } = use_api(cx);
    let toasts = use_toasts(cx);

    let account_stats = create_resource(
        cx,
        || {},
        move |_| async move {
            match api.get().unwrap().get_stats().await {
                Ok(stats) => Some(stats),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading stats"));
                    None
                }
            }
        },
    );

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    view! {cx,
        <Drawer links={drawer_links}>
            <div class="p-4 rounded bg-base-200">
                <h1 class="text-3xl font-bold mb-2">"Home"</h1>
                <h2 class="text-2xl mb-2">"Your Stats"</h2>
                <Suspense fallback=move || view!{cx, <></>}>
                    {move || {
                        account_stats.read(cx).map(|v| {
                            let stats = v.map_or_else(|| AccountStats{ user_count: 0, recipe_count: 0 }, |v| v);
                            let stats = vec![
                                Stat::new("Number Of Users", &stats.user_count.to_string(), None),
                                Stat::new("Number Of Recipes", &stats.recipe_count.to_string(), None),
                                Stat::new("Number Of Books", "0", None),
                            ];
                            view!{cx, <Stats stats={stats}/>
                        }})
                    }}
                </Suspense>
            </div>
        </Drawer>
    }
}

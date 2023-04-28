use crate::{
    components::{drawer::*, stats::*},
    contexts::{
        login::{use_login, CurrentLogin},
        prelude::{use_api, use_toasts, CurrentApi},
    },
    helpers::{api_error_to_toast, login_redirect_effect, logout_on_401, LoginState},
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
    let CurrentLogin { set_login, .. } = use_login(cx);
    let toasts = use_toasts(cx);

    let account_stats = create_resource(
        cx,
        || {},
        move |_| async move {
            if let Some(api) = api.get() {
                match api.get_stats().await {
                    Ok(stats) => Some(stats),
                    Err(err) => {
                        toasts.push(api_error_to_toast(&err, "loading stats"));
                        logout_on_401(&set_login, &err);
                        None
                    }
                }
            } else {
                None
            }
        },
    );

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    view! {cx,
        <Drawer links={drawer_links}>
            <div class="p-4 rounded bg-base-200">
                <h1 class="text-3xl font-bold mb-2">"Home"</h1>
                <h2 class="text-2xl mb-2">"Your Stats"</h2>
                {move || {
                    account_stats.read(cx).map(|v| {
                        let stats = v.map_or_else(|| AccountStats{ ..Default::default() }, |v| v);
                        let stats = vec![
                            Stat::new("Number Of Users", &stats.user_count.to_string(), None),
                            Stat::new("Number Of Recipes", &stats.recipe_count.to_string(), None),
                            Stat::new("Number Of Labels", &stats.label_count.to_string(), None),
                        ];
                        view!{cx, <Stats stats={stats}/>
                    }})
                }}
            </div>
        </Drawer>
    }
}

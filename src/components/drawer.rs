use leptos::*;
use leptos_router::{AProps, A};
use mcc_frontend_core::APP_TITLE;

use crate::contexts::prelude::{use_login, CurrentLogin};

#[component]
fn DrawerHeader(cx: Scope) -> impl IntoView {
    let CurrentLogin { login, .. } = use_login(cx);

    view! {cx,
        <div class="navbar bg-neutral text-neutral-content sticky top-0 z-50">
            <div class="flex-none">
                <label class="btn btn-square btn-ghost drawer-button lg:hidden" for="main-drawer">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </label>
            </div>
            <div class="flex-1">
                <span class="text-xl p-2">{APP_TITLE}</span>
            </div>
            <div class="flex-none">
                {move || {
                    if login.get().is_none() {
                        view!(cx, <A href="/login" class="btn btn-ghost">"Login"</A>)
                    } else {
                        view!(cx, <A href="/logout" class="btn btn-ghost">"Logout"</A>)
                    }
                }}
            </div>
      </div>
    }
}

pub struct DrawerLink {
    pub href: String,
    pub text: String,
    pub active: bool,
}

impl DrawerLink {
    pub fn new(href: &str, text: &str, active: bool) -> Self {
        Self {
            href: href.to_owned(),
            text: text.to_owned(),
            active,
        }
    }
}

#[component]
pub fn Drawer(
    cx: Scope,
    #[prop(into)] links: Vec<DrawerLink>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <div class="drawer drawer-mobile">
            <input id="main-drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content pb-8">
                <DrawerHeader/>
                <div class="pt-3 px-3">
                    {children(cx)}
                </div>
            </div>
            <div class="drawer-side">
                <label for="main-drawer" class="drawer-overlay"></label>
                <ul class="menu gap-2 p-4 w-80 bg-base-200">
                    {links.into_iter().map(|link|{
                        if link.active {
                            view!{cx, <A href={link.href} class="btn btn-primary">{link.text}</A>}
                        } else {
                            view!{cx, <A href={link.href} class="btn dark:bg-base-100">{link.text}</A>}
                        }
                    }).collect::<Vec<_>>()}
                    <li class="mt-auto">
                        <a href="https://github.com/my-cooking-codex" target="_blank" rel="noopener noreferrer" class="text-sm block leading-relaxed">
                            "Powered By "
                            <span class="font-bold">"My Cooking Codex"</span>
                            <span class="text-error">" (BETA)"</span>
                            <br/>
                            "Licenced Under AGPL-3.0"
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}

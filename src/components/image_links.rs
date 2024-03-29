use leptos::*;
use leptos_router::A;

#[derive(Clone)]
pub struct ImageLinkItem {
    pub key: String,
    pub href: String,
    pub title: String,
    pub image_src: Option<String>,
}

#[component]
fn ImageLinkGridItem(item: ImageLinkItem) -> impl IntoView {
    view! {
        <A
            href={item.href}
            class="bg-base-100 shadow-xl aspect-square w-full relative rounded-md">
            {move || {
                if let Some(image_src) = &item.image_src {
                    view! {
                        <img class="object-cover w-full h-full rounded-lg hover:brightness-50 hover:duration-200"
                            src={image_src}
                        />
                    }.into_any()
                } else {
                    view! { <div
                        class="w-full h-full rounded-md hover:bg-neutral-focus hover:duration-200">
                        </div>
                    }.into_any()
                }
            }}
            <span
                class="absolute bottom-0 left-0 p-1 w-full bg-[#000000cc] rounded-b-md whitespace-nowrap \
                    overflow-hidden text-ellipsis text-lg font-bold text-slate-300">
                {item.title}
            </span>
        </A>
    }
}

#[component]
pub fn ImageLinksBox(items: ReadSignal<Vec<ImageLinkItem>>) -> impl IntoView {
    view! {
        <div class="grid gap-4 grid-cols-1 md:grid-cols-3 lg:grid-cols-5 rounded p-3 bg-base-200">
            <For
                each=move || items.get()
                key=move |item| item.key.to_owned()
                children=move |item: ImageLinkItem| view!{
                    <ImageLinkGridItem item={item}/>
                }
            />
        </div>
    }
}

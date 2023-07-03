use leptos::*;

mod api;
mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let needs_refetch = create_rw_signal(cx, false);
    let fetch_posts = create_resource(cx, move || needs_refetch.get(), |_| async move {
        api::get_posts().await.unwrap()
    });

    let posts_view = move || {
        fetch_posts.read(cx).map(|posts_body| {
            posts_body.posts.iter()
                .map(|post| view! { cx, <p>{ post.message.clone() }</p> })
                .collect_view(cx)
        })
    };

    set_interval(move || {
        needs_refetch.update(|v| { *v = !*v; } );
    }, std::time::Duration::from_secs(1));

    view! { cx,
        <p>
            <Transition fallback = move || {
                view! { cx, "Loading..." }
            }>
                { posts_view }
            </Transition>
        </p>
    }
}

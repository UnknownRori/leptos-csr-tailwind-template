use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="min-h-screen min-w-full flex flex-col justify-center items-center gap-8">
            <h1 class="text-6xl font-bold font-mono tracking-widest text-red-600">404</h1>
            <h2 class="text-4xl font-mono tracking-wide">"Not Found"</h2>
        </main>
    }
}

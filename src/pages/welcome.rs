use leptos::*;

use crate::built_info::{get_leptos_version, get_rustc_version};

#[component]
pub fn Welcome() -> impl IntoView {
    let version = get_leptos_version();
    let rustc = get_rustc_version();

    view! {
        <main class="min-h-screen min-w-full bg-gray-50 dark:bg-black flex flex-col justify-center items-center gap-8">
            <div class="relative px-6 pt-10 pb-8 bg-white shadow-xl ring-1 ring-gray-900/5 mx-auto md:w-5/6 sm:w-4/6
                dark:text-white dark:bg-black dark:hover:bg-[#0a0f17] dark:shadow-gray-700 dark:shadow-[0_0_2px_#fff,inset_0_0_2px_#fff,0_0_5px_#08f,0_0_15px_#08f,0_0_30px_#08f] dark:ring-gray-700 dark:hover:ring-gray-600 transition-all duration-500
                rounded
            ">
                <div class="flex justify-center">
                    <img src="/assets/leptos.jpg" class="h-24 rounded-full" alt="Leptos" />
                </div>
                <div class="divide-y divide-gray-300/50">
                    <div class="py-8 text-base leading-7 space-y-6 text-gray-600 dark:text-slate-300">
                        <h3 align="center" class="text-xl">"Welcome to Leptos CSR + Tailwind Template"</h3>
                        <p>"Leptos is full-stack, fully typed cutting edge Rust framework for modern web.
                            Leptos makes it easy to build applications in the most-loved programming language, 
                            combining the best paradigms of modern web development with the power of Rust."</p>
                    </div>
                </div>

                <div class="pt-8 text-base leading-7 font-semibold">
                    <p class="text-gray-900 dark:text-slate-100">"Want to dig deeper into Leptos?"</p>
                    <p>
                        <a href="https://book.leptos.dev/getting_started/index.html" class="text-sky-600 hover:text-sky-500 duration-200" inner_html="Read the docs &rarr;"></a>
                    </p>
                </div>
            </div>
            <div>
                <span class="dark:text-slate-400">
                    "Leptos : v" {version} " - " {rustc}
                </span>
            </div>
        </main>
    }
}

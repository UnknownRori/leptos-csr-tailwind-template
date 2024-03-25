# Bare Bone Leptos CSR + Tailwind

This template created from scratch using stable [Leptos](https://leptos.dev/). 
It's also include [Leptos Router](https://crates.io/crates/leptos_router), no nightly version is used.

## Overview

This template demonstrate how to integrate [TailwindCSS](https://https://tailwindcss.com/) with [Leptos](https://leptos.dev/) 
and use basic tools like `trunk`, `npx`, `npm`.

```sh
# Install trunk if you haven't
cargo install trunk

# Install toolchain if you haven't
rustup target add wasm32-unknown-unknown
```

## Installation

### Git Clone

```sh
# Don't forget to rename it :)
git clone https://github.com/bare-bone-leptos-tailwind
cd bare-bone-leptos-tailwind

# Optional but if not installed there will be installed automatically
npm i -D tailwindcss
```

### Cargo Generate

> [!WARNING]
> I don't know if it work or not.

## Running

```sh
# To start development server
trunk serve

# To start development server and open it with favorite browser
trunk serve --open
```

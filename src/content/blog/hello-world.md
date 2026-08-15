[Home](./index.md) | [Blog](./blog/index.md)

---

# Hello World

◦ Hello 👋🏿 Bonjour 👋🏾 Привет 👋🏽 你好 👋🏼 안녕하세요 👋🏻 こんにちは 👋 नमस्ते ◦

> **Incredible is a Rust Text User Interface (TUI) Framework for the 2nd Quarter of the 21st Century.**

In our world of computer programming, the term *Hello World* refers to a small and simple program that prints the words "Hello, World" to a screen. This small and simple program is an introduction consisting of two parts: an illustration and a demonstration.

## Illustration

For a programming language, a small *Hello World* illustrates the syntax. For libraries and frameworks, it is usually used to illustrate the shape of the API. As such, it doesn't have to be limited to printing "Hello, World". As long as it succinctly illustrates some core ideas, that is OK.

Here is a *Hello World* for Incredible.

It takes the form of a *Moose Clicker*, a small Rust program  that prints a moose emoji to the screen. When the moose emoji is clicked with the mouse, a counter is incremented.

```rust
use incredible::*;
use incredible_elements::{App, Text};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {
    clicks: usize,
}

let app = App::default();

let moose_clicker: Text<State> = Text::default();
moose_clicker
    .on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.clicks += 1;
        }
    })
    .on_state(|el, state| {
        el.text(&format!("🫎: {}", state.clicks));
        el.draw();
    });

app.add(moose_clicker);

app.run(State::default());
```

## Demonstration

When a *Hello World* program runs, it demonstrates that the language, the library, or the framework works in the greater context of the system for which it is designed. It is as if the program has come to life and is greeting the world around it.

Here is the moose clicker running. Try it out. It works. It counts.

<embed type="application/incredible" src="moose_clicker">


## More

While something resembling the *Moose Clicker* is the common type of introduction, it feels lacking. We can do better. Actually, much better. This website as a whole is a much improved *Hello World* for the framework. This is because this is not a typical website. It is not made out of HTML, CSS, and JavaScript. It does not rely on the browser to provide layout, styling, and interaction.

This website is actually an Incredible app compiled to WASM.

Creating the screen, reading a markdown file, laying out the text, coloring it, enabling the scroll, managing it, changing the mouse pointer over the scroll bar, responding to the drag on said scroll bar, underlining the links, coloring them when they are hovered, removing color when they are not, inverting them when they are clicked, loading the next markdown page, changing the browser URL hash, the keyboard shortcuts, Ctrl+e to select all, Ctrl+c to copy, this, that and all the rest - it's all coming from Incredible, not the browser.

You are interacting with precompiled Rust, not JIT-compiled JavaScript. You can download the app as [pre-built binaries for other platforms](https://github.com/ronilan/incredible/releases) too, if you wish.

It works. It demonstrates. It illustrates (see the [code](https://github.com/ronilan/incredible/tree/main/src)). As such, I think this is a proper introduction.

Will iter back,
Ron

*August 15, 2026*

---

[Home](./index.md) | [Blog](./blog/index.md)

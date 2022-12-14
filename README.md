# Rusty Snake

This is me trying to learn Rust by building a snake game.

I am loosely following the great tutorial [Let's Build Snake with Rust](https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html) by Jonathan Henderson.

<p align="center">
<img src="screenshot.png" />
</p>

## Thoughts & Impressions
I've listed every passing thought about Rust during the time I worked on this project. Some of the things I wrote down are probably elementary and can be easily answered by diving into the docs or asking around the community, but I believe that at the very least it can help portray the learning process I've had with the language.

### Developer Experience
* Love how easy it is to get started! (with both cargo and the rust-analyzer vscode extension)
* Out of the box auto formatting? Sign me up!!
* But no auto semicolon addition? I understand that this is a syntax error (and not styling), but at least let us [prettier](https://prettier.io/) enthusiast to opt in :)
* I **LOVE** how every tiny thing (unused imports, for instance) shows up.
* In general, dead code elimination here is impeccable. It tells you exactly what is not being used, no ifs, no buts, no maybes. This is where the language's super strict nature really shines, and I think that despite the verboseness it imposes, I am definitely willing to pay the price.

### Modules
* Declaring modules upfront? Interesting concept. There **is** some redundancy, though, in my opinion. Easily solvable with an "add module" vscode task.

### Ownership & Borrowing
* Rust has a unique memory model. Not sure what's my take on it, but I definitely like the challenge!
* Borrowing would've been a complicated fit if not for the compiler, so thanks, Rust!
* That said, a hint that says `you might want to borrow here` for functions that simply observe a given value would be nice, since in most cases where we have non-primitives, it's probably more efficient to borrow if possible.
* Thinking about it, shouldn't have **borrowing** be the default behavior? My code is littered with `&` signs

### Conventions
* `impl` is an interesting concept, similar to Go's methods. It's a way to separate data from behavior. The thing is, I see mutators \ selectors as composites over the data type: while most of them work over a single struct, the fact that they're separated makes them eligible for handling higher order mutation \ selection over multiple structs. Having `impl` as a construct of the language makes a clear cut and provides something more akin to an OOP approach, rather than functional one
* For a language that uses immutability by default, I was surprised with how many mutators there are as opposed to maps. It is way easier to `pop` something out of a vector than to create a trimmed version of it.
* Immutability costs more, both in run time and memory space. Perhaps the language could use some sort of a branching memory model for immutability? (similar to event sourcing)
* casting is so so SO VERBOSE. From u16 to u32 to i64 to usize, each library and struct and construct decides on its own convention, which makes it really hard to sanely run arithmetic operations. That said, this is probably unavoidable for performance-oriented languages, and I'm **surely** yet to find the conventional way to do cross-type arithmetics in rust (no way that this problem was left unattended by the community). Gotta dive deeper.
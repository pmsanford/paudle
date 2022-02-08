Paudle
======

A reimplementation of the excellent word game [Wordle](https://www.powerlanguage.co.uk/wordle/) by [Josh Wardle](https://twitter.com/powerlanguish). This version was created using [Yew](https://yew.rs/) and [Rust](https://www.rust-lang.org/). I cribbed the colors and layout (and of course the game logic) from Wordle, but the implementation is all original.

Running
-------
First, [install the webassembly target and trunk](https://yew.rs/docs/getting-started/introduction). Clone the repo, run `yarn install`, run `trunk serve`, and navigate to http://localhost:8080 using your browser.

Todo
----

Still missing:
- Hard mode
- Animations

Needs cleanup:
- Clones everywhere on prop passing
- Lots of prop drilling
- Scoreboard spaghetti

[The word corpus](src/awords.txt) is taken from [this repo](https://github.com/oldfartdeveloper/wordle-generate-word-list) under the MIT license.

Special Thanks
--------------
To @Cadiac, author of [Sanuli](https://github.com/Cadiac/sanuli) (a Finnish Wordle), for showing me how to properly attach a keyboard listener.

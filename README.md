Paudle
======

A reimplementation of the excellent word game [Wordle](https://www.powerlanguage.co.uk/wordle/) by [Josh Wardle](https://twitter.com/powerlanguish). This version was created using [Yew](https://yew.rs/) and [Rust](https://www.rust-lang.org/). I cribbed the colors and layout (and of course the game logic) from Wordle, but the implementation is all original.

Unlike the original, this one is entirely client side, so there's nothing to stop you from cheating - if you can figure out how to extract the current word from the running WASM (I can't). 

Still missing:
- Copyable scoreboard with unicode blocks
- Allowing you to pick up where you left off (if you navigate away or refresh)
- Tracking how well you did across game sessions
- Hard mode

[The word corpus](src/awords.txt) is the 2250 most common 5 letter words in the English wikipedia, corpus, retrieved [here](https://github.com/IlyaSemenov/wikipedia-word-frequency/tree/master/results) under the MIT license. This is actually a pretty bad word list because it's missing things like plural forms and other common words unlikely to be in Wikipedia articles like "slugs" and "creep" but includes more common proper nouns like "klaus."

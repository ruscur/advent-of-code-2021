# Day 18

I don't think I've ever fought the Rust compiler as much as today.

Self-referential types are hard.

I spent at least half an hour banging my head against a wall until
I figured out `.into_iter()` solved all my problems when implementing
`.add()`.  Pretty much everything that was a pain today would've been
trivial in Python.

Disappointed with my code today but not willing to clean it up.  I
would like to learn how to correctly implement self-referential types
rather than what I actually did which was ping-pong between `Element`
and `Pair`.

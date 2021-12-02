# Day 2 stuff

Decided to learn stuff about error types with this one.  This solution is
completely over-engineered, but it's probably good to get that out of the
way before the harder problems start coming in and the "maintainability"
factor goes way down in favour of just solving the problem.

I'd never actually defined my own error type before.  Errors can be any
type, kind of.  When implementing the `FromStr` trait, it's literally
just any type.  When returning a `Result<T, E>`, nothing actually
cares if `E` implements the `std::error::Error` trait or not, it was
only when it was getting bubbled up to the `Box` that it mattered.

So it's made writing error types less scary for me, though it's still 
an annoying amount of boilerplate (especially when it's a pointless
project).  If I wasn't trying to avoid Cargo at all costs I would
just use `anyhow` and never care, avoid defining error types and
use `bail!()` when something was wrong.

The benefit of over-engineering is that changing stuff for part 2s
is really easy.

I'm not super happy with the `fold()` either.  My `Submarine` struct
is just 3 values so repeatedly yielding it isn't a big deal, but it
still seems gross especially since the part 2 change makes it so that
you couldn't get the same results from parallelising the `fold()`.

So I'd probably prefer to have it in a simple for loop to better
communicate that it needs to be handled sequentially.  I do love
avoiding a mutable result though.

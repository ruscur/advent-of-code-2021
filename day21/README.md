# Day 21

Another terrific puzzle that I thoroughly enjoyed.

I love these ones that are conceptually easy to understand, but the devil
is in the details.  I also love ones where you write a solution and it's
too slow and you have to figure out how to make it faster.

This was the first time I wrote an iterator method (`.sum()`).

One thing about writing Rust is that mutable state feels a bit wrong.  My
first solution didn't have the database and it was way too slow since it
was doing a bunch of redundant work.  I wonder if there was a way to go
about this without mutable state.

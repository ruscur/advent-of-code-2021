# Day 5

Another one that was made for NumPy or similar.  Same gripes as yesterday.

I can definitely be so much faster in Rust than I am, but I don't really
see how tasks like this can ever feel good in Rust (until the point you're
optimising performance, maybe).

There was no need to do stuff like checking direction at parse time, I think
my natural instinct is to parse things in a way that any subsequent users
have the least amount of work to do in order to do useful things with it.  In
cases like this it's just slow, but in the "real world" maybe it saves lots of
work.

That said, the Rust way would probably be to have calculating direction be a
method of the struct, so it's a zero cost abstraction until it's needed for
something.

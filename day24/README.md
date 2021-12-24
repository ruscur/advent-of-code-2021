# Day 24

I just brute forced it.

The way I should have done it is actually to reverse engineer the code.
I wanted to try a method where I *didn't* do that, to see if there was
a sensible programmatic way to go about it.

I had this master plan that I would try and invert the bytecode, and
whenever doing that was destructive (i.e. rounding down on division,
many different options for modulo) I would split off a new universe
and simulate further.  That way, there's still a lot of calculation,
but I can start from the point of `z = 0` and work my way backwards.

That was pretty insane and I'm not sure if it can even work, but I
spent a few hours trying anyway, before just deciding that it's not
*that* slow and I can just run for a few minutes and brute force
the answer.

I should have just reverse engineered it and not judged that as
"cheating".  I haven't looked at other solutions yet, so I might
have a go at reverse engineering it sometime later.  I am definitely
not satisfied with having something that takes minutes to run when
the rest of my solutions have been under a second!

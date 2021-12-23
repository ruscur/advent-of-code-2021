# Day 23

Another fun one!  Again, love these with simple concepts but difficult execution.

Got my highest rank ever on this, despite starting over an hour late, and taking
a very comfortable amount of time writing out all my hardcoded boilerplate.

There's still performance improvements to be made - we remove a lot of duplication
by checking if the position was accessed with less energy previously, however we
should also save the rest of the energy used to complete the burrow optimally
from that position.  This saves work if we reach the position faster in future,
no position should have to be calculated twice.

Happy to have this one down in the 3-4 hours it took me, anyway!  Two to go!

Also `Vec`s being unable to `Clone` or `Copy` is the bane of my existence and
I really need a better solution than hiding slices inside enums.

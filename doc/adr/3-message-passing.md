# Using message passing instead of memory sharing
**Date**: 2022-11-01

# Context
When I start this project i was using memory sharing across thread, using `mutex`, `rwLock` and `arc`.

It quickly became difficult to track which part of the code was "responsible" to maintain the state of the server.

In addition `mutex` or `lock` does not come without issue, although I didn't had the issue often, `deadlocks` can quickly become a nightmare.

# Option evaluation
Another approach could be to use `message passing` instead of `memory sharing`, using `mpsc` in an event loop would allow a single consumer to mutate the state, from within a dedicated thread.

It would allow to mutate state from a single place, making it easier to track which part mutate state. In addition, this approach does not need any lock.

# Decision
Rewrite state mutation to leverage a single game loop responsible to mutate the state of the server.
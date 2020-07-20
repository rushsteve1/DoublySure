# DoublySure

> Using types to make sure that you're sure, sure, and doubly sure

Development Status: **Alpha**

## Description

Users get prompted to make sure they want to perform a destructive action, why
shouldn't developers?
Rust is full of considerations of safety and security, protecting the code from
itself. But what about protecting the code from us, the developers who wrote it?
Destructive and costly actions are often just a `.delete()` away.

DoublySure wants to help prevent at least some of that by providing a single
type `AreYouSure`, and a macro `make_sure` to convert data and functions to it.
`make_sure` will also defer function calls, so that `.delete()` won't get called
immediately.

When you encounter an you can either call `.yes_i_am_sure()` which will return
the inner value or run the deferred function call, or you can say
`.no_i_am_not_sure()` which will discard the data and not call deferred
functions.

## Use Case

Any instance in which a dangerous operation could be performed, and there is
little resistance to performing it.
DoublySure exists to make programmers stop and think "Is this what I mean to
do?", as well as provide programmatic second chances to say no.

Also please do not abuse the function call deferment mechanism this crate
provides. That will only end in sadness for you and me both.

## Alternatives

- Rust's built in `Result` and `Option` types.
- Futures for deferment

## ToDo

- Look into using Futures as a way of deferring execution.

# advent_of_code_2022
Advent of Code 2022 solutions in Rust.

Decided to pair this challenge with learning a new language this year. After
going through the [Rust Book](https://doc.rust-lang.org/stable/book/) this
seemed like a great way to practice the language and get used to writing it.
I'm sure I'll learn a lot along the way.

## Project Structure
I did the first two days as just functions in the `lib.rs` file to see what
kind of pieces I could factor out.

Created the `Solver` trait to act as an interface for each day's methods.
It was strange to work with wrapping the trait in a `Box<dyn Solver>` to allow
for returning any instance of the trait from a function as opposed to C# where
I am used to just returning the interface type itself. Once I wrapped my head
around this and the `Option` type around it, it was actually pretty intuitive
to write.

## Day 1
Not so bad. A nice little intro to trying to do some functional style
programming in the language. A bit trickier to wrap my head around some of the
methods coming from LINQ, especially having so specify types for things like
`sum()::<u32>`.

Part 2 was simple enough, but again I found I was missing things from LINQ,
like being able to chain all of the calls instead of separating the `sort()`
into it's own call and manipulating the data in place. I'd rather have some
type of `sorted()` iterator that allows this chaining and doesn't require a
mutable copy of the data.

## Day 2
Pretty simple today. I took the easy way out by hardcoding all 9 possible
outcomes in both scenarios. There is probably a more programatic way of doing
the calculation, but oh well. This way works and gets me 2 stars closer to
completion. I may revisit this one at a later date and find a more elegant
approach.

## Day 3
Wow. What feels like a pretty simple problem has turned into a convoluted
mess of code. I originally tried Part 1 by splitting the string in half and
converting each half into a `HashSet` so I could use the `intersection()`
method to find the shared character. This ended up turning into a mess of
dereferencing and odd code that was not pleasant to look at. I might just be
too lacking in my understanding of Rust at this point to pull that solution off
cleanly. Maybe I'll come back to it later and try again.

Using a simple for loop to check the characters turned out much nicer to look
at. Maybe my first solution was just too overcomplicated when the basics work.

Finding the `chunk()` method on the `Vec<>` type was also quite nice here.
Really saved some work in having to have another loop going through three
items at a time and keeping track of indexes.
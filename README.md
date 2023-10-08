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

## Day 4
This puzzle was pretty easy to come up with a solution for, but oh boy do I
hate how I wrote the code. The `parse_sections_to_u32s()` function is rather
verbose and returning a tuple of tuples seems a little gross, but it works.
This may be another one I come back to at the end and try to rework to cleaner
code with a better understanding of Rust.

The expressions I use in the two `filter()` calls are quite obnoxious as well.
Again, it works which is good, but I'd like to clean it up. There may be a way
to just turn them into simple ranges and use or write a function that checks if
a range contains or overlaps another. That sounds nice - I'll definitely be
coming back to this one. I think ideally I will just come back to the beginning
and go through each day again once I've completed all 25 and work on cleaning
one up with new knowledge or by researching cleaner solutions.

## Day 5
Holy cow. My initial run at this is just hideous code to me. It works, but oh
boy do I hate looking at it. I think a lot of this is probably just my
inexperience in the language showing. There are probably much easier and more
idiomatic ways of writing this one. This definitely solidifies my plan of
looping back around at the end and taking the time to clean each of these up.

## Day 6
I enjoyed this one. It was a nice simple break after the mess I wrote for Day
5. The only setback was that I was trying to create a `HashSet` from the
windows, but the type checker wouldn't allow it. After some research about
different types I found that `char` doesn't implement the right trait to be
hashable, but it does work in a `BTreeSet`. Once I had the right type to use
for my set this was a piece of cake. The `windows()` function that is akin to
the `chunk()` function I used in day 3 made this a quite simple and nice piece
of code to write.
# advent_of_code_2022
Advent of Code 2022 solutions in Rust.

## Day 1
Not so bad. A nice little intro to trying to do some functional style programming in
the language. A bit trickier to wrap my head around some of the methods coming from
LINQ, especially having so specify types for things like `sum()::<i32>`.

Part 2 was simple enough, but again I found I was missing things from LINQ, like
being able to chain all of the calls instead of separating the `sort()` and
`reverse()` into their own calls to modify the data in place.

## Day 2
Pretty simple today. I took the easy way out by hardcoding all 9 possible outcomes
in both scenarios. There is probably a more programatic way of doing the calculation,
but oh well. This way works and gets me 2 stars closer to completion. I may revisit
this one at a later date and find a more elegant approach.
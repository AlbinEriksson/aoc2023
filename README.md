# Advent of Code 2023

These are my solutions for Advent of Code 2023. Like last year, they're all in Rust since I don't really get to use Rust at
work or in my projects.

I even copied my `util` module from last year. The idea is that it would "speed things up", but really I'm just going to keep
adding to it, wasting even more time. Can't change old habits. 😋

# AoC '23 thoughts

Here are my thoughts on the puzzles I consider as challenges.

## Day 5 (If You Give A Seed A Fertilizer)

AoC yet again proving that naive solutions don't scale, unless you want to store 2.4 billion integers at once. I did the math
on that one. But actually, 2.4 billion is still doable, so someone must've done it.

Instead, I, and likely many other people, stored the seed numbers as ranges and split them when necessary. This way, I ended up
with 119 ranges consisting of two integers each. That makes 238 integers, which is ten million times less than 2.4 billion.

It's fun to compare a solution to the naive one, and the bigger the difference, the better it feels.

## Day 8 (Haunted Wasteland)

Ghosts. In a desert. During a sandstorm. And the puzzle is called Haunted Wasteland? This reminds me of [a certain video game](https://zelda.fandom.com/wiki/Haunted_Wasteland).

This puzzle was hard, but even harder if you try to solve it for all edge cases. That's because this is one of those puzzles
where you can make assumptions about the input.

Part 1 was trivial, so here are my assumptions for part 2.

1. All A-nodes move on distinct cycles and never overlap.
2. All A-nodes will reach exactly one distinct Z-node in its cycle.
3. Each Z-node takes the same amount of steps to reach as the number of steps in the cycle.
4. The L/R instruction sequence does not repeat, besides starting over from the first instruction.
5. The L/R sequence length is prime.
6. Each cycle length is a distinct prime number multiplied by the L/R sequence length.
7. The cycle length's prime number is not the L/R sequence length.

I found these assumptions to be true by measuring the cycle length, number of steps before the cycle starts and the number of
steps until the Z-node is reached.

Because of assumptions 2, 3 and 4, you can get the answer by taking the LCM (lowest common multiple) of all cycle lengths.
Otherwise, you'd have to check multiple Z-nodes per cycle and take their odd positions in the cycle into account. And without
assumption 4, we'd have to subdivide the cycle.

Because of assumptions 5, 6 and 7, the LCM is easily calculated by multiplying the distinct primes from each cycle, as well as
the L/R sequence length (which is a factor that all cycle lengths have in common). The final product is the answer to part 2.

Seriously, if those assumptions were not true, we'd all be here for much longer. I really hope those assumptions hold for
everyone else's inputs, otherwise I'd be extremely lucky.

# Usage

You're not going to cheat, right? Good.

Because if you were, I would tell you not to create a directory called `input`. Then, I would suggest not naming the input
files `day1`, `day2` and so on. And most importantly, I would never advise running this command to get your answers:

```sh
cargo run --release <all|1-25> [repeat_count]
```

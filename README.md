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

# Usage

You're not going to cheat, right? Good.

Because if you were, I would tell you not to create a directory called `input`. Then, I would suggest not naming the input
files `day1`, `day2` and so on. And most importantly, I would never advise running this command to get your answers:

```sh
cargo run --release <all|1-25> [repeat_count]
```

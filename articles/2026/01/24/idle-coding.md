# Idle coding
2026-01-24T11:52:04-08:00
tags: Codex, LLMs, AI, Programming

I've discovered a new hobby.
It involves creating non-trivial libraries from scratch.
(Not UI slop!)
Except I don't write any code in editors.
An agent does all of it.

It goes like this:

Set up an infinite loop so the coding agent:

1. has a clear guard for correctness (unit tests)
2. has an objectively measurable, pass/fail outcome (performance benchmarks)
3. has tools for styling (dead-code tool, formatter)
4. leaves artifacts for future instances (commit changes on a branch, writes failure_analysis_SHA.md)

... and watch number go up.

You'll need a robust prompt, then just feed it to a single agent, over and over. No need to convince the agent instance it needs to repeat (doesn't hurt to try, but they usually give up after a while).

It's more fun if you ask for a success artifact (benchmark_success_SHA.tsv) and have a script that monitors for new ones. I run this script with @steipete's VibeTunnel so I can watch the number go up on my phone.

It may take some setup to get going.
For example,
at the beginning you want to import a large number of unit tests.
You'll want to source some input for benchmarking.
You'll want some infrastructure (SwiftPM project, Scripts directory, git).

But quickly,
it becomes an [idle game](https://en.wikipedia.org/wiki/Incremental_game).
At first,
you'll watch the number of unit tests go from 100% fail to 100% pass.
Then,
the performance benchmarks.
Bonus points if you benchmark against existing libraries,
and beat them in performance.
This has happened on every project I've done.

Codex btw.

I should clarify:
this is not serious software engineering.
The goal is having fun.
Although the end result usually is really useful,
and because of robust unit testing + benchmarking,
it might be good enough to use in production.

We'll find out.

My favorite idle game in 2026 so far.

# An Invisible Chasm
2022-02-17T13:40:27-08:00
tag: Software Scaling, Programming Language, Tooling

In the journey of growing software systems, there comes a point where certain engineering decision will create
a chasm so large that it's practically impossible to go back. The initial crossing of this chasm is so easy
that it often happens without anyone noticing. Nonetheless, it is a costly mistake.

On one side of this chasm, any member of the engineering team can answer the question "how, exactly, is my
code being consumed in our engineering org?". Here lies clarity. Your team fearlessly delete dead code. Legacy
logic has their days marked. Lacking documentation? Not ideal, but you can always find how an API gets used
in all scenarios easily.

On the other side, no one has the answer. The corpses pile up. Deprecating things comes with uncertainty. You
warn your team "please carefully consider adding things here, because we'll keep it around forever". Need to
respond to an incident? Better hope your monitoring system is good. Trying to trace the code path won't get
you answers in time. We'll leave that for the post-mortem.

Tooling, yes, exactly what decision leads to the chasm depends on the state of tooling for your technology
stack. The programming languages, build system, repository setup, coding style guides, IDEs, linters, search
engine ... all factor into the final answer. A statically typed language compiler won't help you look up
users in a different micro-service. State-of-art symbol lookup may exist for a dynamically typed system
because some companies have thrown enormous amount of money to solve it. Maybe your kick-ass style guides
enforced by linters across multiple languages makes it possible to locate anything by text. You see, it is
a quantitative matter: it is always *possible* to revert to the good old days. But it will be *expensive* in
terms of dollars, and/or will power.

When did you cross the chasm?

Beware of the chasm.

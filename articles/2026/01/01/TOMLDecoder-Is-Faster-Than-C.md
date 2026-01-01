# TOMLDecoder Is Now Faster Than C (Thanks to AI)
2026-01-01T11:07:05-08:00
tags: AI, TOML, Swift, Performance, C

Recently,
I gave my TOML library written in Swift [an 800% speed boost](/2025/12/10/TOMLDecoder-0.4.1/).
The natural question after that is:
how much faster can I push it?

<details>
<summary>
I'm happy to report that TOMLDecoder now parses the <a href="https://github.com/dduan/TOMLDecoder/blob/cea8f0bee33f37e0fcc33b566a742485c71196e7/Sources/Resources/fixtures/twitter.toml">Twitter payload example</a> 1.8x faster than the C library <a href="https://github.com/cktan/tomlc99">tomlc99</a>, and 5x faster than <a href="https://github.com/marzer/tomlplusplus">TOML++</a>.
</summary>

I tried to be as charitable as possible for the non-Swift libraries while keeping the call sites in Swift.
For example,
it takes time to create or copy the UTF-8 bytes of a `Swift.String` into a contiguous region.
And that's not counted towards the other libraries' parsing time.
TOML++ runs faster with exceptions enabled.
So that's the path I chose to benchmark.
When bridging the C++ code,
I made sure there's no allocation,
no checking for input/output, etc,
so that the bridging overhead is trivial.

Here's the benchmark code run repeatedly to collect an average,
with warmups ahead of time:

```swift
func benchmarkTOMLDecoder(source: String) throws -> Double {
    let start = CFAbsoluteTimeGetCurrent()
    let table = try TOMLTable(source: source)
    let end = CFAbsoluteTimeGetCurrent()
    blackhole(table)
    return end - start
}

func benchmarkCTOML99(source: String) -> Double {
    var source = source
    var duration: Double = 0
    source.withUTF8 {
        $0.withMemoryRebound(to: CChar.self) { buffer in
            let baseAddress = UnsafeMutableRawPointer(mutating: buffer.baseAddress!)
            let start = CFAbsoluteTimeGetCurrent()
            let table = toml_parse(baseAddress, nil, 0)
            duration = CFAbsoluteTimeGetCurrent() - start
            blackhole(table)
        }
    }
    return duration
}

func benchmarkCTOMLPlusPlus(source: String) -> Double {
    var source = source
    var duration: Double = 0
    source.withUTF8 {
        $0.withMemoryRebound(to: CChar.self) { buffer in
            let start = CFAbsoluteTimeGetCurrent()
            let table = tomlpp_parse(buffer.baseAddress, buffer.count)
            duration = CFAbsoluteTimeGetCurrent() - start
            blackhole(table)
        }
    }
    return duration
}
```

where `tomlpp_parse` is a minimal wrapper for the TOML++ library:

```cpp
void *tomlpp_parse(const char *conf, size_t conf_len) {
    try {
        static toml::table table{};
        table = toml::parse(std::string_view{conf, conf_len});
        return static_cast<void *>(&table);
    } catch (...) {
        return nullptr;
    }
}
```
If any of these measures are unfair to the C/C++ libraries,
I'd love your feedback!
</details>

Here's the output of the benchmark program I wrote:

```
Benchmarking TOML parsers...
File size: 443461 bytes

Warming up...
Running 100 iterations...

Results:
═══════════════════════════════════════════════════════════
TOMLDecoder:
  Average: 1.232 ms
  Min:     1.203 ms
  Max:     1.332 ms

cTOML99:
  Average: 2.226 ms
  Min:     2.190 ms
  Max:     2.341 ms

cTOMLPlusPlus:
  Average: 6.107 ms
  Min:     6.038 ms
  Max:     6.377 ms

TOMLDecoder is 1.81x faster than cTOML99
TOMLDecoder is 4.96x faster than cTOMLPlusPlus
═══════════════════════════════════════════════════════════
```

I charted the wall clock time and instruction counts over the commit history.
You can see that the latest release is a lot faster than 0.4.1:

<iframe id="benchmark-iframe" src="/assets/2026/01/tomldecoder-0.4.3-improvements.html" width="100%" height="1200" frameborder="0" style="border: none; display: block; margin: 20px 0;"></iframe>

<script>
window.addEventListener('message', function(event) {
    if (event.data.type === 'resize') {
        const iframe = document.getElementById('benchmark-iframe');
        if (iframe) {
            iframe.style.height = event.data.height + 'px';
            iframe.style.transition = 'none';
        }
    }
});
</script>

... and, the majority of these commits are authored by AI! How did that happen?

## It's old-fashioned engineering, baby!

I ended the [last post](/2025/12/10/TOMLDecoder-0.4.1/) with the following (emphasize in __bold__):

> ...the project also gained a bunch of infra improvements.
> * It has a DocC-based documentation site.
> * __The entirety of the official test suite is now programmatically imported as unit tests.__
> * __The source code style is now enforced by swiftformat__
> * Platform checks are more comprehensive and modern on CI.
> * __Benchmarks are now modernized with ordo-one/package-benchmark.__

If you set out to optimize the runtime performance of a software project,
infra improvements like these will ensure that
your engineer can explore options for optimization with confidence that
they won't break the expected behavior,
and their efforts can be measured objectively.

Most importantly,
as detailed in the last post,
the architecture of the TOML parser has received some significant upgrades.
This type of change is rare in a small project,
and I don't expect it to happen again in the next phase of optimization.

I set up a separate project that calls into TOMLDecoder
so that I can profile it with Instruments.

It was during the holidays, and
although the idea of trying my hand at micro-optimizing the code,
and gradually squeezing out performance juice sounded really fun,
I also had a bunch of travel planned.
So what else is there to do?

I booted up codex.

## gpt-5.2-codex, my performance engineer

For the most part,
I simply fed this prompt to codex over and over again:

> Objective: Try to make the p50 of "parse twitter.toml" benchmark improve by > 1.1% on instructions or retains compared to the `main` branch. Improvements on either is acceptable as a success, but regression in either should be considered a failure. Other metrics in the benchmark does not matter.
>
> Verify iteratively:
> 1. Make code changes
> 2. Format with `make format`.
> 3. Make sure all tests passes by running `swift test`.
> 4. Create a branch prefixed with `cc/` in name
> 5. Commit all changes. Include description of the optimization as body of the commit message.
> 6. Use Scripts/benchmark.sh to run the benchmark, recording its output in a text file
> 7. If the benchmark result meets the improvement threshold, cherry-pick the change onto main. Otherwise, Commit the benchmark results file to the branch you created, switch back to main and start over.
> 
> When you run the benchmark script, NEVER use `HEAD` as its argument. Use explicit SHAs. Only use Scripts/benchmark.sh SHA_OF_BASE SHA_OF_TARGET to run the benchmarks. Do not try to run the underlying commands directly.
> 
> You must NOT look at the content of Benchmarks/, or the content of Sources/Resources.
> 
> To give you some direction, I've profiled the parsing the twitter example, and included the inverted time profile call tree in /tmp/trace-tree.txt.

The prompt changed gradually in these ways:
1. Wording became more streamlined as I figured out how gpt-5.2-codex interprets specific things.
2. The optimization threshold decreased as the lower-hanging fruits got picked
3. The benchmark to optimize changed a bunch of times because they have different data profiles

Each time the optimization threshold is met,
I collect another time profile from instruments with the latest change,
and restart the session with the same prompt.

I actually started the journey with gpt-5.1-codex-max.
It would find 5-10% improvements consecutively at the beginning.
Then it would start to struggle,
then I'd switch to gpt-5.2-codex with the default "Medium" setting,
then "High", and eventually "Extra high".
Towards the end,
the LLM could barely find any speed improvements
without regressing other benchmarks in some way.
That's when I decided it's time to cut a release.


## My observations of the model

Despite occasional struggles with conventional Swift coding style,
I find that gpt-5.2-codex is good at analyzing the flow of the parser,
and finds ways to short circuit certain logic.
These types of discoveries made the parser quite a bit faster.

It replaced key comparisons in a hot loop with hash value comparisons,
which brought in a significant speedup.
In retrospect, this idea seemed fairly obvious,
but, in my imagination,
I wouldn't have been bold enough to try it.

The LLM has a few favorite things to try at the start of each session.
* It would see a linear search and try to replace it with a dictionary lookup
* It would try unrolling loops (in a few cases, this actually helped),
* It would reserve array capacities ahead of time,
* It would eliminate copies by converting things into classes

But then the benchmark would regress,
which forces it to explore other paths.

Although my prompt tells the model to not look at the benchmark itself,
it sometimes goes and does it anyways.
I suppose benchmark-maxing is a temptation too great to resist for it?

As reported by [others](https://steipete.me/posts/2025/shipping-at-inference-speed),
I also observed that gpt-5.2-codex would spend a lot of time just analyzing,
before attempting any changes.
The code change it produces is almost always one-shot.
It rarely goes back and revises the idea it's attempting to implement.

## Conclusions

Good engineering practices continue to pay dividends with LLMs.

TOMLDecoder reached a point where its runtime performance is itself a feature worth talking about.

The setup of this project can serve as a benchmark for LLMs, I think?
Here's a prompt,
a concrete, measurable outcome represented by numbers,
a huge test suite.
How far can you push those numbers?

---
title: Hello World In WebAssembly
tags: [WebAssembly, YouTube]
date: 2017-12-30 18:43:49-0800
---

Every now and then, I check on the progress of Web Assembly. I did it again
around the time of this post and finally found enough tutorials, examples, and
working software to get myself started in this area. In doing so, I made a video
to demo some progress. (_this article includes all the same information and
more, so just read on if you don't have 15 minutes for YouTube_).

<iframe width="560" height="315" src="https://www.youtube.com/embed/yEYtwmI7bDg" frameborder="0" gesture="media" allow="encrypted-media" allowfullscreen></iframe>

<br>

## Our goal:

1. Use as much built-in tools on a Mac as possible. The web development
   toolchain scares me.
2. Target the browser. That's where the value of WebAssembly is. (Node supports
   it as well. BUT, WHY THO?)
3. Build from scratch. In the video I started from `mkdir` a folder. We should
   strive to understand details on every level whenever possible. Boilerplates
   and dependencies should come later.

## Things you'll need:

1. Safari 11+
2. Xcode. More specifically, you should be able to run `clang` in a shell.

## The Workflow

Having these things installed, get a copy of [The WebAssembly Binary
Toolkit][1] (wabt). Build it. The README has detailed instructions. I just went
into the folder and ran

```
make clang-release
```

This will generate a bunch of binary files in `out/clang/Release` and you need
to make sure you can run them from wherever you want to work on WebAssembly
project (so either copy them into a folder included in your `PATH` environment
variable or add the absolute path to `out/clang/Release` to `PATH`).

Among the binaries "wabt" builds, `wat2wasm` takes a `.wat` file and compiles it
to a WebAssembly binary. A `.wat` is a source file in the [text format][2] for
WebAssembly, which is in the form of S-expressions. So

```
wat2wasm main.wat -o main.wasm
```

…will compile your WebAssembly module in `main.wat` to generate `main.wasm`, the
binary file. For now, `main.wat` can be the simplest WebAssembly program:

```lisp
(module)
```

Running the binary in a browser demands the bulk of the work. First, we'll need
a web page. It doesn't need any content other than invoking some JavaScript
code.

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title></title>
</head>
<body>
    <!-- The only thing that matters is the following line,
    although having a valid HTML5 page is nice. -->
    <script src="play.js"></script>
</body>
</html>
```

First, the Javascript logic needs to _fetch and instantiate the compiled
WebAssembly module_. Since this is not a JS or WebAssembly tutorial, I'll point
you to the docmuntation for [Fetch API][3], [Promises][4], and [the WebAssembly
object][5] for details:

```javascript
fetch("main.wasm").then(reponse =>
    reponse.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, {})
).then(result =>
    result.instance
).then(main);

```

This snippet fetches `main.wasm` (adjust this URL according to your choosing),
instantiate it, then pass it into a function named `main`, we can put
a placeholder logic for it for now:

```javascript
function main(wasm) {
    console.log(wasm);
}
```

Before we move on, you'll find that simply opending your HTML file in browser
and looking at developer console won't work. Safari would complain about
cross-domain request error for `fetch`. So we need to serve these resources
locally. I usually use the built in server module from Python standard library
for this kind of things:

```
# In your source folder, run
python -m SimpleHTTPServer
```

Now go to <http://localhost:8000> and click on your HTML file. If everything
went well, you should see a WebAssembly instance logged in the developer
console.

Congratulations! You can start writing WebAssembly locally. Just remember to
re-compile `main.wat` with `wat2wasm` whenever you want to test things out in
browser.

## An Actual "Hello, World!" Implementation

This is my implementation:

```lisp
(module
  ;; Allocate a page of linear memory (64kb). Export it as "memory"
  (memory (export "memory") 1)

  ;; Write the string at the start of the linear memory.
  (data (i32.const 0) "Hello, world!") ;; write string at location 0

  ;; Export the position and length of the string.
  (global (export "length") i32 (i32.const 12))
  (global (export "position") i32 (i32.const 0)))
```

In other words, we expose information of the linear memory we manipulated to the
JavaScript environment. Things that has been `export`ed will show up as
properties of `exports` of the `WebAssembly` instance. We can access them in the
`main` JavaScript functions:

```javascript
function main(wasm) {
    const memory   = wasm.exports.memory;
    const length   = wasm.exports.length;
    const position = wasm.exports.position;
    ...
}
```

Then it's just plain-old Javascript (tho I had to steal it from tutorials).
`memory.buffer` is of type [ArrayBuffer][6]. We need to convert it into a string
and log it to the console:

```javascript
function main(wasm) {
    const memory   = wasm.exports.memory;
    const length   = wasm.exports.length;
    const position = wasm.exports.position;

    const bytes = new Uint8Array(memory.buffer, position, length);
    const s = new TextDecoder('utf8').decode(bytes);

    console.log(s);
}
```

Et, voilà! `Hello, World!` hot off a Web Assembly module in your developer
console. To conclude, I personally like to use a `Makefile` to streamline some
of the typing. Here's what I used for this demo:

```makefile
compile:
	wat2wasm main.wat -o main.wasm

serve:
	python -m SimpleHTTPServer
```

## Conclusion

No fancy schmancy Javascript build stack, no 3rd-party code dependency. Write
code, compile, run on your (virtual, in browser) machine, repeat. That sounds like
"assembly" to me!

[1]: https://github.com/WebAssembly/wabt
[2]: https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format
[3]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
[4]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise
[5]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly
[6]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer

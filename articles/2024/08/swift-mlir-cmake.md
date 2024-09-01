# Notes on Using the MLIR C API in Swift
2024-08-31T18:44:48-07:00
tag: Swift, MLIR, CMake, LLVM

For curiosity's sake, I decided I want to play with MLIR's C API with Swift.
I spent quite some time to get a skeleton project up and running on my Mac.
Here's my notes for future reference. (If you find this useful, I'd be curious
to know what you're working on!).

Modern LLVM comes shipped with MLIR. At the time of writing, all I had to do to
get it is `brew install llvm`. If you used the default Homebrew installation
options, you'll find `libMLIR.dylib` and friends under
`/opt/homebrew/opt/llvm/`. Building MLIR following the
[instructions](https://mlir.llvm.org/getting_started/) on the website is also
fairly straightforward.

You'll want `llvm-config` from your version of LLVM to be in your path. For the
Homebrew-installed version, you want `/opt/homebrew/opt/llvm/bin/` to be one of
the place the shell looks.

Now, it's time to make the project. With CMake, of course. Because I couldn't
figure out how to tell SwiftPM to link the right dylib :) But worry not, CMake
ain't that bad.

Like with SwiftPM, we want to make a module for the MLIR C API. I call the module
`cmlir`. Make a directory with that name, and create 2 text files:

First, `module.modulemap`:

```
module cmlir [system] {
  header "shim.h"
  export *
}
```

Second, `shim.h`:
```c
#include <mlir-c/IR.h>
```

Amazing.

Let's assume we want to have a Swift library that uses `cmlir`. And a executable
that depends on the library. You can organize the Swift source files for these
as you like (yay CMake!).

The sample library has one file, `lib.swift`:

```swift
import cmlir

public func makeAContext() -> MlirContext {
    mlirContextCreate()
}
```

The sample app is just a `main.swift`:

```swift
import MLIRSwift

print(makeContext())
```

... as you can see, through these targets, we are expecting to properly execute
some code from MLIR.

All that's left is to build all these stuff. AKA, the hard part! But the
`CMakeLists.txt` really isn't that bad. I'll just leave it here with comments:

```cmake
cmake_minimum_required(VERSION 3.22)

# Note we include "C" here, without it there'd be a build error 🤷
project(swift-mlir LANGUAGES C CXX Swift)

# This is where llvm-config comes to play
find_package(MLIR REQUIRED CONFIG)

include_directories(${MLIR_INCLUDE_DIRS})

# Include our modulemap
include_directories(cmlir)

# I can't believe this is all it takes to make a Swift dylib!
add_library(MLIRSwift SHARED lib.swift)

# Wasted a lot of time on figuring out the right library to link T-T
target_link_libraries(MLIRSwift PRIVATE MLIRCAPIIR)

# Nothing special here
add_executable(myapp main.swift)
target_link_libraries(myapp PRIVATE MLIRSwift)
```

And there you have it. Here's the file structure in the end:

```
.
├── CMakeLists.txt
├── cmlir
│   ├── module.modulemap
│   └── shim.h
├── lib.swift
└── main.swift
```

For completeness, I'll also include commands that builds this project. It's just
the simplest cmake commands. But it may not be obvious for Swift programmers:

```
make build # make a bulid direcory anywhere, make sure you .gitignore it if necessary
cd build
cmake -G Ninja ..
cmake --build .
```

With this sample project, running `build/myapp` should get you this output:

```
MlirContext(ptr: Optional(0x0000600001784180))
```

And that's just exciting, isn't it?

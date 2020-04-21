## Overview

This is a CLI program that generates https://duan.ca . It takes 3 inputs:

1. URL of the site
2. Directory containing the input
3. Directory for outputing the generated site

## Syntax Highlighting

Syntax highlighting is achieved thanks to [syntect][]. Notably, SublimeText
syntax files are embedded in the generator's executable. This requires
a serialized file with all the syntax files. A helper program is written to
generate this file:

```
cargo run \
    --example sublime-syntax-dumper \
    path_to_folder_of_sublime_syntaxes \
    src/syntax.dump
```

So, updating syntaxes essentially boils down to updating the syntax files and
then re-generate this binary dump as instructed above.

[syntect]: https://github.com/trishume/syntect

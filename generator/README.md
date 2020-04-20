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

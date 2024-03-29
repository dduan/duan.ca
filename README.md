## Overview

This is the source code for https://duan.ca . The website is static. The
compiler that outputs the site is in `/generator`.

The compiler, tho tailor built for this website, actually pretends to be
generic. That is, it in theory could generate other sites, if the sites follow
the convention of the inputs and have similiar structures.

The generator can be built to portable executables, which are checked in as part
of the project in `/bin`. Adding content to the site shouldn't require
rebuilding the executables.

See  `make build` in `Makefile` for how to use the generator.

## Updating the site

Things to do to add some content. After the described step, regenerate the site
(duh).

### Adding an article

Add a Markdown document in `/articles`. It must begin with the following format

```markdown
# Title
2020-04-21T01:16:35-07:00
tags: tag1, tag2
```

So a h1 title, a RFC3339 date, and a list of tags starting with `tags: `,
separated by commas.

The relative path to `/articles` of the Markdown file becomes the article's
relative URL.

### Adding a web page

Add a HTML file in `/pages`. Its file name will become `<title>`. Its content
will become the `<body>` of the base template. Its relative path in `/pages`
becomes its URL. For example, `/pages/you/Have a nice day.html` will become
`example.com/you/`.

### Adding an asset

Put it in `/static`. Its relative path to `/static` becomes its URL.

### Adding a new type of page

1. Add a template in `generator/templates`.
2. Add a template type in `generator/src/templates.rs`.
3. Create instances of the templates, render them, and write them to disk in
   `generator/src/builder.rs`.
4. Rebuild generator.

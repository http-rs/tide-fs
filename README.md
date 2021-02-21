# tide-fs
File system handlers for the Tide web-framework


Tide-fs contains extensions for the Tide web-framework to serve files or whole directories from your file-system. Tide-fs provides a `ServeDir` and a `ServeFile` endpoint. It also provides a convenient extension trait to the `tide::Route` type that allow you to add these endpoints with the `serve_dir` and `serve_file` methods. `ServeFile` serves a single file on a single route;
```rust
use tide_fs::prelude::*;

let mut app = tide::new();
app.at("index.html").serve_file("examples/content/index.html")?;
```

`ServeDir` maps a section of a route to files in a directory
```rust
use tide_fs::prelude::*;

let mut app = tide::new();
app.at("static/").serve_dir("examples/content/")?;
```

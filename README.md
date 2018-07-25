# Scale Model of Buck + Rust monorepo

This is a small-scale model of a Rust + Buck monorepo, incorporating:
- libraries
- a top-level executable
- prebuilt third-party libraries

## Getting Started: Build Buck

In a separate dir, check out the open-source buck release:
```
$ git clone https://github.com/facebook/buck.git
```

Then build it:
```
$ cd buck
$ ant
```
(Buck needs a Java build environment.)

## Getting Started: Build Third-party Deps
In this dir, run:
```
$ ./setup.sh
```
This runs `cargo` to download and build a selection of crates which are used by monorepo code.

# Using Buck

Buck is in `~/git/buck/bin/buck` (or wherever you checked it out).

Buck's rules are in `BUCK` files (usually). Each dir with one or more targets will have its own `BUCK` file
specifying rules for those targets. The rules specify a full dependency graph, and references between dirs are
always absolute relative to the top-level directory (where `.buckconfig` is, which is why its present even if
its empty).

The syntax is basically Python.

## Building things

Build the server executable and show its location in `buck-out`:
```
$ buck build --show-output server:server
```

Just run it immediately:
```
$ buck run server:server
```

Check-build a library:
```
$ buck build project1:project1#check
```

Generate save-analysis (nightly/dev rustc only):
```
$ buck build --show-json-output project1:project1#save-analysis
```

## Querying things

(Official docs: https://buckbuild.com/article/query_cheat_sheet.html#content)

Map from source file to owning rule
```
$ buck query 'owner(project1/src/lib.rs)'
```
(add `--json` to do the obvious)

Rust deps for a rule:
```
$ buck query 'deps(//server:server)'
```

Combining several different terms, find all Rust library, binary or unittest rules which are transitive
dependencies of the rule owning a given source file:
```
$ buck query 'kind("^rust_(binary|library|unittest)$", deps(owner("server/src/main.rs")))'
```
Find the paths of any prebuilt deps (`--output-attributes` implies json output):
```
$ buck query 'kind("prebuilt_rust_library", deps(owner("server/src/main.rs")))' --output-attributes name crate
```

You can use `--output-attributes .*` to see all the attributes for the selected rules.

# Putting it together

You can use `buck query` to get the list of rule names, then 
```
$ buck build --show-json-output rule1#save-analysis rule2#save-analysis
```
to get save-analysis data for all the in-tree things.

`prebuild_rust_library` doesn't support the `#save-analysis` flavour, so I think the best thing to do for now is
compute the save-analysis `.json` file path from the `.rlib` path.
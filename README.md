# Xi Editor Demo Plugin
This repository contains a plugin that demonstrates how errors and warnings
could be added as annotations to the Xi Editor.

It is based off of the template found
[here](https://github.com/xi-editor/xi-editor/tree/master/rust/sample-plugin).

### Installation
Either

run `make install`.

or

move the compiled binary and manifest.toml to a directory located in
`$XI\_CONFIG\_DIR/plugins`. Within that directory, the binary should be placed
in a `bin/` subdirectory.

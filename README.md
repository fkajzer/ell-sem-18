# ell-sem-18
working with files in rust, evaluation of languages and libraries

requirements: [cargo and rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

```
curl https://sh.rustup.rs -sSf | sh

source $HOME/.cargo/env
```

Use `python3 generate-fake-data.py` to create fake directories with song names.

Use `pwd` in your terminal to get the working directory of the tracks/ORG/NML.
Then update the locations of your files in `src/main.rs`:

# example

```
const TRACKS_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const TRACKS_TARGET_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const ORG_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/org/tracks.org";
const NML_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/nml/collection.nml";

...

const DEBUG_MODE: DebugMode = DebugMode::WRITETODEBUGFILES;
```

Use `cargo build` to build the project and install all dependencies.

After specifying their file path in `src/main.rs`, use `cargo run` to update the ORG and NML.

The `DebugMode` is set to `WRITETODEBUGFILES` as default, which will create a copy of your ORG and NML file in the same folder of the original file.
The ORG will be created completely new, the NML will be a copy with overwritten values from the original NML file.

If you wish to overwrite your files directly, set `DebugMode` to `NONE`.

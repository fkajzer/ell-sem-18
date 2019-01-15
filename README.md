# ell-sem-18
working with files in rust, evaluation of languages and libraries

requirements: [cargo and rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

use `pwd` on in your terminal to get the working directory of the tracks/org/nml then update the locations of your files in `src/main.rs`:

#example

```
const TRACKS_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const TRACKS_TARGET_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const ORG_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/org/tracks.org";
const NML_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/nml/collection.nml";
```

The `DebugMode` is set to `WRITETODEBUGFILES` as default, which will create a copy of your org and nml file in the same folder.

If you wish to overwrite your files, set `DebugMode` to `None`.

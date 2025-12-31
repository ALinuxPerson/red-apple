# red apple

what if video players empowered everyone to build reliable and efficient, memory safe, and blazingly fast software?

- **we-have-the-technology**: bad apple generator. NOT vibe coded. run with `WHTT_NO_DRY_RUN=1 cargo run <path to directory>`
- **wet-appl**: the dynamic library i had to load in which hooks fs system calls to sanitize/fix/ignore the long file names for each frame. probably only supports macos. its also vibe coded, so please don't have any faith in the code
- **whynot**: generated output. have fun! run `RUSTFLAGS="-Awarnings" DYLD_INSERT_LIBRARIES=<path of libwet_appl.dylib> cargo check` on a 96x39 terminal (or was it 96x38? i forgot)

# fun facts

- every crate name represents one frame. if you only include the 96x36 resolution, that means the crate name is at least **3456 characters long**. this doesn't include the frame number and the padding bytes to ensure the video is actually aligned with the terminal
- since cargo generates files on the target directory based on the crate name rather than the directory name, this means it will easily fail without loading in the dynamic library hooks.
- even then, _i still_ get warnings about the file names being too long, but at least they're just warnings instead of errors. easily suppressed by passing `RUSTFLAGS="-Awarnings"`.

# license

this project is licensed under the unlicense, because who the hell would want to use this code?

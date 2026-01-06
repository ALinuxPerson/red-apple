<div align="center">
  <a href="https://www.youtube.com/watch?v=EZr698boQ-Y">
    <img src="https://img.youtube.com/vi/EZr698boQ-Y/maxresdefault.jpg" alt="Watch the video" style="width:600px;">
  </a>
</div>

# red apple

what if bad apple empowered everyone to build reliable and efficient, memory safe, and blazingly fast software?

- **we-have-the-technology**: bad apple generator. NOT vibe coded. instead, it is drunk coded. 
- **wet-appl**: the dynamic library i had to load in which hooks fs system calls to sanitize/fix/ignore the long file names for each frame. probably only supports macos. its, unfortunately, vibe coded (euugghhh 🤮🤮🤮), so please don't have any faith in the code or use it as a sample on how to hook into function calls with a dynamic library
- **whynot**: generated output. the name speaks for itself.

## what's with the names of the crates?

i've got not clue either. i just blacked out and woke up with 107mb worth of text files.

# how to run (you masochist)

0. make sure you're running on macOS, as this relies on macOS-specific features (`DYLD_INSERT_LIBRARIES`). yes, i know, burn me at the stake.

1. clone the github repo and change directories to it **(IMPORTANT: you should probably clone it onto a temporary ramdisk. `whynot` would ram your SSD with small random reads/writes, which i'm not so sure will have any noticeable effect, but it's good to be cautious just in case. unless, you only have 8 gb of ram or something. then just rawdog it i guess, this repo is only 107 mb _including_ `whynot`)**.

```bash
$ git clone https://github.com/ALinuxPerson/red-apple.git && cd red-apple
```

2. go to `wet-appl` and build it.

```
$ cd wet-appl/ && cargo build --release && cd ..
```

2. follow the instructions in the next section to either use the pre-built one (`whynot`) or generate one.

3. you cannot rawdog it just by using `cargo check` alone. if you do, cargo will _easily_ overflow the stack. you need to set the stack size to unlimited. nothing bad could possibly happen giving a process held up by hopes and dreams an unlimited stack size, right?

```bash
$ ulimit -s unlimited
```

4. run this command on a 96x39 terminal (or was it 96x38? i forgot). `RUSTFLAGS` disable cargo warnings, and `DYLD_INSERT_LIBRARIES` loads the `wet-appl` library hook we built earlier.

```bash
$ RUSTFLAGS="-Awarnings" DYLD_INSERT_LIBRARIES=<path of libwet_appl.dylib> cargo check
```

5. have fun!

---

## prebuilt using **whynot**

1. change directories to `whynot`.

```bash
$ cd whynot
```

## generating it (you goober)

1. change directories to `we-have-the-technology`.

```bash
$ cd we-have-the-technology
```

2. run it, passing the directory you want the generated cargo project to be in--DRY RUN FIRST. **(IMPORTANT: directory should be on a ramdisk for the same reasons as above)**

```bash
$ cargo run <path to directory>
```

3. once you're completely sure where you want to put it, run it without protection.

```bash
$ WHTT_NO_DRY_RUN=1 cargo run <path to directory>
```

4. change directories to the path you gave it.

```bash
$ cd <path to directory>
```

# fun facts

- in order to play bad apple, the dependency graph must form a straight line. that is, we start at frame0, which has no dependencies. frame1 depends on frame0, frame2 depends on frame1, frame{n} depends on frame{n-1}, and so on.
- every crate name represents one frame. if you only include the 96x36 resolution, that means the crate name is at least **3456 characters long**. this doesn't include the frame number and the padding bytes to ensure the video is actually aligned with the terminal
- since cargo generates files on the target directory based on the crate name rather than the directory name, this means it will easily fail without loading in the dynamic library hooks.
- even then, _i still_ get warnings about the file names being too long, but at least they're just warnings instead of errors. easily suppressed by passing `RUSTFLAGS="-Awarnings"`.
- this may or may not consume 93 gb of memory when you run `cargo check`.
- i could probably refactor this project to not operate on just bad apple, but on any video file, just convert the video to ascii art of _s and Xs, specify the ideal terminal size, and boom. as for the implementation of that, though, i leave that as an exercise to the reader :)

# motivations

i saw, like, a resurgence of bad apple memes on youtube because of a... _certain occurrence_ regarding a man and his island whose incriminating files got released that happened, and i was compiling a bevy project and then i thought to myself while i saw the crate names scroll down on my screen, what if we could run bad apple on the cargo compilation output?

# faq

1. why?

why not?

2. should i actually run this?

no, but you're going to do it anyway, are you?

3. is this a war crime?

the geneva conventions doesn't cover software development yet, so as of the time of writing, no.

# license

this project is licensed under the unlicense, because who the hell would want to use this code? i am not liable as to whatever you use this monstrosity for, so DO NOT use it in produ--actually, i want to see _how_ you'd use it in production. don't blame though if you cause an oopsie woopsie :3

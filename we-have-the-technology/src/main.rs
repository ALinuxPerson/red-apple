use fs_err as fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::{env, io, thread};
use prev_iter::PrevPeekable;
use threadpool::ThreadPool;

const CURSED: &str = include_str!("../cursed.json");
const PADDING: &str = "______________________________________________________________________________";

const WORKSPACE_CARGO_TOML: &str = r#"[workspace]
members = ["crates/*"]
resolver = "3""#;

enum FsAction {
    CreateDirAll(PathBuf),
    Write(PathBuf, Vec<u8>),
}

impl FsAction {
    fn execute(self) -> io::Result<()> {
        match self {
            Self::CreateDirAll(path) => fs::create_dir_all(path),
            Self::Write(path, contents) => fs::write(path, contents),
        }
    }

    fn describe(&self) -> String {
        match self {
            Self::CreateDirAll(path) => format!("CREATE DIRECTORY ALL {}", path.display()),
            Self::Write(path, contents) => format!(
                "WRITE PATH {} WITH CONTENTS LEN={}",
                path.display(),
                contents.len()
            ),
        }
    }
}

enum FsActions {
    Sequential(Vec<FsAction>),
    Parallel(Vec<FsAction>),
}

fn main() {
    let not_dry_run = env::var("WHTT_NOT_DRY_RUN").is_ok();
    let out_dir: PathBuf = env::args().nth(1).unwrap().into();
    let crates_dir = out_dir.join("crates");
    let cursed: Vec<Vec<String>> = serde_json::from_str(CURSED).unwrap();
    let num_frames = cursed.len();
    let mut fs_actions = Vec::with_capacity(2);
    fs_actions.push(FsActions::Sequential(vec![
        FsAction::CreateDirAll(crates_dir.clone()),
        FsAction::Write(out_dir.join("Cargo.toml"), WORKSPACE_CARGO_TOML.into()),
    ]));

    let mut frames = PrevPeekable::new(cursed.iter().enumerate());
    while let Some((i, frame)) = frames.next() {
        let mut parallel_fs_actions = Vec::with_capacity(3);
        let frame = frame.join("");
        let crate_dir = crates_dir.join(format!("f{i:04}"));
        let src_dir = crate_dir.join("src");
        parallel_fs_actions.push(FsAction::CreateDirAll(src_dir.clone()));
        let crate_name = format!("f{i:04}{PADDING}{frame}");
        let mut cargo_toml = format!(
            r#"[package]
name = "{crate_name}"
version = "0.6.7"
edition = "2024"
"#
        );
        if (1..num_frames).contains(&i)
            && let Some((next_i, next_frame)) = frames.prev_peek()
        {
            let next_frame = next_frame.join("");
            let next_crate_dir = Path::new("..").join(format!("f{next_i:04}"));
            let next_crate_name = format!("f{next_i:04}{PADDING}{next_frame}");
            cargo_toml.push_str(&format!(
                r#"[dependencies.{next_crate_name}]
path = "{}""#,
                next_crate_dir.display()
            ));
        }
        parallel_fs_actions.push(FsAction::Write(
            crate_dir.join("Cargo.toml"),
            cargo_toml.into(),
        ));
        parallel_fs_actions.push(FsAction::Write(
            src_dir.join("lib.rs"),
            b"#![allow(nonstandard_style)]".to_vec(),
        ));
        fs_actions.push(FsActions::Parallel(parallel_fs_actions));
    }

    let (printer_tx, printer_rx) = mpsc::channel();
    let printer_handle = thread::spawn(move || {
        for message in printer_rx {
            println!("{message}")
        }
    });
    let pool = ThreadPool::new(thread::available_parallelism().unwrap().get());
    for actions in fs_actions {
        match actions {
            FsActions::Sequential(actions) => {
                for action in actions {
                    let describe = action.describe();
                    printer_tx.send(describe.clone()).unwrap();

                    if not_dry_run {
                        action.execute().unwrap()
                    }

                    printer_tx.send(format!("DONE: {describe}")).unwrap();
                }
            }
            FsActions::Parallel(actions) => {
                let printer_tx = printer_tx.clone();
                pool.execute(move || {
                    for action in actions {
                        let describe = action.describe();
                        printer_tx.send(describe.clone()).unwrap();
                        if not_dry_run {
                            action.execute().unwrap();
                        }
                        printer_tx.send(format!("DONE: {describe}")).unwrap();
                    }
                })
            }
        }
    }
    drop(printer_tx);
    pool.join();
    printer_handle.join().unwrap()
}

use notify::INotifyWatcher;
use notify::{Config, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

fn get_paths() -> (PathBuf, PathBuf) {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        println!("Usage: {} <path1> <path2>", args[0]);
        std::process::exit(1);
    }

    let local_path = PathBuf::from(&args[1]);
    let remote_path = PathBuf::from(&args[2]);

    if !local_path.exists() {
        println!("Path {} does not exist", local_path.to_str().unwrap());
        std::process::exit(1);
    }

    (local_path, remote_path)
}

fn sync(source: &Path, destination: &Path) {
    Command::new("rclone")
        .args(&[
            "sync",
            "--update",
            &source.to_str().unwrap(),
            &destination.to_str().unwrap(),
        ])
        .output()
        .unwrap();
}

fn main() {
    let (local_path, remote_path) = get_paths();
    let (sender, receiver) = channel();
    let config = Config::default().with_poll_interval(Duration::from_secs(2));

    let mut watcher: INotifyWatcher = Watcher::new(sender, config).unwrap();

    watcher
        .watch(&local_path, RecursiveMode::Recursive)
        .unwrap();

    loop {
        std::thread::sleep(Duration::from_secs(1));
        sync(&remote_path, &local_path);

        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {
                println!("Change detected, syncing...");
                sync(&local_path, &remote_path);
            }
            Err(_) => {}
        }
    }
}

use notify::INotifyWatcher;
use notify::{Config, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct Syncer {
    local_path: PathBuf,
    remote_path: PathBuf,
}

impl Syncer {
    pub fn new() -> Syncer {
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

        Syncer {
            local_path,
            remote_path,
        }
    }

    fn sync(&self, source: &Path, dest: &Path) {
        Command::new("rclone")
            .args(&[
                "sync",
                "--update",
                source.to_str().unwrap(),
                dest.to_str().unwrap(),
            ])
            .output()
            .unwrap();
    }

    pub fn watch(&self) {
        let (sender, receiver) = channel();
        let config = Config::default().with_poll_interval(Duration::from_secs(2));

        let mut watcher: INotifyWatcher = Watcher::new(sender, config).unwrap();

        watcher
            .watch(&self.local_path, RecursiveMode::Recursive)
            .unwrap();

        loop {
            std::thread::sleep(Duration::from_secs(1));
            self.sync(&self.remote_path, &self.local_path);

            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(_) => self.sync(&self.local_path, &self.remote_path),
                Err(_) => {}
            }
        }
    }
}

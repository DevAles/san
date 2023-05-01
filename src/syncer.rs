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
    pub fn new(local_path: &str, remote_path: &str) -> Syncer {
        let local_path = PathBuf::from(local_path);
        let remote_path = PathBuf::from(remote_path);

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

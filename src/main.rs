mod syncer;

use syncer::Syncer;

fn main() {
    let syncer = Syncer::new();
    syncer.watch();
}

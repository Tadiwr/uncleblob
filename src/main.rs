mod providers;

use providers::utils::gitignore;

fn main() {
    gitignore::add_to_gitignore("/.blob-store");
}


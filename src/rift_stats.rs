pub struct RiftStats {
    game_folders: Vec<String>
}

impl RiftStats {
    pub fn new(game_folder: String) -> RiftStats {
        RiftStats {
            game_folders: vec![game_folder]
        }
    }
    pub fn add_game_folder(&mut self, game_folder: String) {
        self.game_folders.push(game_folder);
    }

    pub fn parse_game_folders(&self) {
        for game_folder in &self.game_folders {
            println!("{}", game_folder);
        }
    }
}
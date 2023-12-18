use std::collections::{BTreeMap, HashMap};
use std::fs;
use serde_json::Value;

pub struct RiftStats {
    game_folders: Vec<String>,
    games: Vec<Value>
}

impl RiftStats {
    pub fn new(game_folder: String) -> RiftStats {
        RiftStats {
            game_folders: vec![game_folder],
            games: vec![]
        }
    }
    pub fn add_game_folder(&mut self, game_folder: String) {
        self.game_folders.push(game_folder);
    }

    pub fn parse_game_folders(&mut self) {
        for game_folder in &self.game_folders {
            for path in fs::read_dir(format!("{}", game_folder)).unwrap() {
                //println!("{}", path.unwrap().file_name().into_string().unwrap());
                let file_path = path.unwrap().path().to_str().unwrap().to_string();
                let mut file_string = fs::read_to_string(file_path).expect("bla");
                let v: Value = serde_json::from_str(file_string.as_str()).expect("TODO: panic message");
                self.games.push(v);
                //println!("{}", v["info"]["gameDuration"]);
            }
        }
    }

    pub fn get_average_game_length(&self) -> f64 {
        let num_games = self.games.len();
        let mut added_overall: i64 = 0;
        for game in &self.games {
            added_overall += game["info"]["gameDuration"].clone().as_i64().unwrap();
        }
        return added_overall as f64/num_games as f64/ 60_f64;
    }

    pub fn get_num_played_champions_list(&self) -> HashMap<String, i64> {
        let mut champion_list: HashMap<String, i64> = HashMap::new();

        for game in &self.games {
            if let Some(participants) = game["info"]["participants"].as_array() {
                for participant in participants {
                    let champ_name: &String = &participant["championName"].to_string();
                    if champion_list.contains_key(champ_name) {
                        champion_list.insert(champ_name.to_string(), champion_list.get(champ_name).unwrap() + 1);
                    }
                    else {
                        champion_list.insert(champ_name.to_string(), 1);
                    }
                }
            }
        }
        return champion_list;
    }
}

pub fn sort_hashmap_to_vec(input_map: &HashMap<String, i64>) -> Vec<(&String, &i64)> {
    let mut out: Vec<(&String, &i64)> = Vec::new();
    println!("map_len: {}", input_map.len());

    for input in input_map {
        for (index, e) in out.clone().iter().enumerate() {
            if input.1 > e.1 {
                println!("{:?}", input);
                out.insert(index, input);
                break;
            }
            if index == out.len()-1 {
                out.push(input);
            }
        }

        if out.len() == 0 {
            out.push(input);
        }
    }
    return out;
}
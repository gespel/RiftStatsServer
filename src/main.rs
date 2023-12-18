use std::collections::HashMap;
use crate::rift_stats::{RiftStats, sort_hashmap_to_vec};

mod rift_stats;
mod classic_game;

fn main() {
    let mut rs = RiftStats::new("games/classic".to_string());
    //rs.add_game_folder(".//gamesa/classic".to_string());
    rs.parse_game_folders();
    let cl = rs.get_num_played_champions_list();
    let sorted_map = sort_hashmap_to_vec(&cl);

    println!("{:?}", sorted_map);
    //println!("{:?}", rs.get_champions_list());

}

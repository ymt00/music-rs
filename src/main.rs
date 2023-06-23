use json::JsonValue;
use std::{collections::VecDeque, env, fs, process::Command, str};

use sway::{bemenu, get_apps, get_tree, scratchpad_show, Node};

const BEMENU_ARGS: [&str; 2] = ["-p", "\u{f001} Musique"];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You must provide a path to a sources music json file.");
    }

    let apps: String = get_tree()["nodes"]
        .members()
        .map(|w: &JsonValue| get_apps(Node::new(w)) + "\n")
        .collect::<String>()
        .trim_end_matches("\n")
        .to_string();

    let show_menu: &str = apps
        .lines()
        .find(|&l| l == "play-radio" || l == "yewtube")
        .unwrap_or("show_menu");

    if show_menu == "show_menu" {
        let music_sources = json::parse(
            fs::read_to_string(&args[1])
                .expect("Unable to read sources music file")
                .as_str(),
        )
        .unwrap();

        let sel: String = bemenu(
            music_sources
                .entries()
                .map(|e: (&str, &JsonValue)| e.0.to_owned() + "\n")
                .collect::<String>()
                .trim_end_matches("\n"),
            &BEMENU_ARGS,
        );

        if !sel.is_empty() {
            let prog: String = music_sources[sel].to_string();
            let mut args: VecDeque<&str> = prog.split_ascii_whitespace().collect();

            Command::new(args.pop_front().unwrap())
                .args(args)
                .spawn()
                .expect("Command failed");
        }
    } else {
        scratchpad_show("play-radio|yewtube");
    }
}

use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, SelectView, ResizedView, NamedView};
use cursive::traits::*;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize, Clone)]
struct Rice {
	name: String,
	alacritty_path: String,
	wall_path: String
}


fn on_submit(s: &mut Cursive, value: &Rice) {
	
	let home_dir: PathBuf = match home::home_dir() {
		Some(path) => path,
		None => panic!("unable to detect home directory"),
	};
	let old_alacritty_config_path = home_dir.join(".alacritty.yml");
	let backup_alacritty_config_path = home_dir.join(".alacritty.yml.bak");
	let new_alacritty_config_path = home_dir.join(&value.alacritty_path);
	
	let new_wallpaper_path = home_dir.join(&value.wall_path);
	
	let vs_code_config_path = home_dir.join(".config/Code - OSS/User/settings.json");

	fs::copy(&old_alacritty_config_path, backup_alacritty_config_path).expect("could not backup the alacritty config");
	fs::copy(new_alacritty_config_path, old_alacritty_config_path).expect("could not paste over the new alacritty config");
	
	Command::new("feh")
			.arg("--bg-scale")
			.arg(new_wallpaper_path.to_str().expect("could not get wallpaper path"))
			.spawn().expect("could not set wallpaper. is feh installed?");
	
	s.add_layer(Dialog::text(format!("You selected: {}", value.name))
        .title(format!("{}'s info", value.name))
	.button("Ok",   |s| { s.pop_layer(); }));

}

fn generate_select_list() -> ResizedView<NamedView<SelectView<Rice>>>
{
	let width: u8 = 20;
	let height: u8 = 10;
	let select_view = SelectView::<Rice>::new()
		.on_submit(on_submit)
        	.with_name("select")
        	.fixed_size((width, height));

	return select_view;
}

fn read_file(file_path : &str) -> String
{
	return fs::read_to_string(file_path).expect("ERROR READING CONFIG FILE")
}

fn decode_config_file(file_contents : &str) -> Vec<Rice>
{
	return serde_json::from_str(file_contents).expect("JSON was not in the expected format");
}

fn main() {

	let configuration: Vec<Rice> = decode_config_file(&read_file("assets/config.json"));
	
	let mut siv = cursive::default();
	siv.add_global_callback('q', |s| s.quit());
    let select_view = generate_select_list();

	siv.add_layer(Dialog::around(LinearLayout::horizontal()
	    .child(select_view))
            .title("Select a Rice"));	
	
		siv.call_on_name("select", |view: &mut SelectView<Rice>| {
		for rice  in 0..configuration.len(){
			view.add_item(&configuration[rice].name, configuration[rice].clone());
		}
	});
    siv.run();
}
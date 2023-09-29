use std::fs;
use std::path::PathBuf;

// Get the mod's slot for one slot effects
fn get_mod_slot(effect_path: &PathBuf, fighter_name: String) -> String{
    let mut result = "".to_string();
    let mut effect_subdirs = std::fs::read_dir(effect_path).unwrap();

    if let Some(dir) = effect_subdirs.next() {
        // Redefine dir to unwrap it, only way for it to work afaik
        let dir = dir.unwrap();
        // Get the current directory name, separate from the full path
        let file_name = dir.file_name();

        // Set result to the slot the effect file is on
        result = file_name.to_str().unwrap().strip_prefix(&(("ef_".to_string() + fighter_name.as_str()) + "_c")).unwrap().strip_suffix(".eff").unwrap().to_string();
    }
    result
}

// Check for one slot effects
fn is_one_slot(effect_path: &PathBuf, fighter_name: String) -> bool {
    let mut result = false;
    let effect_subdirs = std::fs::read_dir(effect_path).unwrap();

    for dir in effect_subdirs {
        // Redefine dir to unwrap it, only way for it to work afaik
        let dir = dir.unwrap();
        // Get the current directory name, separate from the full path
        let file_name = dir.file_name();

        // Check if the filename starts with `ef_{character_name}_c`, because thats the formatting for one slot effects
        if file_name.to_str().unwrap().starts_with(&(("ef_".to_string() + fighter_name.as_str()) + "_c")) {
            // Set result to true, because we found a one slot effect
            result = true;
        }
    }
    result
}

// Function to find the fighter's name from the effect path
fn find_fighter_name(effect_path: &PathBuf) -> String {
    let fighter_dir = std::fs::read_dir(effect_path).unwrap().next().unwrap().unwrap();

    return fighter_dir.file_name().as_os_str().to_string_lossy().to_string()
}

// Function to look for the effect folder inside of the mod folder.
fn look_for_effect(mod_folder: &PathBuf) -> bool {
    // Clone mod_folder into mod_path so that we dont override anything in mod folder
    let mut mod_path = mod_folder.clone();

    // Push effect and fighter to the path
    mod_path.push("effect");
    mod_path.push("fighter");

    // Return if this path exists
    mod_path.exists()
}

fn main() {
    let mut arc_path: String = String::new();
    let mut mods_path: String = String::new();

    // Get the data.arc path
    println!("Please input the path to the root of your extracted data.arc:");
    std::io::stdin().read_line(&mut arc_path).unwrap();

    // Get the ultimate/mods path
    println!("Please input the path to your ultimate/mods folder:");
    std::io::stdin().read_line(&mut mods_path).unwrap();

    // Redefine these while stripping the CRLF ending
    let arc_path: &str = arc_path.strip_suffix("\r\n").unwrap();
    let mods_path: &str = mods_path.strip_suffix("\r\n").unwrap();

    // Read the mods path to get the subdirectories
    let mods = std::fs::read_dir(mods_path).unwrap();

    // Iterate over the mod folders
    for mod_folder in mods {
        // Check if the mod is a fighter effect mod
        let is_effect = look_for_effect( &mod_folder.as_ref().unwrap().path());

        // If the mod is a fighter effect mod, check if it's one slot, and then look for
        // the trail and model directories
        if is_effect {
            // Create the effect path variable, and push the dir names needed
            let mut effect_path = mod_folder.as_ref().unwrap().path();
            effect_path.push("effect");
            effect_path.push("fighter");

            // Find the fighter name from the current effect path
            let fighter_name: String = find_fighter_name(&effect_path);

            // Append the fighter name to the effect path
            effect_path.push(fighter_name.clone());

            // Check if the mod contains one slot effects
            let is_one_slot = is_one_slot(&effect_path, fighter_name.clone());
            if is_one_slot {
                let slot = get_mod_slot(&effect_path, fighter_name);


                dbg!(slot);
            }
        }
    }
}


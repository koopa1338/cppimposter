use clap::{arg, command, Command};
use rand::Rng;
use walkdir::WalkDir;

// code comments are good for documentation so this is a comment uwu
fn get_files_ඞ() -> Vec<String> {
    let mut files_ඞ: Vec<String> = Vec::new();

    for entry_ඞ in WalkDir::new("./src/") {
        let entry_ඞ = entry_ඞ.unwrap();

        let path_ඞ = entry_ඞ.path().to_str().unwrap();

        if path_ඞ.ends_with(".rs") {
            files_ඞ.push(path_ඞ.to_string());
        }
    }

    files_ඞ
}

// wow! such random! ඞ
fn random_chance_ඞ() -> (bool, u8) {
    let mut rng_ඞ = rand::thread_rng();

    // pretty random to me :)
    let random_ඞ = 2;

    let imposter_ඞ = rng_ඞ.gen_range(1..=24);

    if random_ඞ == 2 {
        (true, imposter_ඞ)
    } else {
        (false, imposter_ඞ)
    }
}

fn main() {
    let args_ඞ = command!()
        .subcommand(
            Command::new("IAMTHEIMPOSTER")
                .about("Get rid of the other imposters by running the IMPOSTER-DETECTION-ALGORITHM")
                .arg(arg!([NAME])),
        )
        .get_matches();

    let files_ඞ = get_files_ඞ();

    let mut imposters_ඞ = 0;

    if args_ඞ.subcommand().is_none() {
        for path_ඞ in files_ඞ {
            let imp_ඞ = populate_file_ඞ(&path_ඞ);
            imposters_ඞ += imp_ඞ;

        }

        println!("found {imposters_ඞ} imposters in your code");
    } else {
        for path_ඞ in files_ඞ {
            loop {
                let imp_ඞ = remove_imposters_ඞ(&path_ඞ);

                if imp_ඞ == 0 {
                    break;
                }
            }
        }

        println!("eliminated all imposters from your sus code. it was a clean job");
    }
}

// OMG another comment!!!
fn populate_file_ඞ(path_ඞ: &str) -> u32 {
    let old_data_ඞ = std::fs::read_to_string(path_ඞ).unwrap();
    let new_data_ඞ: &mut Vec<u8> = &mut Vec::new();

    let mut imposters_ඞ = 0;

    for (index_ඞ, character_ඞ) in old_data_ඞ.as_bytes().iter().enumerate() {
        new_data_ඞ.push(*character_ඞ);

        if character_ඞ.to_ascii_lowercase() == 10 {
            if old_data_ඞ.len() > index_ඞ + 2 {
                if old_data_ඞ.as_bytes()[index_ඞ + 1] == 47
                    || old_data_ඞ.as_bytes()[index_ඞ + 2] == 47
                {
                    continue;
                }
            } else if old_data_ඞ.len() > index_ඞ {
                let (chance_ඞ, imposter_ඞ) = random_chance_ඞ();

                if chance_ඞ {
                    imposters_ඞ += 1;

                    add_imposter_ඞ(new_data_ඞ, imposter_ඞ);
                }
            }
        }
    }

    std::fs::write(&path_ඞ, new_data_ඞ).unwrap();

    imposters_ඞ
}

// ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ
fn remove_imposters_ඞ(path_ඞ: &str) -> u32 {
    let mut deleting_ඞ: bool = false;

    let mut imposters_ඞ = 0;

    let old_data_ඞ = std::fs::read_to_string(path_ඞ).unwrap();
    let new_data_ඞ: &mut Vec<u8> = &mut Vec::new();

    for (index_ඞ, character_ඞ) in old_data_ඞ.as_bytes().iter().enumerate() {
        if old_data_ඞ.len() > index_ඞ + 4 && index_ඞ > 4 {
            if old_data_ඞ.as_bytes()[index_ඞ + 1] == 47
                && old_data_ඞ.as_bytes()[index_ඞ + 2] == 47
                && old_data_ඞ.as_bytes()[index_ඞ + 3] == 46
                && old_data_ඞ.as_bytes()[index_ඞ + 4] == 63
            {
                imposters_ඞ += 1;
                deleting_ඞ = true;
            }

            if old_data_ඞ.as_bytes()[index_ඞ - 1] == 47
                && old_data_ඞ.as_bytes()[index_ඞ - 2] == 47
                && old_data_ඞ.as_bytes()[index_ඞ - 3] == 63
                && old_data_ඞ.as_bytes()[index_ඞ - 4] == 46
            {
                deleting_ඞ = false;
            }
        }

        if !deleting_ඞ {
            new_data_ඞ.push(*character_ඞ);
        }
    }

    std::fs::write(&path_ඞ, new_data_ඞ).unwrap();
    imposters_ඞ
}

// now the fun part!!! ඞ
fn add_imposter_ඞ(new_data_ඞ: &mut Vec<u8>, id_ඞ: u8) {
    let imposter_ඞ = match id_ඞ {
        1 => {
            r#"
//.?
//
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⣿⣿⣷⣶⣄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⣰⣾⣿⣿⡿⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⡟⠁⣰⣿⣿⣿⡿⠿⠻⠿⣿⣿⣿⣿⣧⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⣾⣿⣿⠏⠀⣴⣿⣿⣿⠉⠀⠀⠀⠀⠀⠈⢻⣿⣿⣇⠀⠀⠀
//⠀⠀⠀⢀⣠⣼⣿⣿⡏⠀⢠⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⡀⠀⠀
//⠀⠀⣰⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡇⠀⠀
//⠀⢰⣿⣿⡿⣿⣿⣿⡇⠀⠘⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⢀⣸⣿⣿⣿⠁⠀⠀
//⠀⣿⣿⣿⠁⣿⣿⣿⡇⠀⠀⠻⣿⣿⣿⣷⣶⣶⣶⣶⣶⣿⣿⣿⣿⠃⠀⠀⠀
//⢰⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀
//⢸⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠉⢉⣿⣿⠀⠀⠀⠀⠀⠀
//⢸⣿⣿⣇⠀⣿⣿⣿⠀⠀⠀⠀⠀⢀⣤⣤⣤⡀⠀⠀⢸⣿⣿⣿⣷⣦⠀⠀⠀
//⠀⢻⣿⣿⣶⣿⣿⣿⠀⠀⠀⠀⠀⠈⠻⣿⣿⣿⣦⡀⠀⠉⠉⠻⣿⣿⡇⠀⠀
//⠀⠀⠛⠿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠈⠹⣿⣿⣇⣀⠀⣠⣾⣿⣿⡇⠀⠀
//⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣦⣤⣤⣤⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⢿⣿⣿⣿⣿⣿⣿⠿⠋⠉⠛⠋⠉⠉⠁
//           ⠉⠉⠉⠁
//?.
"#
        }

        2 => {
            r#"
//.? ⠀
//      ⠀⠀⠀⠀⠀⢀⣀⣀⣴⣆⣠⣤⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣻⣿⣯⣘⠹⣧⣤⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠿⢿⣿⣷⣾⣯⠉⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⠜⣿⡍⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⠁⠀⠘⣿⣆⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⡟⠃⡄⠀⠘⢿⣆⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣁⣋⣈ ⣤⣮⣿⣧⡀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠈⢻⣿⣿⡄⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣸⣿⡏⠀⠀⠀⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠁⠀⠀⢰⣿⣿⣯⠁⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣷⡄⠀
//⠀⠀⣀⣤⣴⣶⣶⣿⡟⠀⠀⠀⢸⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣷⠀
//⠀⢰⣿⡟⠋⠉⣹⣿⡇⠀⠀⠀⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠀
//⠀⢸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀
//⠀⣸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠀⠀
//⠀⣿⣿⠁⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣧⠀⠀
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
//⠀⢿⣿⡆⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀
//⠀⠸⣿⣧⡀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀
//⠀⠀⠛⢿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⣰⣿⣿⣷⣶⣶⣶⣶⠶⠀⢠⣿⣿⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⣽⣿⡏⠁⠀⠀⢸⣿⡇⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⢹⣿⡆⠀⠀⠀⣸⣿⠇⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠀⠈⠻⣿⣿⣿⣿⡿⠏⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⠿⠿⠿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//?.
"#
        }

        3 => {
            r#"
//.?
//             ⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠈⢻⣿⣿⡄⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣸⣿⡏⠀⠀⠀⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠁⠀⠀⢰⣿⣿⣯⠁⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣷⡄⠀
//⠀⠀⣀⣤⣴⣶⣶⣿⡟⠀⠀⠀⢸⣿⣿⣿⣆🔴⠀⠀⠀⠀🔴⠀⠀⣿⣷⠀
//⠀⢰⣿⡟⠋⠉⣹⣿⡇⠀⠀⠀⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠀
//⠀⢸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀
//⠀⣸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠀⠀
//⠀⣿⣿⠁⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣧⠀⠀
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
//⠀⢿⣿⡆⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀
//⠀⠸⣿⣧⡀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀
//⠀⠀⠛⢿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⣰⣿⣿⣷⣶⣶⣶⣶⠶⠀⢠⣿⣿⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⣽⣿⡏⠁⠀⠀⢸⣿⡇⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⢹⣿⡆⠀⠀⠀⣸⣿⠇⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠀⠈⠻⣿⣿⣿⣿⡿⠏⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⠿⠿⠿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//?.
"#
        }

        4 => {
            r#"
//.?
//⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⠄⠄⣀⣀⣐⡀⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⢠⠄⣠⣶⣿⣿⣿⠿⠿⣛⣂⣀⣀⡒⠶⣶⣤⣤⣬⣀⡀⠄⢀⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⢀⣾⣿⣿⣿⡟⢡⢾⣿⣿⣿⣿⣿⣿⣶⣌⠻⣿⣿⣿⣿⣷⣦⣄⡀⠄⠄⠄⠄⠄
//⠄⠄⣈⣉⡛⣿⣿⣿⡌⢇⢻⣿⣿⣿⣿⣿⠿⠛⣡⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠄⠄⠄
//⠄⠺⠟⣉⣴⡿⠛⣩⣾⣎⠳⠿⠛⣋⣩⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠄⠄
//⠄⠄⠄⠘⢋⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠄
//⠄⠄⢀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠄
//⠄⠄⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⣀
//⠄⠄⠄⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠘⠛
//⠄⠄⠄⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⣀⣀⣠⣤
//⠄⠄⣀⣀⡙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢛⣩⠤⠾⠄⠛⠋⠉⢉
//⠄⠺⠿⠛⠛⠃⠄⠉⠙⠛⠛⠻⠿⠿⠿⠟⠛⠛⠛⠉⠁⠄⠄⣀⣀⣠⣤⣠⣴⣶⣼⣿
//?.
"#
        }

        5 => {
            r#"
//.?
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟥🟥
//🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟥🟥
//🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟥🟥
//🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥 ‎   ‎🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥 ‎ ‎  🟥🟥🟥🟥🟥🟥 
//?.
"#
        }

        // THICCCCCCC!!!!!!!
        6 => {
            r#"
//.?
//⣿⣿⣿⣿⣿⣿⣿⢿⠟⠛⠿⠻⠿⠿⠟⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⡿⠛⢙⣨⣥⣶⣶⣿⢿⣿⣿⣷⣦⣅⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⠟⢀⡴⠟⠋⢉⣀⣠⣤⣤⣤⣀⠉⠻⣿⣧⡈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⠀⠁⣠⣴⣾⣿⣿⣿⣿⣿⣿⣿⣷⠀⢻⣿⣇⠝⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡀⣼⡿⠟⠀⠙⣛⣬⠱⣿⣿⣿⣿⣿⣿
//⣿⣿⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⠿⠋⢀⠄⠁⣠⣶⣾⣿⣿⣿⡆⣼⣿⣿⣿⣿⣿
//⣿⣿⠀⣀⠙⣛⣛⣻⠛⠋⣉⣢⣤⣾⠃⣰⡄⠸⣿⣿⣿⣿⣿⣷⠘⣿⣿⣿⣿⣿
//⣿⣿⣤⢹⣷⣶⣶⣶⣾⣿⣿⣿⣿⣿⡄⠸⣷⠀⢻⣿⣿⡿⠟⠛⠡⣿⣿⣿⣿⣿
//⣿⣿⣿⠄⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠄⠻⠇⢈⠁⠀⠀⠲⠠⠞⠿⣿⣿⣿⣿
//⣿⣿⣿⣷⠈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣶⢤⠀⠀⢲⣿⣿⣿⣷⣤⡉⣻⣿⣿
//⣿⣿⣿⣿⣧⠈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣳⡀⢻⣿⣿⣿⣿⣷⠐⣿⣿
//⣿⣿⣿⣿⣿⣯⡈⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⡇⡆⣿⣿⣿⣿⡟⣀⣿⣿
//⣿⣿⣿⣿⣿⣿⣷⡀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⢃⡿⠿⠛⡋⣀⣾⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⣷⣀⠹⣿⣿⣿⣿⣿⣿⣿⠿⠋⢁⣠⣿⡦⠐⠀⢈⡙⢿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⣿⠋⢀⣿⣿⣿⣿⠟⢃⣤⣤⡀⠻⣿⣇⣠⣴⡿⠄⠹⣧⡸⣿
//⣿⣿⣿⣿⣿⣿⡿⠃⢠⣾⣿⣿⡿⢋⣤⣿⣿⣿⣿⣄⠈⢿⡿⠋⣠⣤⣀⠈⣡⣿
//⣿⣿⣿⠅⣀⣈⠁⣰⣿⣿⡿⠋⣤⣾⣿⣿⣿⣿⣿⣿⣷⣵⣂⣽⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⣄⠘⢿⣿⣿⠟⠋⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣷⣤⣬⣅⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//?.
"#
        }

        7 => {
            r#"
//.?
//⠄⠄⠄⠄⠄⠄⠄⠄⠄⢀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣶⣄⠄⠄⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⠄⠄⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⠄⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⣴⡿⠛⠉⠁⠄⠄⠄⠄⠈⢻⣿⣿⣿⣿⣿⣿⣿⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⢸⣿⡅⠄⠄⠄⠄⠄⠄⠄⣠⣾⣿⣿⣿⣿⣿⣿⣿⣷⣶⣶⣦⠄⠄⠄
//⠄⠄⠄⠄⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠄⠄
//⠄⠄⠄⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄
//⠄⠄⠄⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄
//⠄⠄⠄⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄
//⠄⠄⠄⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄
//⠄⠄⠄⠄⠄⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄
//⠄⠄⠄⠄⠄⠄⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠛⠛⠛⠃⠄⠄
//⠄⠄⠄⠄⠄⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄
//⠄⠄⠄⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄⠄⠄⠄
//⠄⠄⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠄⠄⠄⠄
//⠄⠄⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠄⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠄⠄⠄⠄
//⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⡿⠟⠁⠄⠄⠄⠻⣿⣿⣿⣿⣿⣿⣿⡿⠄⠄⠄⠄⠄
//⠄⠄⢸⣿⣿⣿⣿⣿⡿⠋⠄⠄⠄⠄⠄⠄⠄⠙⣿⣿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄
//⠄⠄⢸⣿⣿⣿⣿⣿⣧⡀⠄⠄⠄⠄⠄⠄⠄⢀⣾⣿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄
//⠄⠄⢸⣿⣿⣿⣿⣿⣿⣿⡄⠄⠄⠄⠄⠄⠄⣿⣿⣿⣿⣿⣿⣿⣷⠄⠄⠄⠄⠄
//⠄⠄⠸⣿⣿⣿⣿⣿⣿⣿⣷⠄⠄⠄⠄⠄⢰⣿⣿⣿⣿⣿⣿⣿⣿⠄⠄⠄⠄⠄
//⠄⠄⠄⢿⣿⣿⣿⣿⣿⣿⡟⠄⠄⠄⠄⠄⠸⣿⣿⣿⣿⣿⣿⣿⠏⠄⠄⠄⠄⠄
//⠄⠄⠄⠈⢿⣿⣿⣿⣿⠏⠄⠄⠄⠄⠄⠄⠄⠙⣿⣿⣿⣿⣿⠏⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠘⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄⠄⣿⣿⣿⣿⡏⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⢸⣿⣿⣿⣧⠄⠄⠄⠄⠄⠄⠄⢀⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄⠄⠄
//⠄⠄⠄⠄⠄⣸⣿⣿⣿⣿⣆⠄⠄⠄⠄⠄⢀⣾⣿⣿⣿⣿⣿⣄⠄⠄⠄⠄⠄⠄
//⠄⣀⣀⣤⣾⣿⣿⣿⣿⡿⠟⠄⠄⠄⠄⠄⠸⣿⣿⣿⣿⣿⣿⣿⣷⣄⣀⠄⠄⠄
//⠸⠿⠿⠿⠿⠿⠿⠟⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠉⠉⠛⠿⢿⡿⠿⠿⠿⠃⠄⠄
//?.
"#
        }

        // THIICCCCCCCC!!!!
        8 => {
            r#"
//.?            
//⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢟⣻⣯⣽⣟⠿⠟⠛⠛⠻⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⡿⣫⣾⣿⣿⣿⡹⢁⣴⣷⣿⣿⣷⣆⣤⡀⠠⢬⣉⠻⣿⣿⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⢱⣿⣿⣿⠿⠩⢠⣿⣿⣿⣿⣿⣏⣿⢸⣿⡖⣄⠹⣷⡌⢿⣿⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⡸⣿⣿⣿⣼⠂⢸⣿⣿⣾⠯⢟⠋⠿⣿⡿⣳⡿⣳⣿⣷⡈⢿⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⣷⣌⡻⣿⢡⣿⡀⢻⣿⣿⣷⣭⣯⡆⢰⣤⣥⣶⣿⣿⣿⣷⠈⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢃⣿⣿⣿⣤⡈⠛⠛⠙⠛⠁⣸⣿⣿⣿⣿⣿⡿⠉⣤⣤⡉⢻⣿⣿
//⣿⣿⣿⣿⣿⠿⠛⢋⣉⣁⣈⣉⣀⣈⣉⣉⣉⣁⡐⠚⣿⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⡀⢿⣿
//⣿⣿⣿⠏⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠈⢿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⡇⢸⣿
//⣿⣿⣿⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⢸⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⡇⢸⣿
//⣿⣿⡟⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⠇⣸⣿
//⣿⣿⣧⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⣸⣿⣿⣿⣿⣿⣧⡈⠛⠿⠋⢠⣿⣿
//⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⢰⣾⣿⣿⣿
//⣿⣿⣿⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿
//⣿⣿⣿⣆⠘⠿⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠋⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿
//⣿⣿⣿⠋⣠⣶⣶⣶⣶⣶⣶⠂⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿
//⣿⣿⠃⣰⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⣿⣿⣿⣿
//⣿⡏⠀⣿⣿⣿⣿⣿⣿⣿⡇⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿
//⣿⣇⠀⣿⣿⣿⣿⣿⣿⣿⣧⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣸⣿⣿⣿
//⣿⣿⡀⢻⣿⣿⣿⣿⣿⣿⣿⣆⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⢸⣿⣿⣿
//⣿⣿⣷⣄⠙⠿⣿⣿⣿⣿⣿⣿⣷⣄⠈⠛⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⣸⣿⣿⣿
//⣿⣿⣿⣿⣷⡄⢀⣉⣩⣿⣿⣿⠋⢠⣄⣀⣀⣀⣀⡀⢶⣿⣿⣿⣿⣿⣿⣿⣿⡏⢁⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣧⠈⣿⣿⣿⣿⣿⠀⣼⣿⣿⣿⣿⣿⣷⡈⠻⣿⣿⣿⣿⣿⣿⡿⢁⣼⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣆⠘⣿⣿⡿⠃⣰⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠉⠛⠻⠿⠏⣠⣾⣿⣿⣿⣿⣿
//⣿⣿⣿⣿⣿⣿⣿⣦⣈⠉⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿
//?.
"#
        }

        9 => {
            r#"
//.?
//⬜⬜⬜🟥🟥🟥🟥🟥🟥⬜⬜⬜
//⬜⬜🟥🟥🟥🟥🟥🟥🟥🟥⬜⬜
//⬜🟥🟥🟦🟦🟦🟦🟦🟦🟥🟥⬜
//⬜🟥🟦🟦🟦🟦🟦🟦🟦🟦🟥⬜
//⬜🟥🟥🟦🟦🟦🟦🟦🟦🟥🟥⬜
//⬜🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//⬜🟥🟥🟥⬜⬜⬜⬜🟥🟥🟥⬜
//?.
"#
        }

        10 => {
            r#"
//.?
//➖➖🟥🟥🟥
//➖🟥🟥🟥🟥🟥
//🟥🟥🟥🟦🟦🟦
//🟥🟥🟥🟦🟦🟦
//🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥
//➖🟥🟥🟥🟥🟥
//➖🟥🟥➖🟥🟥
//➖🟥🟥➖🟥🟥
//?.
"#
        }

        11 => {
            r#"
//.?⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⣿⣿⣷⣶⣄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣾⣿⣿⡿⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⡟⠁⣰⣿⣿⣿⡿⠿⠻⠿⣿⣿⣿⣿⣧⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⠏⠀⣴⣿⣿⣿⠉⠀⠀⠀⠀⠀⠈⢻⣿⣿⣇⠀⠀⠀
//⠀⠀⠀⠀⢀⣠⣼⣿⣿⡏⠀⢠⣿⣿⣿⠇⠀⠀⠀     ⣿⣿⣿⡀⠀⠀
//⠀⠀⠀⣰⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⡀⠀⠀⠀     ⣿⣿⣿⡇⠀⠀
//⠀⠀⢰⣿⣿⡿⣿⣿⣿⡇⠀⠘⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⢀⣸⣿⣿⣿⠁⠀⠀
//⠀⠀⣿⣿⣿⠁⣿⣿⣿⡇⠀⠀⠻⣿⣿⣿⣷⣶⣶⣶⣶⣶⣿⣿⣿⣿⠃⠀⠀⠀
//⠀⢰⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀
//⠀⢸⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠉⢉⣿⣿⠀⠀⠀⠀⠀⠀
//⠀⢸⣿⣿⣇⠀⣿⣿⣿⠀⠀⠀⠀⠀⢀⣤⣤⣤⡀⠀⠀⢸⣿⣿⣿⣷⣦⠀⠀⠀
//⠀⠀⢻⣿⣿⣶⣿⣿⣿⠀⠀⠀⠀⠀⠈⠻⣿⣿⣿⣦⡀⠀⠉⠉⠻⣿⣿⡇⠀⠀
//⠀⠀⠀⠛⠿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠈⠹⣿⣿⣇⣀⠀⣠⣾⣿⣿⡇⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣦⣤⣤⣤⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠈⢻⣿⣿⡄⠀⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⣸⣿⡏⠀⠀⠀⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠁⠀⠀⢰⣿⣿⣯⠁⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣷⡄⠀ 
//⠀⠀⣀⣤⣴⣶⣶⣿⡟⠀⠀⠀⢸⣿⣿⣿⣆⠀⠀⠀⠀     ⠀⣿⣷⠀ 
//⠀⢰⣿⡟⠋⠉⣹⣿⡇⠀⠀⠀⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠀ 
//⠀⢸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀ 
//⠀⣸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠀⠀ 
//⠀⣿⣿⠁⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣧⠀⠀ 
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀ 
//⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀ 
//⠀⢿⣿⡆⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀ 
//⠀⠸⣿⣧⡀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀ 
//⠀⠀⠛⢿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⣰⣿⣿⣷⣶⣶⣶⣶⠶⠀⢠⣿⣿⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⣽⣿⡏⠁⠀⠀⢸⣿⡇⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⢹⣿⡆⠀⠀⠀⣸⣿⠇⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠀⠈⠻⣿⣿⣿⣿⡿⠏⠀⠀⠀⠀ 
//⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⠿⠿⠿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//?.
"#
        }

        12 => {
            r#"
//.?
//⬜️⬜️🟥🟥🟥⬜️⬜️
//⬜️🟥🟥🟥🟥🟥⬜️
//🟥🟥⬛️⬛️⬛️🟥⬜️
//🟥🟥🟥🟥🟥🟥⬜️
//🟥🟥🟥🟥🟥🟥⬜️
//⬜️🟥🟥⬜️🟥🟥⬜️
//⬜️🟥🟥⬜️🟥🟥⬜️      
//?.
"#
        }

        13 => {
            r#"
//.?
//⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬛⬛⬛⬜⬜
//⬜⬜⬛🟫🟫🟫⬛⬜⬜⬜⬛🟫🟫🟫⬛⬜
//⬜⬜⬛🟫🟧🟧⬛⬜⬜⬜⬛🟧🟧🟫⬛⬜
//⬜⬜⬛🟫🟧⬛⬛⬛⬛⬛⬛⬛🟧🟫⬛⬜
//⬜⬜⬜⬛⬛🟫🟫🟫🟫🟫🟫🟫⬛⬛⬜⬜
//⬜⬜⬜⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬜⬜⬜⬛🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬜⬜
//⬜⬜⬜⬛🟫🟫🟫⬛🟦🟦⬜⬜⬜⬜⬛⬜
//⬜⬛⬛⬛🟫🟫⬛🟦🟦🟦⬜⬜⬜⬜🟦⬛
//⬛🟫🟫⬛🟫🟫⬛🟦🟦🟦🟦🟦🟦🟦🟦⬛
//⬛🟫🟫⬛🟫🟫⬛🟦🟦🟦🟦🟦🟦🟦🟦⬛
//⬛🟫🟫⬛🟫🟫🟫⬛🟦🟦🟦🟦🟦🟦⬛⬜
//⬛🟫🟫⬛🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬛🟫🟫⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬜⬜
//⬜⬛⬛⬛🟫🟫🟫⬛⬛⬛⬛🟫🟫⬛⬜⬜
//⬜⬜⬜⬛🟫🟫🟫⬛⬜⬛🟫🟫🟫⬛⬜⬜
//⬜⬜⬜⬛🟫🟫🟫⬛⬜⬛🟫🟫🟫⬛⬜⬜
//⬜⬜⬜⬜⬛⬛⬛⬜⬜⬜⬛⬛⬛⬜⬜⬜      
//?.
"#
        }

        14 => {
            r#"
//.?
//⬜⬜⬜⬜🟥🟥🟥⬜⬜⬜⬜   🟥🟥🟥                                     🟥🟥🟥
//⬜⬜⬜🟥🟥🟥🟥🟥⬜⬜⬜ 🟥                                         🟥
//⬜⬜🟥🟥🟥🟥⬛⬛⬛⬜⬜ 🟥                  🟥          🟥         🟥
//🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬜ 🟥                  🟥          🟥         🟥
//🟥🟥🟥🟥🟥🟥⬛⬛⬛⬜⬜   🟥🟥🟥            🟥          🟥           🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥⬜⬜         🟥          🟥          🟥                 🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥⬜⬜         🟥          🟥          🟥                 🟥
//⬜⬜🟥🟥🟥🟥🟥🟥🟥⬜⬜         🟥            🟥      🟥                   🟥
//⬜⬜🟥🟥🟥🟥🟥🟥🟥⬜⬜   🟥🟥🟥                🟥🟥🟥               🟥🟥🟥
//⬜⬜🟥🟥🟥⬜🟥🟥🟥⬜⬜
//⬜⬜🟥🟥🟥⬜🟥🟥🟥⬜⬜
//?.
"#
        }

        15 => {
            r#"
//.?
//⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
//⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬛⬜⬜⬜⬛⬜⬛⬜⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬜⬜⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬛⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬜⬛
//⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
//⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
//⬛⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
//⬛⬜⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
//⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
//⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜       
//?.
"#
        }

        16 => {
            r#"
//.?
//⚪⚪⚪⚪🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴🔴🔴🔴🔴🔵🔵🔵🔵🔵⚪⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔵🔵🔵🔵⚪⚪🔵⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔵🔵🔵🔵🔵🔵🔵⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴🔵🔵🔵🔵🔵⚪⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴🔴🔴🔴🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴⚪⚪⚪🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴⚪⚪⚪🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪
//⚪⚪⚪🔴🔴🔴⚪⚪⚪🔴🔴🔴⚪⚪⚪⚪⚪⚪⚪      
//?.
"#
        }

        17 => {
            r#"
//.?
//                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣶⣦⣤⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⢿⣟⡿⣿⢿⡿⣿⢿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣻⣽⡾⣽⣯⢿⣾⣻⣽⣯⢿⣽⣯⢿⣳⣟⣾⡽⣯⣟⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣟⡷⣯⣷⢯⣟⡿⣞⣿⣳⣯⣷⣻⣟⣾⣽⣻⣟⣾⣽⣻⢷⣻⣽⡷⣯⢿⣽⢿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣟⡷⣯⢿⣻⢾⣟⣯⢿⣻⢷⣯⡷⣯⣷⢿⣽⢾⡷⣯⣟⣾⢯⡿⣯⡷⣟⣿⣻⢾⣯⡷⣿⣻⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⣿⣿⣻⣞⣿⡽⣟⣯⣿⢾⣯⢿⣯⢿⡾⣽⣟⡾⣟⣾⢿⣽⣻⢾⣯⣟⣿⣳⣿⣻⢷⣻⣯⡷⣟⣯⣷⣟⡿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣟⡾⣷⣻⣾⣻⢯⣟⣾⣟⣾⣟⣾⢿⣽⣟⡾⣟⣯⣟⡿⣞⣿⣻⢾⣽⣾⣻⢾⣽⣻⣟⣾⡽⣿⡽⣾⣽⣻⣽⢿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣟⡾⣿⡽⣷⢯⣟⡿⣽⣾⣻⢾⣽⡾⣟⣾⣽⣻⢯⣟⣾⣟⣯⣷⢿⣯⣷⣯⣟⣯⣟⣷⣯⣷⣿⣯⣟⣷⣯⣟⣾⣿⡽⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣟⡾⣟⣷⢿⣯⣟⣯⣿⣻⢾⣽⣯⡷⣟⣿⣽⣞⣿⣯⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣟⡾⣟⣯⣟⡿⣾⣽⣳⣯⣟⡿⣞⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄⡀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣟⡾⣟⣯⢿⣞⣿⣳⣯⣟⣾⣽⣻⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠛⠛⠋⠉⠉⠉⠉⠉⠉⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠙⠛⠿⠿⣿⣿⣿⣿⣿⣿⣶⣄⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⡿⣞⣿⢯⣟⣯⡿⣾⣽⣳⣯⣟⣾⣿⣿⣿⣿⣿⣿⣏⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠄⠒⠀⠀⠀⠀⠀⠀⠀⠀⠀⠒⠠⠄⢀⡀⠀⠈⠙⠻⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣻⢯⣟⣯⣿⣳⡿⣽⡾⣯⡷⣿⣿⣿⣿⣿⣿⣛⢶⣻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠠⡀⠀⠈⠙⣿⣿⣿⣿⣿⡄⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣯⣟⡿⣽⣳⣯⡷⣿⢯⣟⣷⢿⣿⣿⣿⣿⣿⡳⣝⡮⡽⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠈⢻⣿⣿⣿⣿⡀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⢾⣽⣻⢯⣟⣾⣽⣟⣯⡿⣾⢿⣿⣿⣿⣿⡿⣵⢫⣞⡵⣻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠢⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⠀⠀⠈⣿⣿⣿⣿⣧⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣸⣿⣿⣿⣿⣿⣯⢿⣳⣿⣻⡽⣷⣻⡾⣽⣻⣽⣿⣿⣿⣿⣿⡿⣜⣳⢎⡷⣝⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠐⠠⠄⢀⡀⠀⠀⠀⠀⠀⠀⣀⠀⠤⠊⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⡄
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣠⣤⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢾⣻⣟⣾⣽⣻⣽⢷⣟⡿⣽⣞⣿⣿⣿⣿⣿⡿⣜⢧⡻⣜⡳⣞⢷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⡇
//⠀⠀⠀⠀⠀⢀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣟⡿⣾⣽⣞⣯⣟⡿⣾⣻⣽⡾⣿⣿⣿⣿⣿⣿⡹⢮⣝⢧⡟⣼⢫⡽⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣶⣿⣿⣿⣿⣿
//⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⢿⣿⣿⣿⣿⣿⣿⣳⡿⣽⣷⣻⢾⣻⢾⣻⡷⣟⡷⣿⣽⣿⣿⣿⣿⣿⣝⡳⢮⣳⠽⣎⠿⣼⡹⢷⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⢶⡞⣯⢳⢧⣿⣿⣿⣿⣿
//⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⡿⣟⡿⣽⣯⢷⣯⢿⣽⣻⣿⣿⣿⣿⣿⣟⡷⣿⣻⢾⣽⣟⣯⡿⣷⣟⡿⣽⡷⣯⣿⣿⣿⣿⣿⣾⣹⢳⣭⣛⢾⡹⣖⢯⣳⢎⡟⣿⢲⡶⢶⢶⣤⣤⣤⡤⣤⣄⡤⣤⢤⡤⣤⢤⡖⣶⢲⡖⣿⢻⡽⣹⢎⡷⣹⢞⡽⢮⣿⣿⣿⣿⡏
//⠀⠀⠀⣼⣿⣿⣿⣿⣿⢯⣟⡿⣽⣟⡾⣿⡽⣯⡿⣽⣿⣿⣿⣿⣿⣯⢿⣷⣻⣯⣷⢯⡿⣽⡷⣯⢿⣯⣟⣷⢿⣿⣿⣿⣿⣿⣮⢗⡮⣝⡮⢷⣹⢎⡷⣫⢾⡱⣏⠾⣝⠾⣜⡶⣣⢟⡶⣹⢞⡵⣫⢞⡵⣫⢾⡱⣏⡾⣱⣏⠾⣵⢻⡜⣯⢞⡽⣻⣿⣿⣿⣿⠇
//⠀⠀⢰⣿⣿⣿⣿⣿⣯⢿⣯⣟⡿⣞⣿⣳⡿⣯⣟⣿⣿⣿⣿⣿⣿⢯⣿⣞⣷⣟⡾⣿⡽⣷⣟⣿⣻⢾⣽⣾⣻⢿⣿⣿⣿⣿⣿⣿⣼⢣⡟⣧⣛⢮⢷⡹⣎⢷⣫⢟⣮⢻⡵⣫⢗⡯⣞⡵⣫⡞⣵⣛⢾⡱⣏⢷⡹⣞⡵⣚⠿⣜⣧⢻⡜⣯⣾⣿⣿⣿⣿⡟⠀
//⠀⠀⣾⣿⣿⣿⣿⡿⣽⣻⡾⣽⣻⣯⡷⣿⡽⣷⣻⢿⣿⣿⣿⣿⣿⢯⣷⢿⡾⣽⣻⢷⣟⡿⣞⣷⢿⣯⡷⣯⣟⣯⣿⣿⣿⣿⣿⣿⣿⣿⣾⣵⣫⡞⣧⢻⡝⣮⢳⣏⡞⣧⢻⡵⣫⢞⡵⣫⢷⡹⢮⣝⡮⣽⢺⣭⢳⡝⡾⣭⣛⠾⣜⡧⣟⣿⣿⣿⣿⣿⡟⠀⠀
//⠀⢰⣿⣿⣿⣿⣿⣟⣯⣷⢿⣻⢷⣯⢿⣳⡿⣯⣟⣿⣿⣿⣿⣿⡿⣯⣟⣯⣿⣻⡽⣿⣞⡿⣯⣟⡿⣾⡽⣟⣾⣻⢾⡽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣼⣷⣮⣝⣞⣧⡻⣵⢫⢾⡱⢯⣝⡳⢮⣳⢭⡗⣮⢻⣼⣳⣧⣯⣿⣾⣿⣿⣿⣿⣿⣿⠋⠀⠀⠀
//⠀⣸⣿⣿⣿⣿⣿⣞⡿⣾⣻⢯⣿⢾⣟⣯⢿⣷⣻⣿⣿⣿⣿⣿⣿⣽⢾⣻⡾⣽⣻⢷⣻⣽⣟⡾⣟⣷⢿⣻⢷⣻⣯⢿⣳⣯⢿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀
//⠀⣿⣿⣿⣿⣿⣟⣾⣟⡷⣿⣻⡽⣟⣾⢯⣿⣞⡿⣿⣿⣿⣿⣿⡷⣯⡿⣯⣟⡿⣽⣻⢯⣷⣻⣽⢿⣾⣻⢯⣿⣻⢾⣟⣯⣟⣯⡿⣯⣟⡿⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀
//⢰⣿⣿⣿⣿⣿⣟⣾⣽⣻⢷⣟⡿⣯⣟⣯⣷⣟⣿⣿⣿⣿⣿⣿⣟⡷⣿⡽⣯⣟⡿⣽⣻⢷⣻⡽⣟⣾⡽⣟⣷⣻⣟⣾⣻⡾⣽⣻⢷⣻⣽⢯⣿⢾⡽⣯⣟⣿⣻⣟⡿⣿⢿⡿⣿⢿⡿⣿⣿⣿⣿⣿⣿⣿⣿⢿⡿⣿⢿⣿⣻⢿⡽⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⣻⣞⣷⢿⣻⡾⣿⣽⢾⣻⡾⣽⣾⣿⣿⣿⣿⣿⣯⢿⡷⣟⣯⡿⣽⢿⣽⣻⢯⣟⡿⣽⣻⢯⣟⣷⢯⣟⡷⣿⣻⡽⣟⣯⣟⣯⣟⣯⣿⣽⢾⣳⡿⣞⣿⡽⣯⣿⣽⣻⣽⣷⣻⣞⣷⣻⣞⣷⢯⣟⣿⡽⣟⣾⡽⣟⣿⣿⣿⣿⣿⡗⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⣳⣯⣟⣯⡿⣽⡷⣯⡿⣯⣟⡿⣾⣿⣿⣿⣿⣿⣯⢿⣽⣟⡷⣿⣻⣟⣾⣻⢯⣟⣿⡽⣯⣿⣻⢾⣟⣯⢿⣷⣻⣽⢿⣽⢾⣻⣾⣻⢾⣽⣻⢯⣟⣯⣷⢿⣻⣞⡷⣟⣷⢯⣷⢿⣽⣳⡿⣾⣻⣟⣾⣻⢯⡿⣽⣟⣾⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⣳⢿⣞⣯⡿⣷⣟⣯⢿⣷⣻⣟⣿⣿⣿⣿⣿⣿⡽⣿⢾⣽⣻⢷⣯⢿⣞⡿⣯⢿⣞⣿⣳⣯⣟⡿⣞⣯⣿⢾⡽⣯⡿⣾⣻⢷⣯⣟⣯⣷⢿⣻⡽⣷⢯⣿⣻⢾⣟⡿⣾⣻⡽⣟⣾⢯⣟⣷⢿⡾⣽⢯⣿⣻⣽⡾⣯⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⡽⣯⣟⡷⣿⣳⣯⢿⣻⡾⣷⢯⣿⣿⣿⣿⣿⣿⡿⣽⣯⡷⣿⢯⣟⣯⡿⣽⣻⣯⢿⡾⣽⣳⡿⣽⢿⣽⢾⣻⣟⡷⣿⡽⣯⣿⢾⣽⣳⣯⢿⣯⣟⣯⣿⣳⢿⣻⡾⣟⣷⢿⣽⢿⣽⣻⣽⡾⣿⣽⣻⣯⡷⣟⡷⣿⣽⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⡽⣟⣾⢿⡽⣷⣻⢿⣽⣻⣽⣟⣿⣿⣿⣿⣿⣿⣟⣷⢯⣿⣽⣻⢯⣷⢿⣻⢷⣻⣯⣟⣿⣳⢿⣯⢿⣾⣻⣟⡾⣟⣷⢿⣻⢾⣯⣟⣷⣻⣟⣾⣽⣳⣯⣟⣿⣳⡿⣯⣟⡿⣾⢯⣷⣟⡷⣿⣳⣯⡷⣿⣽⣻⣽⡷⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⣽⣻⣽⣯⢿⣯⣟⣯⣷⣟⡷⣯⣿⣿⣿⣿⣿⣿⣻⢾⣟⡷⣯⣟⡿⣽⣻⢯⣿⣻⢾⣽⣾⣻⣟⣾⣟⡷⣟⣾⣟⣯⣟⣯⣿⣻⢾⣽⣾⣻⢾⣽⡾⣯⡷⣿⢾⣽⣻⢷⣻⣽⣯⢿⣳⣯⢿⣷⣻⢷⣻⡷⣯⣟⣾⣽⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⢾⣽⣳⣯⣿⢾⣽⣳⡿⣞⣿⣽⣻⣿⣿⣿⣿⣿⣟⡿⣾⣻⣽⢯⣿⣻⣽⣟⣷⣻⣯⡷⣯⣷⣻⢷⣯⢿⣻⢷⣯⣟⣾⣻⢾⡽⣟⣷⢯⣟⣯⣷⢿⣳⡿⣯⣟⡷⣿⣻⢯⣷⣻⢿⣽⡾⣟⣾⢯⡿⣯⣟⣷⢿⣽⡾⣽⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⢯⣟⡷⣟⣾⢿⣽⣳⣿⣻⢷⣯⣿⣿⣿⣿⣿⣿⣯⣟⣷⢿⣽⣻⢷⣯⡷⣿⣞⡿⣾⣽⢷⣻⣽⣟⣾⣟⣯⡿⣾⣽⣳⡿⣯⢿⣻⡾⣿⣽⣻⢾⣻⣽⣻⢷⣻⣟⡷⣿⢯⣷⢿⣻⣾⡽⣿⣽⣻⣽⣷⣻⡾⣟⣾⣽⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀
//⢸⣿⣿⣿⣿⣿⣯⢿⣽⢿⣽⣻⡾⣟⣾⡽⣿⢾⣽⣿⣿⣿⣿⣿⣷⣻⡾⣿⡽⣯⣿⣞⣿⣳⡿⣽⡷⣯⣿⣻⢾⣽⡾⣽⣾⣻⢷⣯⢿⣽⣻⢿⣽⣻⢷⣯⣟⡿⣽⣳⣿⣻⣽⡾⣿⡽⣟⣾⢿⣽⣞⣿⣳⣯⣟⣾⣳⡿⣽⣻⣽⡿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
//⠸⣿⣿⣿⣿⣿⡾⣿⣽⣻⡾⣯⣟⣯⣷⢿⣯⣟⣿⣿⣿⣿⣿⣿⣷⣻⣽⢷⣟⡿⣾⣽⢾⣯⢿⣽⣻⢷⣯⣟⣯⣷⢿⣻⡾⣽⣟⣾⣟⡷⣿⢯⣷⣟⡿⣾⣽⣻⢯⣟⣾⣽⣳⣿⣳⢿⣻⡽⣟⣾⣽⢾⣻⡾⣽⣳⡿⣽⣻⣽⡿⣽⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
//⠀⣿⣿⣿⣿⣿⣟⣷⢯⣷⢿⣯⣟⡷⣯⣿⢾⣽⣾⣿⣿⣿⣿⣿⡷⣯⣟⡿⣾⣻⢷⣯⡿⣾⣻⣽⢯⣿⢾⣽⣳⣯⣿⣳⣿⣻⢾⣳⣯⢿⣯⢿⡾⣽⣻⢷⣯⣟⡿⣽⣳⣯⣷⢯⣟⡿⣽⣻⢯⣟⣾⣟⡷⣿⣻⣽⣻⣽⣿⢯⣟⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀
//⠀⣿⣿⣿⣿⣿⣟⣾⢿⣽⣻⡾⣽⣻⣽⡾⣟⣷⢯⣿⣿⣿⣿⣿⡿⣽⡾⣟⣷⢿⣯⡷⣟⣯⡿⣾⣻⣽⣯⣟⡷⣟⣾⣽⣞⣯⡿⣯⣟⡿⣾⣻⣽⢿⣽⣻⡾⣽⣻⢯⣟⣾⣽⣻⡽⣟⣯⢿⣻⣽⡾⣯⣟⣷⣟⣷⣿⣻⡾⣟⣯⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀
//⠀⢸⣿⣿⣿⣿⣿⣽⣻⡾⣯⣟⡿⣽⣳⣿⣻⢾⡿⣿⣿⣿⣿⣿⡿⣽⣻⣽⢯⣿⢾⣽⢿⣽⣻⢷⣟⣷⣻⢾⣟⡿⣽⣞⣯⣷⢿⣽⣾⣻⢷⡿⣽⣻⣞⣯⢿⣻⣽⣟⣯⣷⣻⡽⣟⣯⣿⣻⡽⣷⣻⣽⡾⣿⢾⣯⡷⣿⣽⣻⢷⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀
//⠀⢸⣿⣿⣿⣿⣿⣞⣷⢿⣯⢿⣽⢿⣽⣞⡿⣯⢿⣿⣿⣿⣿⣿⣿⣻⣽⡾⣿⣽⣻⡾⣟⣷⢿⣻⡾⣯⣟⡿⣾⣻⢯⣟⣷⣯⢿⣳⣯⣟⣯⡿⣯⣷⢿⣽⣻⣟⣾⣽⢾⣳⡿⣽⣟⡷⣯⣷⢿⣯⣟⡷⣟⣯⣿⢾⣽⡷⣯⣟⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⣿⣿⣿⣿⣿⣟⣾⣟⣾⢿⣽⣻⣾⣽⣻⣽⣟⣿⣿⣿⣿⣿⣿⣳⣯⢿⣳⣯⣷⢿⣻⡾⣿⣽⣻⢷⣻⣟⣷⢿⣻⣽⡾⣽⣻⣟⣾⡽⣷⢿⣽⣾⣻⡽⣷⢯⣷⣟⡿⣽⣻⣽⡾⣿⣽⢾⣟⣾⣽⣻⢯⣟⣾⣟⡷⣟⣯⡿⣾⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢹⣿⣿⣿⣿⣿⢾⣽⡾⣟⣷⣟⡾⣷⣻⢷⣯⣿⣿⣿⣿⣿⣿⢷⣻⡿⣽⣳⣯⡿⣯⣟⣷⢯⣟⣿⣽⢾⣯⣟⣯⣷⢿⣻⢷⣯⢿⣽⢯⣿⣞⡷⣿⣽⣻⣯⣷⣻⢿⣽⢯⣷⢿⣳⣯⣿⢾⣻⣞⡿⣯⣟⣷⢯⣿⣻⢷⣟⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢸⣿⣿⣿⣿⣿⣟⣾⣽⢿⡾⣽⣻⣽⢯⣿⣞⣿⣿⣿⣿⣿⣿⣟⣯⣿⣽⣻⣾⡽⣷⣻⣽⣟⡿⣾⡽⣟⣾⡽⣷⢯⣿⢯⣿⢾⡿⣽⣻⢷⣯⢿⡷⣯⣷⣟⣾⢯⣿⢾⡿⣽⣻⢯⣷⢯⣿⢯⡿⣽⣟⣾⢯⣿⣳⣿⣻⢾⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⢻⣿⣿⣿⣿⣿⣾⣯⣿⡽⣿⡽⣯⣿⣳⢿⣾⣿⣿⣿⣿⣿⣟⣾⣳⣯⣷⢯⣿⣽⣻⡾⣽⣻⢷⣟⡿⣽⣻⣽⢿⣽⣻⡾⣿⣽⣻⣽⢿⣾⣻⣽⣷⣻⢾⣯⡿⣽⣯⣟⣯⣿⣻⡽⣿⣽⣻⣽⣟⣾⢯⣿⣳⢿⡾⣽⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣷⣟⡷⣯⣿⣳⣯⣷⢿⣻⣽⢿⣞⣿⢯⣟⣾⣟⡷⣿⡽⣷⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣾⣿⣷⣿⣾⣿⣾⣿⣿⣿⣿⣿⣿⣟⣾⣟⡷⣿⢯⣟⡿⣾⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠈⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢾⣽⣻⣽⡾⣯⣷⢯⣿⣻⢾⣯⢿⣾⣻⢯⣷⣻⣽⡷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣻⢷⣯⣷⣻⣽⣯⢿⣯⣟⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠙⠋⠋⠉⠉⠉⠉⢹⣿⣿⣿⣿⣿⣯⡷⣟⣷⢿⣳⣯⣿⣳⡿⣯⣟⡿⣞⣯⣿⣽⣻⡾⣽⣿⣿⣿⣿⣿⡿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣻⢿⣽⣳⣯⣟⡿⣞⣷⣟⡷⣯⣿⢾⣽⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣷⣻⢿⣽⣻⢯⣷⢯⣷⣟⡿⣞⣿⣻⣽⢾⣳⡿⣽⣟⣾⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡾⣽⣯⡷⣟⣷⣻⣽⣟⡷⣯⣿⣻⢾⣻⣽⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣷⣟⣯⣷⢿⣻⡽⣟⣷⣯⢿⣻⢷⣻⣽⣟⣯⢿⣻⡾⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⡿⣽⣞⡿⣯⢿⣽⣞⣯⣿⣽⢾⣻⢿⡽⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣞⡿⣞⡿⣯⣟⣿⢾⣽⣻⢯⣿⢯⣷⢯⣟⣿⣳⡿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⡿⣽⡾⣿⡽⣿⣞⣯⣷⣟⡾⣟⣯⣿⣻⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣯⣟⡿⣽⣯⣟⣾⢿⡽⣯⡿⣯⢿⡾⣟⡿⣾⣽⣻⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⢯⣟⣷⢿⣳⣯⣟⣾⣽⣻⢯⣟⣾⣽⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣷⣻⢿⣽⣞⣯⣟⣯⡿⣷⣟⡿⣯⣟⡿⣽⡷⣯⢿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⢯⣿⢾⣻⣯⢷⡿⣽⣾⣻⢯⣿⣽⣾⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡽⣟⣾⢯⣟⣾⢯⣟⣷⣻⣽⣟⣾⣟⣯⣿⡽⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡿⣽⣻⢷⣻⣯⢿⡷⣯⣟⣿⣳⣯⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣟⣯⣿⢯⣿⣽⣻⣟⣾⢯⣷⣻⢷⣯⣷⣯⣿⣷⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣽⣯⣿⣻⣾⣿⣽⣟⣾⣽⣷⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠸⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠿⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠛⠛⠛⠛⠛⠛⠛⠛⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//?.
"#
        }

        18 => {
            r#"
//.?
//        ⣠⣴⠿⠶⠶⣦⣴⡿⠿⢷⣶⠾⠶⣦⣴⡾⠶⠶⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢰⣿⠁⠀⣦⠀⠈⣿⡇⠀⠀⡇⠀⠀⣿⠁⠀⣴⡄⠀⢻⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠸⣿⠀⠀⠻⢶⣶⣿⡇⠀⠀⡇⠀⠀⣿⡀⠀⠙⠷⣶⡾⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⣀⣴⣾⣿⣷⣤⡀⠀⠙⣿⡇⠀⠀⡇⠀⠀⣿⣿⣦⣀⠀⠈⢻⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⢀⣾⠟⠁⢸⣿⠀⠀⣿⠀⠀⢸⡇⠀⢸⡇⠀⠀⣿⠀⠀⣽⡇⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢠⣿⠃⠀⠀⠘⢿⣦⣀⡀⣀⣠⣿⣷⣄⣀⢀⣀⣴⣿⣧⣀⣈⣀⣠⣾⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⢠⣿⠃⠀⠀⠀⠀⣀⣹⣿⣿⣿⣿⣥⣬⣿⣿⣿⣿⣉⡀⠈⠙⠛⠛⣿⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⣾⡇⠀⣠⣶⡾⠿⠛⠋⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠛⠻⣷⣄⠀⠀⢻⡇⢀⣴⣶⣶⣄⣠⣴⣶⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⢠⣿⠀⣰⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣷⡄⢸⣷⣾⡟⠁⠉⠛⠛⠉⠉⠹⣿⣷⣶⣦⣄⠀⠀⠀⠀⠀
//⢸⡿⢠⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⢸⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠈⠛⠉⠙⣿⣷⣿⣷⡄⠀
//⢸⡇⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⢸⣿⣿⠀⠀⠀⢰⣿⠄⠀⢰⣶⠀⠀⢀⣀⠀⠀⢻⣿⠀
//⢸⡇⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡟⢸⣿⡿⠶⣦⣄⣾⣿⡄⠀⣸⣿⠀⠀⢸⣿⡄⠀⢸⣿⡇
//⢸⡇⠀⢿⣷⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⠁⢸⣿⣧⣄⠀⢉⣻⣿⡇⠀⣽⣿⠀⠀⢸⣿⡇⠀⢸⣿⠇
//⣿⡇⠀⢸⣿⢻⣿⠿⠿⠿⣿⣶⣶⣶⣦⣤⣤⣤⣤⣴⣶⣶⡿⣿⡇⠀⠀⣿⣿⣿⣿⣿⣿⣿⣃⣀⣿⣿⠀⠀⣼⣿⡇⣀⣾⡿⠀
//⣿⡇⠀⠀⠻⣦⣨⡀⠒⠚⠹⠿⠷⠚⠙⠋⠉⠿⠹⠿⠶⢛⣶⡟⠀⢀⣾⣿⢿⣿⠙⣿⡿⠿⠿⢿⣿⡿⠿⠿⠿⠛⠛⠛⠋⠀⠀
//⣿⡇⠀⠀⠀⠀⠉⠉⠛⠒⠒⠦⠄⠤⠤⠤⠤⠤⠶⠖⠚⠉⠁⠀⠀⠀⠻⣿⣿⣿⣲⠏⠀⠀⠀⢀⣹⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡟⠻⣿⣦⣤⣶⡾⠟⠛⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⣿⣧⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⡇⠀⠀⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⣿⡏⠈⠳⢦⡤⣤⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠤⢤⡤⠤⠤⢤⣶⠦⣺⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀      
//?.
"#
        }

        19 => {
            r#"
//.?
//          ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⣤⣤⣤⣄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⡿⠛⠋⠉⠉⠉⠉⠉⡿⠛⠻⠷⣶⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⠟⠉⠀⠙⣦⣀⣀⣀⣠⡤⠴⡿⣄⡀⠀⠀⠉⠻⢷⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⡟⠁⠀⠀⠀⣰⠋⢧⠀⠀⠀⠀⠀⡇⠀⠉⠙⠓⠒⡶⢯⣙⣿⣆⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡟⠀⠀⠀⢀⠞⠁⠀⠈⣳⡤⠤⠴⠚⣟⠛⠒⠒⠒⣺⠳⢤⣀⣉⣻⣷⡀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡿⠶⠶⣤⣾⡁⠀⠀⢀⡜⠉⣧⣠⣤⣴⣾⣶⠶⠶⠶⠷⣶⣶⣶⣬⣭⣙⣷⡀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⠁⠀⠀⣿⠀⠙⢲⣞⠁⣠⡾⠟⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠻⢷⣄⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡏⠀⠀⠀⡇⠀⠀⢸⠏⣿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢷⡄⠀
//⠀⠀⠀⢀⣠⣴⣶⣶⣦⣾⠃⠀⠀⢠⡇⠀⠀⣸⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⡄
//⠀⠀⣠⡿⠿⡄⠀⠀⠈⣿⡀⠀⠀⡼⠀⠀⣴⣃⣤⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇
//⠀⣰⡿⠁⠀⠹⡄⠀⠀⣿⠿⣶⣴⡷⠒⠋⠻⡄⠀⢹⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⠃
//⢀⣿⠁⠀⠀⠀⢹⣀⣴⡏⠀⠀⠙⢿⣦⠀⠀⢹⡄⠀⠻⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣿⠋⠀
//⢸⡿⢧⠀⠀⢀⡼⠯⣼⡇⠀⠀⠀⠀⠙⣷⡄⠀⣇⡤⠞⠉⠻⢷⣤⣄⣀⣀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣤⣶⣿⣿⣿⠀⠀
//⣿⡇⠈⢧⣠⠎⠀⠀⢸⡇⠀⠀⠀⠀⠀⠘⣿⡞⠛⠢⣄⠀⣠⠏⠈⠉⡿⠛⠛⠛⢻⠛⠛⠛⠛⢿⠉⠁⣴⠟⠁⣿⠀⠀
//⣿⠃⠀⡼⠧⣄⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠘⣷⡀⠀⠘⣶⣁⡀⠀⡼⠁⢀⡀⠀⠘⡇⣀⣠⣤⣬⣷⣾⠏⠀⠀⣿⠀⠀
//⣿⣀⡞⠁⠀⠈⢣⡀⢸⡇⠀⠀⠀⠀⠀⠀⠀⢹⣇⠀⡼⠁⠀⠉⣹⠛⠉⠉⡉⠉⢙⣏⠁⠀⠀⠀⣼⡏⠀⠀⠀⣿⠀⠀
//⣿⡿⣄⠀⠀⠀⠀⢳⣼⡇⠀⠀⠀⠀⠀⠀⠀⠈⣿⡾⠁⠀⠀⢀⡇⢠⠂⣜⣠⣤⠸⡟⢣⠀⠀⢰⡿⠀⠀⠀⠀⣿⠀⠀
//⣿⡇⠈⠳⡄⠀⠀⣨⢿⡇⠀⠀⠀⠀⠀⠀⠀⠀⣿⡗⠒⠲⢤⣸⠀⣸⣄⣿⣿⣿⣷⣿⣞⣠⣤⣿⠇⠀⠀⠀⠀⣿⠀⠀
//⢸⣇⠀⠀⢹⡀⡰⠃⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⣹⢉⡽⣿⢿⣿⣿⣿⣿⣅⠀⠀⣿⠀⠀⠀⠀⢀⣿⠀⠀
//⠘⣿⡀⠀⠈⡿⠁⠀⢸⣷⣦⣤⣤⣄⣀⡀⠀⠀⢸⡇⠀⠀⠀⡟⠘⡅⢇⢸⣿⣿⠇⡇⡸⠀⠀⣿⠀⢀⣀⣠⣼⣿⠀⠀
//⠀⢻⣇⠀⣰⠛⠒⠦⣼⡇⠀⠀⠉⠉⠙⢻⣷⣦⣼⡏⠉⠓⠦⣿⠤⠵⠾⠾⠿⢿⣸⣯⠧⠖⠚⣿⡾⠟⠋⠉⣹⡇⠀⠀
//⠀⠈⢿⣶⠇⠀⠀⠀⢸⣿⣶⣤⣤⣤⣀⣼⣀⣈⣙⣃⡀⠀⠀⢹⡀⠀⠀⢀⣀⣀⣸⣁⣀⣀⣤⣤⣤⣶⠶⠿⣿⡇⠀⠀
//⠀⠀⠈⠻⣷⣄⠀⠀⢘⣧⠀⠀⠉⠉⠉⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⢉⡉⠉⠉⠀⠀⠀⢀⣿⠁⠀⠀
//⠀⠀⠀⠀⠈⠙⠛⠿⠻⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡷⠶⠶⠶⢶⡶⠿⠿⠿⠛⠋⠀⠀⢀⣀⣤⣾⡿⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡇⠀⠀⠀⢸⣧⣤⣤⣤⣶⣶⠶⠿⠛⠋⠁⣼⡇⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⡍⠙⠛⢿⠿⠷⠶⠶⠾⠿⠿⠟⢻⡇⠀⠀⠀⢸⡏⠉⠁⠀⣀⣀⣀⣀⣄⣀⣀⣿⠁⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡟⠲⢦⣼⣀⣀⣀⣤⣤⣀⣀⡀⢸⡇⠀⠀⠀⢸⣷⠖⠚⠉⠉⠀⠀⠀⠀⠀⣸⡏⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⠀⠀⡏⠁⠀⠀⠀⠀⠀⠈⠉⣿⠇⠀⠀⠀⠀⢿⣆⠀⠀⠀⠀⠀⠀⠀⣠⡿⠁⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣧⣀⡇⠀⠀⠀⠀⠀⢀⣠⣾⠟⠀⠀⠀⠀⠀⠈⠻⢷⣶⣶⣶⣶⡶⠿⠛⠁⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠛⠿⠿⠿⠿⠿⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀      
//?.
"#
        }

        20 => {
            r#"
//.?
//⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
//🟥🟥🟥🟥🟥⬛⬜⬜⬜⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
//🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬜⬜⬜⬜⬜⬜⬜
//🟥🟥🟥🟥🟥🟥🟥🟥⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬛⬛
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛🟨🟨⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛🟨🟨⬛⬛⬛⬛🟥🟥🟥⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟨⬛🟨⬛🟦🟦🟦🟦⬛🟥🟥🟥⬛⬛🟥🟥🟥🟥⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟨⬛🟨🟨⬛⬛⬛⬛🟥🟥🟥🟥🟥⬛🔪🟥⬛⬛🟧🟧🟧⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟨⬛🟨🟨🟨🟨🟨⬛🟥🟥🟥🟥🟥🔪🔪🔪🟧⬛🟧🟧⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟨⬛🟨🟨🟨🟨🟨⬛🟥🟥🟥🟥🟥🟥🔪⬛🟧⬛🟧⬛🟦🟦🟦⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛🟨🟨🟨🟨🟨⬛🟥🟥🟥🟥🟥⬛🟧🟧⬛🟧🟧🟧⬛🟦🟦⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟨⬛⬛⬛🟨⬛🟥🟥🟥🟥🟥⬛⬛⬛🟧🟧🟧🟧🟧⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛🟥⬛⬛⬛🟥🟥🟥🟥🟥🟥⬛🟧🟧🟧🟧🟧⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛🟧⬛⬛⬛🟧⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
//🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛⬛⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛🟥⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛
//🟥🟥⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬜⬜⬜⬜
//⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛🟥🟥🟥⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜
//⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜      
//?.
"#
        }

        21 => {
            r#"
//.?
//           ⠀⣀⡴⠒⣛⣉⣉⣉⣹⣷⠄⠀⠀⠀⣴⠒⢦⠀⠀⠀⠀⣴⡟⣶⠀⠀⠀⢀⣤⠖⣞⣋⣉⣉⣉⣿⡶⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡿⠀⣟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢸⠀⠀⠀⠀⣿⡇⢻⠀⠀⠀⣿⡃⢾⡉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⣈⡛⠒⠦⠤⣄⣀⠀⠀⠀⠀⣿⠀⣼⠀⠀⠀⠀⢹⡇⢸⠀⠀⠀⠉⠳⣄⣙⠓⠲⠤⢤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠒⢤⠉⢧⠀⠀⠀⣿⠀⢷⠀⠀⠀⠀⣸⠇⣸⠀⠀⠀⠀⠀⠀⠀⠉⠉⠓⠢⡄⠙⡄⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡤⢤⣤⣤⣤⣤⣟⣁⡿⠀⠀⠀⠙⢧⡈⠳⠤⠤⠶⣋⣰⠏⠀⠀⠀⢀⣤⠤⣤⣤⣤⣤⠴⢋⣸⠇⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢙⣶⠞⠋⠉⠁⠀⠈⠉⠙⠲⣄⠀⠀⠀⠉⠙⠒⠒⠛⠉⠀⠀⠀⠀⢀⡼⠟⠉⠉⠀⠀⠈⠉⠛⠦⣄⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⡄⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡤⠖⠚⠉⠳⡄⠀⠀⠀⠀⠀⠀⣠⣴⣇⣀⡤⠴⠒⠛⠓⢦⡀⠀⠀⠀⠀⠀⢹⡄⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠃⠀⠀⠀⠀⠀⠀⢀⣠⠖⠋⠁⠀⠀⠀⠀⠀⠸⡄⠀⠀⠀⢀⡾⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⠀⠀⠀⠀⠈⢧⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢀⣠⣿⠀⠀⠀⠀⠀⠀⢠⡾⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀⠀⢻⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢸⢦⣀⠀⠀⠀
//⠀⠀⢀⣴⠆⠀⣰⠋⢀⣷⠀⠀⠀⠀⠀⠀⠘⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠈⢷⡀⠀⣀⣀⣀⠀⠀⠀⠀⢀⣼⠃⠀⠀⠀⠀⠀⠘⡇⠈⢳⡀⠀
//⠀⢠⡟⢸⠀⣰⠇⠀⠈⡏⠀⠀⠀⠀⠀⠀⠀⢻⣀⢀⣀⣀⣀⣀⣤⠤⠤⣾⠋⠀⠀⣄⡀⢠⡏⠉⠁⠀⠉⠉⠉⠓⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢧⠀
//⠀⢸⡇⢸⠀⣿⠀⠀⢰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⠀⠀⠀⣻⠀⠀⠀⢿⣍⠙⣲⣤⠤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢸⠀
//⢀⣼⣇⣸⡀⣿⠀⠀⠸⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡀⠀⠀⠀⠈⣿⡟⢹⡶⢀⠈⠙⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢸⠀
//⣼⣿⣹⡏⢻⣿⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡏⠉⠛⢧⣿⡖⠒⠚⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⢸⠀
//⢻⣟⢒⣆⢸⣿⠀⠀⠀⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠀⠀⠀⢸⠀
//⠻⠿⠿⠟⢻⣄⣀⣀⣼⣄⣄⣄⣄⣄⣄⣄⣀⣀⣀⣀⣤⣀⣀⣀⣀⣀⣠⣿⣤⣀⣀⣾⣧⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣠⣀⣀⣀⣀⣀⣀⣿⣆⣀⣐⣾⠃     
//?.
"#
        }

        // awwwwwwww!! ඞ
        22 => {
            r#"
//.?
//        ⣤⡴⠾⠷⠾⠷⠿⠾⠷⠿⠶⢷⣤⣤⣀⣀⣀⣀⣀⣤⣠⣀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⣀⡤⠖⠒⡛⣉⠩⢁⠢⡑⢂⠲⡐⠢⢆⡱⢌⡜⢠⢃⡜⡩⡉⢍⠩⡉⢍⡩⢻⠛⣛⠳⠶⣄⠀⠀⠀⠄⠀⠀⠀⠀⠀
//⢀⠒⡌⢡⠒⡨⢑⠰⠠⢃⠅⢣⠘⡄⢣⠘⡡⢊⡔⣢⢌⢣⠎⡴⣡⠝⣢⢣⡙⢦⡑⣣⠹⣄⠫⡜⡹⢷⣄⠀⠀⠀⠀⠀⠀⠀
//⢈⠒⢌⠢⠘⡄⠣⢌⠱⡈⠜⡠⢃⠜⣠⢋⡴⢃⢮⡑⣎⢧⡛⣴⣃⡟⣦⢳⣚⡵⣪⣕⣳⡞⣷⡼⣑⠦⣍⢷⣄⠀⠀⠀⠀⠀
//⠤⢉⠢⢌⠱⣀⠣⠌⢢⠑⣌⡑⢎⠼⣠⢳⡸⣍⢶⡹⣜⣶⣻⣵⣯⣻⣵⣯⢯⣷⣝⣮⢷⣿⣽⣻⣧⡛⡬⢎⡽⣆⡀⠀⠀⠀
//⢄⢃⠒⡌⠒⢤⢂⡍⢢⡙⢤⡙⡌⢮⡱⢆⠿⣽⣮⣷⢫⣶⣿⣿⣞⣷⣿⣿⣿⣿⣿⣾⣿⣿⣿⣧⣿⣹⢳⣛⣶⣹⢷⡀⠀⠀
//⠌⢦⠱⣈⠝⡤⢣⠜⣢⡙⡤⡓⠼⣑⢮⡙⢮⡝⣻⢿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⢷⣣⢏⣷⣻⣄⠀
//⣋⢆⠳⣌⠚⣔⢣⠛⣤⡙⡲⢩⢳⣉⠶⣩⠳⣜⡱⢎⡜⣙⢫⡙⡍⢎⡱⣉⢎⡱⢌⠦⡑⢦⠱⡌⢦⢡⢛⣿⣿⣮⢷⣩⣿⡀
//⡍⢮⠱⣌⡹⢤⡋⡽⢰⣍⠳⣍⠳⣜⡸⢣⣛⢬⠳⣭⡜⢦⣣⢼⡸⢥⢣⣍⢶⣩⢎⢧⡙⣎⢳⣙⣎⡳⢎⢶⣿⣟⣯⣾⠟⠁
//⢮⡱⢣⢖⡱⢦⡙⣖⢣⢎⡳⣜⡹⢦⡝⢣⢞⡣⣟⣶⡹⣇⡞⣦⡝⣮⢷⣾⢿⣳⢏⣾⡱⣽⣚⣶⣭⢿⣽⣾⠿⠾⣿⣿⣦⡄
//⢧⣙⢧⣎⠵⣎⡵⢎⡳⣎⠵⣎⡵⣶⢻⡿⣾⢿⣿⢧⡿⣼⡽⣶⢻⣯⣿⣟⣯⢷⣻⣾⣽⣳⢯⣷⣿⠿⢋⡁⢆⠡⢹⣿⣿⣧
//⣧⡻⣖⢮⣻⡜⢾⣩⢗⣮⢟⡼⣟⣿⣯⣿⣿⣿⢿⣻⣿⣿⣻⣽⣿⣿⣷⣿⣿⣿⣿⡟⠟⡛⢋⠍⣄⠢⣡⢘⣄⣣⣼⣿⣿⣿
//⣷⢫⡽⢧⣳⡽⢧⣻⢞⡽⣾⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣟⡾⣽⣻⢿⡿⣿⢿⣻⢿⣻⢿⣻⣟⣯⣿⣿⣿
//⣯⣷⣻⢯⣗⡿⣯⣷⣯⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠋⠁⠀⠘⣿⣿⣿⣿⣿⣳⣯⢿⡽⣯⢿⡽⣯⣟⣯⣷⣿⣿⣿⣿⠓
//⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢟⡻⢯⣿⣿⣿⣓⢾⡇⠀⠐⠠⠀⠙⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⣿⡿⠁⠀
//⣿⣿⠿⠻⠛⠟⠋⠩⣿⣿⣿⣏⠮⣕⢯⣿⣿⣿⡭⢞⣷⠀⠀⢀⠁⠀⡀⠀⠉⠛⠻⠿⠿⠿⠛⠛⠩⠙⠁⠁⠀⣿⣿⣿⠀⠀
//⠋⠁⠀⢸⠙⡆⠀⠰⣿⣿⣿⡏⣞⡱⣞⣿⣿⣿⡝⢮⣿⡆⠀⠀⠠⠀⠀⠀⠂⢀⠀⠀⠀⠀⢀⠀⠄⠀⠀⠁⢠⣿⣿⣿⣦⠀
//⠀⠀⡜⢫⡀⣿⢧⢘⣿⣿⣿⢳⠼⣱⣹⣿⣿⣿⡝⢦⢻⣷⠀⠈⠀⡀⠈⠀⠀⡀⠀⠀⠂⠀⠀⠀⠀⢀⠈⠀⢰⣿⣿⣿⣿⠀
//⠀⠀⠙⠋⠁⠈⠁⢈⣿⣿⣿⡏⣞⡱⢞⣿⣿⣿⡝⣎⢧⢻⣧⠀⠀⠠⠀⠄⠂⠀⠀⠄⠀⠂⠀⠁⠄⠠⠀⠠⣿⣿⣿⣿⣿⠀
//⠀⠀⠀⠀⠀⠀⠀⢈⣿⣿⣿⡷⣸⢱⣫⣿⣿⣿⡽⡸⢎⢧⢻⣦⣀⠀⠀⠀⠀⠈⠀⠀⠀⠐⠈⠀⠀⠀⣠⣽⠿⣿⣿⣿⠁⠀
//⠀⠀⠀⠀⠀⠀⠀⠨⣿⣿⣿⡷⣡⠧⣝⣿⣿⣿⡷⣩⢻⢬⡓⣎⢟⡷⣾⣶⣶⣤⣆⣤⣠⣄⣤⣦⣶⡿⢻⡱⣛⣿⣿⣟⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣥⣛⢼⣻⣿⣿⣗⡣⣏⢶⡹⣚⡼⣜⡱⣎⡼⣩⢏⡽⣩⢏⢯⡙⡦⣝⢣⢷⣻⣿⣿⡯⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣧⠽⣸⠖⣭⢳⠼⣸⣽⣿⣷⣯⣾⣶⣷⣾⣮⣽⣷⣯⡝⢮⣿⣿⣿⡇⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠿⠻⠟⢿⣿⣿⣟⠮⡕⣏⢮⠳⣍⣳⣿⣿⣿⡿⢿⣿⣿⣿⣿⠿⣏⠳⣜⢯⣿⣿⣿⠃⠀⠀
//⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⡹⣜⢣⠯⣝⢲⣹⣿⣿⣿⠀⠀⢿⣿⣿⣧⣛⣬⡛⣼⣻⣿⣿⣿⠀⠀⠀      
//?.
"#
        }

        23 => {
            r#"
//.?
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⣤⣤⣤⣤⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⠛⠛⠛⠛⠿⠿⣿⣿⣷⣄⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣷⠀⠀
// ⠀⠀⢀⣠⣤⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣇⠀
// ⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣤⣤⣤⣤⣤⣤⣤⣴⣶⣿⣿⡿⠀
// ⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀
// ⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀
// ⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀
// ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀
// ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀
// ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀
// ⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
// ⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
// ⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀
// ⠀⠀⠀⠙⠿⠿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⢐⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⡿⣟⣯⣿⠟⡉⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⢿⣽⣿⣿⣿⠿⠿⠟⠒⠉⠉⠉⠉⠉⠉⠉⠙⠋⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠿⠋⠉⢀⣠⣤⣤⡔⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠾⠛⠋⠉⠀⢀⣀⠐⣤⣶⣶⡤⢤⣤⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣰⣶⣾⣿⣿⣿⣆⠀⣀⣀⡀⣀⡀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠀⢀⢀⣀⠀⣀⣈⡿⠿⠿⠽⠃⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠿⠿⠿⠿⠾⠟⢁⣀⡴⣦⠆⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢦⣤⣀⣀⠀⠀⠀⠀⢘⣿⣍⡷⠆⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢶⣄⠈⠉⠛⠛⠿⠓⠀⠉⠋⠉⣀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⡀⠙⠻⢶⣶⡤⠀⠀⠛⠶⠾⠼⠋⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣆⠈⠻⣶⣤⡀⠀⠀⢸⠿⣶⣦⣤⣠⣾⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠙⢷⣤⣀⠈⠁⠀⠀⢠⣤⣀⠈⠉⠈⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡌⢧⣀⠉⠛⠃⠀⠀⠀⠀⠉⠛⠿⠿⠻⠃⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⢳⣄⠙⠛⢋⠁⠀⠀⠀⠘⠿⣴⣤⣄⣤⡄⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣄⡙⠛⠋⠀⠀⠀⠀⠀⠰⣤⣀⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢠⡈⠉⠉⠀⠀⠀⠀⠀⠀⢀⡈⠙⠛⠛⠛⠁⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢦⡉⠛⡁⠀⠀⠀⠀⠀⠀⠈⠻⠷⣶⣦⡆⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢠⡈⢷⣌⠙⠛⠁⠀⠀⠀⠀⠀⠀⠰⣦⣄⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⣄⡉⠛⠛⠀⠀⠀⠀⠀⠀⠀⢀⠈⠙⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢦⣀⠉⠛⠷⠖⠀⠀⠀⠀⠀⠀⠀⠘⠿⣶⣦⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⣠⣀⠙⠳⠶⠶⠀⠀⠀⠀⠀⠀⠀⠀⢠⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠙⠻⢿⣶⣤⣤⠀⠀⠀⠀⠀⠀⠀⢠⠛⠛⠻⠿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠰⣦⣄⠈⠉⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣶⡆⠀⠀⠀⠀⠀⠀⠀⠺⠿⠿⠿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⠟⠁⠀⠀⠀⠀⠀⠀⢀⣤⣤⣤⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣀⣀⣀⣀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠻⠿⠿⠧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣞⣻⣿⣿⣔⣿⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠋⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀      
//?.
"#
        }
        
        24 => {
             r#"
 //.?
//👉🏿👇🏿👇🏿👇🏿👇🏿👇🏿👇🏿👇🏿👇🏿👇🏿👈🏿

//👉🏿👇🏾👇🏾👇🏾👇🏾👇🏾👇🏾👇🏾👇🏾👇🏾👈🏿

//👉🏿👉🏾👇🏽👇🏽👇🏽👇🏽👇🏽👇🏽👇🏽👈🏾👈🏿

//👉🏿👉🏾👉🏽👇🏼👇🏼👇🏼👇🏼👇🏼👈🏽👈🏾👈🏿

//👉🏿👉🏾👉🏽👉🏼👇🏻👇🏻👇🏻👈🏼👈🏽👈🏾👈🏿

//👉🏿👉🏾👉🏽👉🏼👉🏻 ඞ 👈🏻👈🏼👈🏽👈🏾👈🏿

//👉🏿👉🏾👉🏽👉🏼👆🏻👆🏻👆🏻👈🏼👈🏽👈🏾👈🏿

//👉🏿👉🏾👉🏽👆🏼👆🏼👆🏼👆🏼👆🏼👈🏽👈🏾👈🏿

//👉🏿👉🏾👆🏽👆🏽👆🏽👆🏽👆🏽👆🏽👆🏽👈🏾👈🏿

//👉🏿👆🏾👆🏾👆🏾👆🏾👆🏾👆🏾👆🏾👆🏾👆🏾👈🏿

//👉🏿👆🏿👆🏿👆🏿👆🏿👆🏿👆🏿👆🏿👆🏿👆🏿👈🏿
//?.
 "#
         }
        _ => {
            r#"/*    
            ┼┼║┼┼ No case......
            */"#
        }
    };

    for i_ඞ in imposter_ඞ.chars() {
        let mut buf_ඞ = [0; 5];

        let x_ඞ = i_ඞ.encode_utf8(&mut buf_ඞ);

        for i_ඞ in x_ඞ.as_bytes() {
            new_data_ඞ.push(*i_ඞ);
        }
    }
}

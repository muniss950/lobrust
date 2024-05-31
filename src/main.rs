use clap::Parser;
use lobrust::cli::Args; 
use lobrust::utils;

// fn main() {
//      let args = Cli::parse();
//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
// }

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();

    // Print the parsed arguments for debugging purposes
    println!("{:?}", args);

    // Implement your logic based on the parsed arguments
    // Example: match different options and handle them accordingly
    if args.cuntinue {
        println!("Continuing from current history...");
    }

    if args.clear_history {
        println!("Clearing history...");
    }

    if let Some(Some(download_path)) = args.download {
        println!("Downloading to path: {}", download_path);
    } else if let Some(None) = args.download {
        println!("Downloading to current directory...");
    }

    if args.edit {
        println!("Editing config file...");
    }


    if args.image_preview {
        println!("Showing image previews...");
    }

    if args.json {
        println!("Outputting JSON...");
    }

    if let Some(Some(language)) = args.language {
        println!("Subtitle language: {}", language);
    } else if let Some(None) = args.language {
        println!("Subtitle language: English");
    }

    if args.rofi {
        println!("Using rofi instead of fzf...");
    }

    if let Some(provider) = args.provider {
        println!("Using provider: {}", provider);
    }

    if let Some(quality) = args.quality {
        println!("Video quality: {}", quality);
    }

    if args.quiet {
        println!("Suppressing output from mpv...");
    }

    if let Some(Some(recent_type)) = args.recent {
        println!("Selecting from recent: {}", recent_type);
    } else if let Some(None) = args.recent {
        println!("Selecting from recent: movies");
    }

    if args.syncplay {
        println!("Using Syncplay to watch with friends...");
    }

    if args.trending {
        println!("Selecting from trending movies and shows...");
    }

    if args.update {
        println!("Updating the script...");
    }


    if args.debug {
        println!("Debug mode enabled...");
    }

    if let Some(query) = args.query {
        println!("Searching for query: {}", query);
    }
    else{
        utils::get_input(args);
    }
}

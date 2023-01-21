// 1.zip file is for testing purpose, you can use any zip file

use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    //check if the user passed a filename as an argument
    if args.len() < 2 {
        println!("Usage: {}, <filename>", args[0]);
        return 1;
    }

    //get the filename from the arguments
    let filename = std::path::Path::new(&*args[1]);

    //open the file
    let file = fs::File::open(&filename).unwrap();

    //create a zip archive from the file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    //iterate over all the files in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        //get the output path for the file
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        //print the file's comment, if it has one
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        //check if the file is a directory
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            //create parent directory if it doesn't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            //create the file and copy the contents
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        //set file permissions on unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}

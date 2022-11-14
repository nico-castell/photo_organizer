use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

use iphone_organizer::{self, Config};

#[test]
fn run_works() -> Result<(), Box<dyn Error>> {
    // Prepare a source directory
    {
        fs::create_dir_all("mockSOURCE_INTEGRATION_01/202211__")?;
        fs::create_dir_all("mockSOURCE_INTEGRATION_01/202212__")?;

        File::create("mockSOURCE_INTEGRATION_01/202211__/IMG_0001.JPG")?;
        File::create("mockSOURCE_INTEGRATION_01/202211__/IMG_0002.JPG")?;
        File::create("mockSOURCE_INTEGRATION_01/202211__/IMG_0002.AAE")?;
        File::create("mockSOURCE_INTEGRATION_01/202212__/IMG_0003.JPG")?;
        File::create("mockSOURCE_INTEGRATION_01/202212__/IMG_0004.JPG")?;
    }

    // Run the test function
    {
        let args = vec![
            String::from("program_name"),
            String::from("mockSOURCE_INTEGRATION_01"),
            String::from("mockDESTINATION_INTEGRATION_01"),
        ];

        let config = Config::build(args.into_iter())?;

        iphone_organizer::run(config)?;
    }

    // Test the destination directory
    {
        let file_list = vec![
            PathBuf::from("mockDESTINATION_INTEGRATION_01/2022/11/IMG_0001.jpg"),
            PathBuf::from("mockDESTINATION_INTEGRATION_01/2022/11/IMG_0002.jpg"),
            PathBuf::from("mockDESTINATION_INTEGRATION_01/2022/11/IMG_0002.aae"),
            PathBuf::from("mockDESTINATION_INTEGRATION_01/2022/12/IMG_0003.jpg"),
            PathBuf::from("mockDESTINATION_INTEGRATION_01/2022/12/IMG_0004.jpg"),
        ];

        for file in file_list {
            if !file.exists() {
                panic!("File {} was not copied", file.display());
            }
        }
    }

    // Clean up
    {
        fs::remove_dir_all("mockSOURCE_INTEGRATION_01")?;
        fs::remove_dir_all("mockDESTINATION_INTEGRATION_01")?;
    }

    Ok(())
}

use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

use iphone_organizer::FileList;

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
        let source = PathBuf::from("mockSOURCE_INTEGRATION_01");
        let destination = PathBuf::from("mockDESTINATION_INTEGRATION_01");

        let source_list = FileList::build(&source)?;

        iphone_organizer::organize(
            &source_list,
            false,
            &source.to_str().unwrap(),
            &destination.to_str().unwrap(),
        )?;
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

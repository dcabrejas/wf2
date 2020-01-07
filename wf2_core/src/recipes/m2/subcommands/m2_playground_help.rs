use crate::recipes::m2::subcommands::m2_playground::M2Playground;

pub fn help(pg: &M2Playground) -> String {
    format!(
        r#"Next steps:

    Stop existing docker containers:

       docker stop $(docker ps -qa) && docker rm $(docker ps -qa)

    Now start wf2 in the new directory:

       cd {}
       wf2 up

    {}"#,
        pg.dir.file_name().unwrap().to_string_lossy(),
        up_help()
    )
}

pub fn up_help() -> String {
    String::from(
        "If this is your first time running this site - you
    should now run the following:

       wf2 doctor
       wf2 composer install
       wf2 exec magento-install

    That's it - you should find the site running at

       https://local.m2

    Admin credentials:

       admin
       password123

    Have fun :)

    ",
    )
}

#[test]
fn test_help() {
    let pg = M2Playground {
        dir: std::path::PathBuf::from("/user/shakyshane"),
        version: String::from("2.3.4"),
        username: "".to_string(),
        password: "".to_string(),
    };
    let _h = help(&pg);
    //    println!("{}", h);
}

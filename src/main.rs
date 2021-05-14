#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

use std::io;

mod error;
mod api;

const GRAPH_BASE_URI: &str = "https://graph.microsoft.com/beta";

pub type Result<T> = std::result::Result<T, error::Error>;



fn graph_url(path: &str) -> String {
    format!("{}{}", GRAPH_BASE_URI, path)
}

fn main() -> Result<()> {
    // To acquire OAuth token, grant all "Tasks" permissions within MS Graph Explorer, then click "Access Token"
    // See: https://blog.osull.com/2020/09/14/backup-migrate-microsoft-to-do-tasks-with-powershell-and-microsoft-graph/
    // See: https://gotoguy.blog/2020/05/06/oauth-authentication-to-microsoft-graph-with-powershell-core/
    println!("Paste OAuth2 Token");

    let mut token = String::new();
    io::stdin().read_line(&mut token).expect("Failed to read line");
    let token = token.trim_end();

    println!();

    let client = reqwest::blocking::Client::new();
    
    let response = client.get(graph_url("/me"))
        .bearer_auth(token)
        .send()?;

    let me = match response.json::<api::Response<api::User>>()? {
        api::Response::Success(me) => me,
        api::Response::Error(e) => {
            println!("ERROR: Code: {} Message: {}", e.error.code, e.error.message);
            return Ok(());
        }
    };

    println!("User: {} / {}", me.display_name, me.user_principal_name);

    Ok(())
}

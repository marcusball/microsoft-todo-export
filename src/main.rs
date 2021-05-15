#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

use std::io;

mod error;
mod api;

use error::Result;

const GRAPH_BASE_URI: &str = "https://graph.microsoft.com/beta";


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
    let token = token.trim();

    println!();

    let client = reqwest::blocking::Client::new();
    
    let response = client.get(graph_url("/me"))
        .bearer_auth(token)
        .send()?;

    let me = match response.json::<api::Response<api::user::User>>()? {
        api::Response::Success(me) => me,
        api::Response::Error(e) => {
            println!("ERROR: Code: {} Message: {}", e.error.code, e.error.message);
            return Ok(());
        }
    };

    println!("User: {} / {}", me.display_name, me.user_principal_name);

    let lists: api::tasks::TodoTaskListCollection = client.get(graph_url("/me/todo/lists"))
        .bearer_auth(token)
        .send()?
        .json()?;

    println!();
    println!("Todo Lists:");

    for (i, list) in lists.value.iter().enumerate() {
        println!("{}. {}", i + 1, list.display_name);
    }

    println!();
    println!("Enter number of list to fetch: ");

    let selected_list: &api::tasks::TodoTaskList = {
        loop {
            let mut index_str = String::new();
            io::stdin().read_line(&mut index_str).expect("Failed to read selected list!");
            let selected_list_index: u32 = index_str.trim().parse()?;

            let selected_list = lists.value.get((selected_list_index as usize) - 1);

            if let Some(list) = selected_list {
                break list;
            }
        }
    };

    println!();
    println!("Fetching list: {}", selected_list.display_name);

    Ok(())
}

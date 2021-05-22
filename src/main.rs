#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

use std::io;
use serde::de::DeserializeOwned;

mod error;
mod api;

use api::Collection;
use error::Result;

const GRAPH_BASE_URI: &str = "https://graph.microsoft.com/beta";

/// A hacky fucking struct for reading from a paged `Collection`. 
struct CollectionReader<'a, T> where T: DeserializeOwned + Clone {
    /// The `reqwest` client from which to read the next links (pages) in the collection.
    client: &'a reqwest::blocking::Client,

    /// The user's OAuth access `token`. 
    token: &'a str,

    /// The `Collection` to read.
    collection: Option<api::Collection<T>>,

    /// The full list of `items` which have been read from the `Collection` so far.
    items: Vec<T>,

    /// When iterating, the current index in the `items` vec.
    iter_index: usize,
}

impl<'a, T: DeserializeOwned + Clone> CollectionReader<'a, T> {

    /// Create a new collection reader, given the `client` and the access `token`. 
    pub fn new(client: &'a reqwest::blocking::Client, token: &'a str) -> Self {
        Self {
            client,
            token,
            collection: None,
            items: Vec::new(),
            iter_index: 0,
        }
    }

    /// First action:
    /// Fetch the requested collection from the given `url`. 
    pub fn fetch<S: AsRef<str>>(&mut self, url: S) -> Result<usize> {
        self.fetch_inner(url)
    }

    /// Fetch the requested collection from the given `url`. 
    /// Append the received collection items into the `items` property. 
    fn fetch_inner<S: AsRef<str>>(&mut self, url: S) -> Result<usize> {
        self.collection = Some(self.client
            .get(url.as_ref())
            .bearer_auth(self.token)
            .send()?
            .json()?);

        let new_item_count = self.collection.as_ref().unwrap().value.len();

        // Copy all of the newly fetched items and put them into the `items` vec.
        self.items.append(&mut self.collection.as_ref().unwrap().value.clone());

        Ok(new_item_count)
    }

    /// Fetch the next page of items into the `items` property.
    pub fn fetch_next(&mut self) -> Result<usize> {
        if self.collection.is_none() {
            return Ok(0);
        }

        let link = self.collection.as_ref().unwrap().odata.next_link.clone();

        match link {
            Some(link) => self.fetch_inner(&link),
            None => Ok(0)
        }
    }

    /// Does the collection have any further links (pages)? 
    pub fn has_next_link(&self) -> bool {
        match &self.collection {
            Some(c) => c.odata.next_link.is_some(),
            None => false
        }
    }
}

impl<'a, T: DeserializeOwned + Clone> Iterator for CollectionReader<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.collection.is_none() {
            return None;
        }

        if self.collection.as_ref().unwrap().value.is_empty() {
            return None;
        }

        // If we're at the last item in the currently loaded items
        if self.iter_index == self.items.len() {
            if !self.has_next_link() {
                return None;
            }

            self.fetch_next().expect("Failed to fetch next items!");
        }

        if self.iter_index == self.items.len() {
            return None;
        }

        let fetch_index = self.iter_index;
        self.iter_index += 1;

        self.items.get(fetch_index).map(|item_ref| item_ref.clone())
    }
}


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

    let lists: Collection<api::tasks::TodoTaskList> = client.get(graph_url("/me/todo/lists"))
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
    println!("Fetching list: {} ({})", selected_list.display_name, selected_list.id);

    let fetch_url = graph_url(&format!("/me/todo/lists/{}/tasks", selected_list.id));

    let mut task_collection = CollectionReader::<api::tasks::TodoTask>::new(&client, &token);
    task_collection.fetch(fetch_url)?;

    println!();
    println!("Tasks: ");

    for task in task_collection {
        println!("{}", task.title);
    }

    Ok(())
}

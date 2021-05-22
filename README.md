# microsoft-todo-export
Quick and dirty application for fetching all tasks from a Microsoft Todo list.

This is heavily based on [dan-osnull's](https://github.com/dan-osull) [Todo Backup/Restore Powershell script](https://github.com/dan-osull/PowerShell-Backup-Restore-Microsoft-Todo). 
See his [explanation blog post](https://blog.osull.com/2020/09/14/backup-migrate-microsoft-to-do-tasks-with-powershell-and-microsoft-graph/) for further information. 

## Usage

 * Following the instructions from the above blog post, use [Microsoft's Graph Explorer](https://developer.microsoft.com/en-us/graph/graph-explorer) to retrieve an OAuth Token. 
 * Use `cargo run` to run the program.
 * Paste in your auth token when prompted. 
 * Select the desired list. 
 * The list of tasks will be downloaded and printed. 
 
## Notes

This is far from any sort of "complete" program. All **I** needed was to download all task names to a text file, and this acheived that. 
However, while untested, this should retrieve all of the properties present on the Todo Lists, and Tasks within them (as well as the User),
so anyone familiar with Rust could likely adapt this to their own needs. 

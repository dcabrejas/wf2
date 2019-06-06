use crate::{context::Context, task::Task} 
use std::path::PathBuf;

pub fn exec(ctx: &Context, trailing: Vec<String>) -> Vec<Task> {
    
    let container_name = format!("wf2__{}__php", ctx.name);
    let prefix = PathBuf::from("/var/www");

    let create_command = |file: String| {
    let path = prefix.join(file.clone());
    let to = path.parent().unwrap_or(&prefix);    
        format!(
            r#"docker cp {from} {container_name}:{to}"#,
            container_name = container_name,
            to = to.display(),
            from = file
        )
    };

    trailing
        .iter()
        .map(|file| Task::SimpleCommand {
            command: create_command(file.clone()),
        })
        .collect()
}

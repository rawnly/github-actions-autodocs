use std::io::Write;
use std::path::Path;

use clap::Parser;
use github_actions_autodocs::cli::Args;
use github_actions_autodocs::models::Action;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !is_valid_yaml(&args.file) {
        println!("The file is not a valid yaml/yml file");
        return Ok(());
    }

    let file_path = Path::new(&args.file);
    let action = Action::read_from_file(&file_path)?;

    let mut inputs_markdown = String::from(
        r#"
## Inputs
| Name | Description | Default | Required | 
| ---- | ----------- | ------- | -------- |
"#,
    );

    let mut outputs_markdown = String::from(
        r#"
## Outputs 
| Name | Description |
| ---- | ----------- |
"#,
    );

    if let Some(inputs) = action.inputs {
        for (name, input) in inputs {
            inputs_markdown += &format!("{}\n", input.to_markdown(&name));
        }
    }

    if let Some(outputs) = action.outputs {
        for (name, output) in outputs {
            outputs_markdown += &format!("{}\n", output.to_markdown(&name));
        }
    }

    let readme = format!(
        r#"
# {}
> {}

{}

{}
        "#,
        action
            .name
            .unwrap_or(file_path.file_name().unwrap().to_str().unwrap().into()),
        action.description.clone().unwrap_or_default(),
        inputs_markdown,
        outputs_markdown
    );

    // write to file
    let mut file = std::fs::File::create("README.md")?;
    file.write_all(readme.as_bytes())?;

    Ok(())
}

/// checks if the file
/// is a valid yaml/yml file by checking the extension
fn is_valid_yaml(path: &str) -> bool {
    let path = Path::new(path);

    let extension = path.extension().unwrap().to_str().unwrap();

    extension == "yaml" || extension == "yml"
}

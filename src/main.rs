use colored::*;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use github_actions_autodocs::cli::Args;
use github_actions_autodocs::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !is_valid_yaml(&args.file) {
        println!("The file is not a valid yaml/yml file");
        return Ok(());
    }

    let file_path = Path::new(&args.file);
    let action = Action::read_from_file(&file_path)?;

    // Inputs

    // empty string if there are no inputs
    let mut inputs_markdown = match action.inputs {
        None => String::from("> No inputs"),
        _ => String::from(
            r#"
| Name | Description | Default | Required | 
| ---- | ----------- | ------- | -------- |
"#,
        ),
    };

    if let Some(inputs) = action.inputs {
        for k in inputs.sorted_keys() {
            let input = inputs.0.get(k).unwrap();
            inputs_markdown += &format!("{}\n", input.to_markdown(k));
        }
    }

    // Outputs

    // empty string if there are no outputs
    let mut outputs_markdown = match action.outputs {
        None => String::from("> No outputs"),
        _ => String::from(
            r#"
| Name | Description |
| ---- | ----------- |
"#,
        ),
    };

    if let Some(outputs) = action.outputs {
        for k in outputs.sorted_keys() {
            let output = outputs.0.get(k).unwrap();
            outputs_markdown += &format!("{}\n", output.to_markdown(k));
        }
    }

    let readme = format!(
        r#"
# {}
> {}

## Inputs 
{}

## Outputs 
{}
        "#,
        action
            .name
            .unwrap_or(file_path.file_name().unwrap().to_str().unwrap().into()),
        action.description.clone().unwrap_or_default(),
        inputs_markdown,
        outputs_markdown
    );

    if args.dry {
        println!(
            "{}",
            format!(
                "Running in {}. No file will be created",
                "DRY mode".yellow()
            )
        );
        println!();
        println!("{}", readme.dimmed());

        return Ok(());
    }

    let filename = args.output.unwrap_or_else(|| "README.md".into());

    // write to file
    let mut file = std::fs::File::create(filename.clone())?;
    file.write_all(readme.as_bytes())?;

    let f = Path::new(&filename);

    println!(
        "{} {}",
        "created successfully!".bold().green(),
        f.file_name().unwrap().to_str().unwrap().underline().green()
    );

    Ok(())
}

/// checks if the file
/// is a valid yaml/yml file by checking the extension
fn is_valid_yaml(path: &str) -> bool {
    let path = Path::new(path);

    let extension = path.extension().unwrap().to_str().unwrap();

    extension == "yaml" || extension == "yml"
}

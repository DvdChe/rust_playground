use clap::Parser;
use std::process;
mod api;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "gemini-2.5-flash")]
    model: String,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    prompt: Vec<String>,
}

fn build_prompt(prompt_parts: &Vec<String>) -> String {
    if prompt_parts.is_empty() {
        eprintln!("No prompt provided");
        process::exit(1);
    }

    let prompt_text = prompt_parts.join(" ");

    let final_prompt = format!(
        "You are an expert Linux command-line assistant. \
       Give ONLY the requested command, without markdown, without explanations. \
       If multiple steps are necessary, combine them with &&. \
       Request: {}",
        prompt_text
    );
    return final_prompt;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| {
        eprintln!("GEMINI_API_KEY must be set");
        process::exit(1);
    });

    let final_prompt = build_prompt(&args.prompt);

    let response = api::query_gemini(&args.model, &api_key, &final_prompt).await?;
    println!("{}", response);

    Ok(())
}

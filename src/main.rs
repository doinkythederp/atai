use std::{env, io::stdout};

use adapters::{chatgpt::Chatgpt, Adapter};
#[cfg(feature = "clipboard")]
use arboard::Clipboard;
use config::Config;
use crossterm::{
    cursor::MoveUp,
    terminal::{Clear, ClearType},
    tty::IsTty,
    ExecutableCommand,
};
use parse::extract_code;
use tracing_subscriber::EnvFilter;

mod adapters;
mod config;
mod parse;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::open();

    let mut args = std::env::args();
    args.next();
    let mut prompt = args.collect::<Vec<String>>().join(" ");

    if prompt.is_empty() {
        prompt = dialoguer::Input::<String>::new()
            .with_prompt("What would you like help with?")
            .interact()
            .unwrap();
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
    let cwd = env::current_dir()
        .unwrap_or_else(|_| "/".into())
        .display()
        .to_string();

    let mut adapter = Chatgpt::new(config.chatgpt.token, config.chatgpt.url);

    let mut prompt = format!(
        "You are a state-of-the-art assistant that generates `{}` shell commands in a code block. Your code does not have comments, but you include a short explanation of what it does. I am on a {} device and am in the directory `{}`, but use relative paths when concise. Here is my request:\n{}",
        shell,
        env::consts::OS,
        cwd,
        prompt
    );

    let mut used_response_lines = 0;
    let mut stdout = stdout();
    let is_tty = stdout.is_tty();

    loop {
        println!();
        let response = adapter
            .generate(&prompt, &mut |progress: String| {
                if is_tty {
                    let markdown = termimad::term_text(&progress);
                    let lines = markdown.lines.len() as u16;
                    if used_response_lines > 0 {
                        stdout
                            .execute(MoveUp(used_response_lines))
                            .unwrap()
                            .execute(Clear(ClearType::FromCursorDown))
                            .unwrap();
                    }
                    used_response_lines = lines;
                    print!("{}", markdown);
                }
            })
            .await;
        let response = response.unwrap_or_else(|err| {
            eprintln!("ChatGPT adapter failed. Tips:");
            eprintln!("- Find your token: https://chat.openai.com/api/auth/session");
            eprintln!("- Update your config: ~/.config/atai/config.toml");
            panic!("{err}");
        });
        if !is_tty {
            println!("{}", response);
        }
        println!();

        const RUN_IN_SHELL: &str = "Run in shell";
        #[cfg(feature = "clipboard")]
        const COPY_TO_CLIPBOARD: &str = "Copy to clipboard";
        const REFINE: &str = "Refine your request";
        const QUIT: &str = "Quit";

        let code = extract_code(&response);

        let run_in_shell = format!("{} ({})", RUN_IN_SHELL, code);

        let selections = &[
            run_in_shell.clone(),
            #[cfg(feature = "clipboard")]
            COPY_TO_CLIPBOARD.to_string(),
            REFINE.to_string(),
            QUIT.to_string(),
        ];

        let selection = dialoguer::Select::new()
            .with_prompt("What would you like to do?")
            .items(selections)
            .default(0)
            .interact()
            .unwrap();

        match selections[selection].as_str() {
            x if x == run_in_shell => {
                println!();
                std::process::Command::new(shell)
                    .arg("-c")
                    .arg(code)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                break;
            }
            #[cfg(feature = "clipboard")]
            COPY_TO_CLIPBOARD => {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(code).unwrap();
                break;
            }
            REFINE => {
                used_response_lines = 0;
                let refine = dialoguer::Input::<String>::new()
                    .with_prompt("Refine your request")
                    .interact()
                    .unwrap();
                prompt = format!("Refine your previous command suggestion: {}", refine);
            }
            QUIT => {
                break;
            }
            _ => unreachable!(),
        }
    }
}

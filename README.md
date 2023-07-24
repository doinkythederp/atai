# AtAI

> Get AI-powered command suggestions in your terminal, for free

## Installation (ChatGPT backend)

1. Run `cargo install --git https://github.com/doinkythederp/atai.git`
2. Log into https://chat.openai.com
3. Visit https://chat.openai.com/api/auth/session and copy your access token
5. Create an AtAI config file at `~/.config/atai/config.toml`:
   
   ```toml
   [chatgpt]
   token = "your_access_token"
   url = "custom_proxy_url" # optional
   ```
6. Optionally, make an alias for AtAI in your shell profile:
   
   ```zsh
   alias @=atai
   ```


## Usage

Use the atai command or alias (suggested) to access AI-powered suggestions in your terminal

   ```
   user$ @ Find all .txt files

   find . -type f -name "*.txt"

   This command uses the find utility to search for all files (-type f) within the current directory
   and its subdirectories, with the file name pattern *.txt to match any file ending with ".txt".

   What would you like to do?:
     Run in shell (find . -type f -name "*.txt")
     Copy to clipboard
     Refine your request
     Quit
   ```

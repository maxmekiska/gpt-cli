# gpt-cli

CLI to interact with OpenAIs ChatGPT model - `gpt-3.5-turbo`.

## Prerequisite  

Add your OpenAI API key to the following env variable `OPENAI_API_KEY`.

## How to use? - Windows

### Installation

1. clone this repo
2. build the CLI .exe via `cargo build --release`
3. add the CLI .exe path to your env variables
    - System Properties
    - Environment Variables...
    - System Variables
    - Path
    - Edit
    - New
    - add path to the `release` directory which contains the .exe
4. launch cmd or powershell and type: `gpt-cli.exe` to activate the program

### Commands

#### `exit`

- will terminate the conversation

#### `clear`

- clears the whole chat history. The conversation will start from scratch.

#### `undo`

- will remove the last request and answer from the chat history.

#### `log`

- will activate logging. Each request will print the current chat history

#### `deactivate log`

- will deactivate logging.

<br>

![demo](assets/demo.gif)

## References

<details>
  <summary>Expand</summary>
  <br>

Rust OpenAI Integration (GPT-3) - Code to the Moon: https://www.youtube.com/watch?v=5WhJQMnJjik&t=724s

</details>
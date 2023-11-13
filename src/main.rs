use env_logger;

mod oai;
mod interact;

mod chatroutine;
mod oneshotroutine;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "gpt-cli")]
#[command(author = "Max Mekiska. <maxmekiska@gmail.com>")]
#[command(version = "0.2.0")]
#[command(about = "CLI to interact with OpenAi LLMs.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    OneShot {
        #[arg(short, long, help = "The prompt for the model.")]
        prompt: String,

        #[arg(short, long, default_value_t = 300, help = "The maximum number of tokens the response should have.")]
        max_tokens: u32,

        #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"), help = "The GPT model to use.")]
        gpt_model: String,

        #[arg(short, long, default_value_t = 0.2, help = "The temperature for sampling.")]
        temperature: f32,

        #[arg(short, long, action, help = "Enable logging.")]
        logging: bool,
    },
    Chat {
        #[arg(short, long, default_value_t = 300, help = "The maximum number of tokens the response should have.")]
        max_tokens: u32,

        #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"), help = "The GPT model to use.")]
        gpt_model: String,

        #[arg(short, long, default_value_t = 0.2, help = "The temperature for sampling.")]
        temperature: f32,

        #[arg(short, long, action, help = "Enable logging.")]
        logging: bool,
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let args = Args::parse();

    match &args.command {

        Some(Commands::OneShot { prompt, max_tokens, gpt_model, temperature, logging }) => {
            oneshotroutine::one_shot(prompt.to_string(), Some(*max_tokens), Some(gpt_model.to_string()), Some(*temperature), Some(*logging)).await
        },

        Some(Commands::Chat { max_tokens, gpt_model, temperature, logging }) => {
            chatroutine::run_chat(Some(*max_tokens), Some(gpt_model.to_string()), Some(*temperature), Some(*logging)).await
        }
        None => Ok({})
    }
}

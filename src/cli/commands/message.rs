//! Message command implementations.

use clap::Parser;
use miette::IntoDiagnostic;

#[derive(Parser, Debug)]
pub enum MessageCommand {
    /// Send a message to an agent
    Send {
        /// Agent ID to send message to
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Message text to send
        message: String,
        /// Message role (user, system, assistant)
        #[arg(short = 'r', long, default_value = "user")]
        role: String,
        /// Maximum steps for processing
        #[arg(short = 's', long)]
        max_steps: Option<i32>,
        /// Disable streaming (by default, messages are streamed)
        #[arg(long)]
        no_stream: bool,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// List messages for an agent
    List {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Maximum number of messages to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: i32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
}

pub async fn handle(cmd: MessageCommand, client: &crate::LettaClient) -> miette::Result<()> {
    match cmd {
        MessageCommand::Send { .. } => {
            todo!("Message send implementation will be moved here")
        }
        MessageCommand::List { .. } => {
            todo!("Message list implementation will be moved here")
        }
    }
}

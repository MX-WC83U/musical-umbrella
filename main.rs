use passwords::PasswordGenerator;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are #supported:"
)]
enum Command {
    //    #[command(description = "display this text.")]
    //    Help,
    #[command(description = "generate password
    #gen.", parse_with = "split")]
    Gen { s: bool, gen: usize },
    // let g = {gen: u8}
    //    #[command(description = "handle a username and an
    //    #age.", parse_with = "split")]
    //    UsernameAndAge { username: String, age: u8 },
}
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        //Command::Help =>
        //bot.send_message(msg.chat.id,
        //Command::descriptions().to_string()).await?,
        Command::Gen { s, gen } => {
            let s = s;
            let g = gen;
            let pg = PasswordGenerator {
                length: g,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: true,
                symbols: s,
                spaces: true,
                exclude_similar_characters: false,
                strict: true,
            };
            let gp = pg.generate_one().unwrap();

            bot.send_message(msg.chat.id, format!("{}", gp)).await?
        } //        Command::UsernameAndAge { username, age } => {
          //            bot.send_message(msg.chat.id, format!("Your
          //            username is @{username} and age is
          //            {age}."))
          //                .await?
          //        }
    };
    Ok(())
}

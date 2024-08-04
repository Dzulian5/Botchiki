use teloxide::prelude::*;
use teloxide::types::ParseMode;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Utwórz instancję bota z tokenem, który otrzymałeś od @BotFather
    dotenv().ok();
    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    teloxide::repl(bot, |message: Message, bot: Bot| async move {
        // Odpowiedz na każdą wiadomość tekstową tym samym tekstem
        bot.send_message(message.chat.id, message.text().unwrap_or("Brak tekstu"))
            .await?;
        respond(())
    })
    .await;
}

use std::error::Error;
use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me, InputFile
    },
    utils::command::BotCommands,
};
use url::Url;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Show the main menu")]
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}

fn home_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let github_bot_url = Url::parse("https://github.com/junque1r4/rust-telegram-portifolio").unwrap();


    let callback_buttons = [
        "About me", "Jobs", "Social Media", "Skills",
    ];

    let url_buttons = [
        "Bot Repository"
    ];

    for buttons in callback_buttons.chunks(2) {
        let row = buttons
            .iter()
            .map(|&button| InlineKeyboardButton::callback(button.to_owned(), button.to_owned()))
            .collect();

        keyboard.push(row);
    }

    url_buttons.iter().for_each(|&button| {
        keyboard.push(vec![InlineKeyboardButton::url(
            button.to_owned(),
            github_bot_url.to_owned(),
        )])
    });

    InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(bot: Bot, msg: Message, me: Me, ) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Start) => {                
                // TODO: Alterar de photo para message, assim consigo alterar o texto no callback_handler.
                // TODO: Adicionar string invisivel par hyperlink da imagem "ㅤ"
                let photo_link = Url::parse("https://www.jokesforfunny.com/wp-content/uploads/2021/06/0596bdb89b60fe771acd2f5972a9d3e3-1158x1536.jpg")?;
                bot.send_photo(msg.chat.id, InputFile::url(photo_link))
                    .caption("My Portifolio!")
                    .reply_markup(home_keyboard())
                    .send()
                    .await?;
                }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

async fn inline_query_handler(bot: Bot, q: InlineQuery, ) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Inline query from: {:?}", q.from);
    let choose_home_option = InlineQueryResultArticle::new(
        "0",
        "What info do you need?",
        InputMessageContent::Text(InputMessageContentText::new("Home options:")),
    )
    .reply_markup(home_keyboard());

    bot.answer_inline_query(q.id, vec![choose_home_option.into()]).await?;

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(option) = q.data {
        let text = format!("You chose: {option}");
        bot.answer_callback_query(q.id).await?;

        if let Some(Message { id, chat, .. }) = q.message {
            match option.to_lowercase().as_str() {
                "about me" => {
                    bot.send_message(chat.id, "I'm a 24 years old developer from Brazil. I'm currently working at AleloBrasil.").await?;
                },
                _ => {
                    bot.send_message(chat.id, text).await?;
                }
            }
            //bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        log::info!("You chose: {}", option);
    }

    Ok(())
}


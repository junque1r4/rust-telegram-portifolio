use std::error::Error;
use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me, InputFile, ParseMode
    },
    utils::command::BotCommands,
};
use url::Url;
use json;
use serde_json::error::Category::Data;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use lazy_static::lazy_static;


#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Show the main menu")]
    Start,
}

#[derive(Deserialize, Clone)]
struct Config {
    home: String,
    about_me: String,
    social_media: String,
    jobs: String,
    skills: String,
    alelo: String,
}

fn load_config() -> Config {
    let mut file = File::open("config.json").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

lazy_static! {
    static ref CONFIG: Config = load_config();
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

fn create_inline_keyboard_rows(buttons: &[&str], chunk_size: usize) -> Vec<Vec<InlineKeyboardButton>> {
    let mut keyboard = Vec::new();
    for chunk in buttons.chunks(chunk_size) {
        let row: Vec<InlineKeyboardButton> = chunk
            .iter()
            .map(|&button| InlineKeyboardButton::callback(button.to_owned(), button.to_owned()))
            .collect();
        keyboard.push(row);
    }
    keyboard
}

fn add_url_buttons(keyboard: &mut Vec<Vec<InlineKeyboardButton>>, buttons: &[(&str, &str)]) {
    for (label, url_str) in buttons {
        let url = Url::parse(url_str).unwrap();
        keyboard.push(vec![InlineKeyboardButton::url(label.to_string(), url)]);
    }
}

fn home_keyboard() -> InlineKeyboardMarkup {
    let callback_buttons = [
        "About me", "Jobs", "Social Media", "Skills",
    ];

    let url_buttons = vec![
        ("Github", "https://github.com/junque1r4/rust-telegram-portifolio")
    ];

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = create_inline_keyboard_rows(&callback_buttons, 2);
    add_url_buttons(&mut keyboard, &url_buttons);

    InlineKeyboardMarkup::new(keyboard)
}

async fn social_media_buttons() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let url_buttons = vec![
        ("Github", "https://github.com/junque1r4/rust-telegram-portifolio"),
        ("Linkedin", "https://www.linkedin.com/in/joao-victor-junqueira-1b9114164/"),
        ("X", "https://twitter.com/KRNJun"),
    ];

    add_url_buttons(&mut keyboard, &url_buttons);

    let back_button = InlineKeyboardButton::callback("back", "back");

    keyboard.push(vec![back_button]);

    InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(bot: Bot, msg: Message, me: Me, ) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Start) => {                
                bot.send_message(msg.chat.id, &CONFIG.home)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(home_keyboard())
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
        InputMessageContent::Text(InputMessageContentText::new(&CONFIG.home).parse_mode(ParseMode::MarkdownV2)),
        
    )
    .reply_markup(home_keyboard());

    bot.answer_inline_query(q.id, vec![choose_home_option.into()]).await?;

    Ok(())
}

async fn back_button() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let back_button = InlineKeyboardButton::callback("back", "back");

    keyboard.push(vec![back_button]);
    InlineKeyboardMarkup::new(keyboard)
}

async fn jobs_button() -> InlineKeyboardMarkup {
    let callback_buttons = [
        "Alelo", "Vivo", "Assaí", "Freelancers", "Back"
    ];

    let keyboard: Vec<Vec<InlineKeyboardButton>> = create_inline_keyboard_rows(&callback_buttons, 2);

    InlineKeyboardMarkup::new(keyboard)
}

async fn skills_button() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let callback_buttons = [
        "Rust", "Python", "Kotlin", "Java", "Cybersecurity", "Back",
    ];

    for button in callback_buttons.chunks(2) {
        let row = button
            .iter()
            .map(| &button | InlineKeyboardButton::callback(button.to_owned(), button.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(option) = q.data {
        let text = format!("You chose: {option}");

        let config = load_config();
        bot.answer_callback_query(q.id).await?;
        if let Some(Message { id, chat, .. }) = q.message {
            // Store and pass a vector to the match list.
            match option.to_lowercase().as_str() {
                "about me" => {
                    bot.edit_message_text(chat.id, id, &CONFIG.about_me)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup( back_button().await )
                    .await?;
                },
                "back" => {
                    bot.edit_message_text(chat.id, id, &CONFIG.home)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(home_keyboard())
                        .await?;
                },
                "social media" => {
                    bot.edit_message_text(chat.id, id, &config.social_media)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(social_media_buttons().await)
                        .await?;
                },
                "jobs" => {
                    bot.edit_message_text(chat.id, id, "Each button will show you a little bit about my experience in each company")
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(jobs_button().await)
                        .await?;
                },
                "skills" => {
                    bot.edit_message_text(chat.id, id, "Each button will show you a little bit about my experience in each skill")
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(skills_button().await)
                        .await?;
                },
                "rust" => {
                    bot.edit_message_text(chat.id, id, "Each button will show you a little bit about my experience in each skill")
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(skills_button().await)
                        .await?;
                },

                "alelo" => {
                    bot.edit_message_text(chat.id, id, &CONFIG.alelo)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(back_button().await)
                        .await?;
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


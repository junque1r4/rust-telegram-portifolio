mod infos;

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


#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Show the main menu")]
    Start,
}

// Can i create a struct or constant to store all strings and texts so i can edit them easily? Make the bot easy to share

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

async fn social_media_buttons() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let github_url = Url::parse("https://github.com/junque1r4").unwrap();
    let linkedin_url = Url::parse("https://www.linkedin.com/in/joao-victor-junqueira-1b9114164/").unwrap();
    let x_url = Url::parse("https://twitter.com/KRNJun").unwrap();


    keyboard.push(vec![InlineKeyboardButton::url(
        "Github".to_owned(),
        github_url,
    )]);

    keyboard.push(vec![InlineKeyboardButton::url(
        "Linkedin".to_owned(),
        linkedin_url,
    )]);

    keyboard.push(vec![InlineKeyboardButton::url(
        "X".to_owned(),
        x_url,
    )]);

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
                bot.send_message(msg.chat.id, "Welcome to my portifolio [ㅤ](https://www.jokesforfunny.com/wp-content/uploads/2021/06/0596bdb89b60fe771acd2f5972a9d3e3-1158x1536.jpg)")
                    .parse_mode(ParseMode::Markdown)
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
        InputMessageContent::Text(InputMessageContentText::new("Welcome to my portifolio [ㅤ](https://www.jokesforfunny.com/wp-content/uploads/2021/06/0596bdb89b60fe771acd2f5972a9d3e3-1158x1536.jpg)").parse_mode(ParseMode::Markdown)),
        
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
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let callback_buttons = [
        "Alelo", "Vivo", "Assaí", "Freelancers", "Back"
    ];

    for buttons in callback_buttons.chunks(2) {
        let row = buttons
            .iter()
            .map(| &button | InlineKeyboardButton::callback(button.to_owned(), button.to_owned()))
            .collect();
    
        keyboard.push(row);
    }

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
        bot.answer_callback_query(q.id).await?;
        // These giant strings are kinda ugly, 
        // TODO: Improve string formatting and store in a struct or constant
        if let Some(Message { id, chat, .. }) = q.message {
            match option.to_lowercase().as_str() {
                "about me" => {
                    let about_me = "
[ㅤ](https://www.jokesforfunny.com/wp-content/uploads/2021/06/0596bdb89b60fe771acd2f5972a9d3e3-1158x1536.jpg)	
I'm a 24 years old Cybersecurity Analyst from Brazil. I'm currently working at [Alelo](https://www.alelo.com.br)

I don't have a degree because i quit in the last semester of my Computer Science course, because i was planning to move to another country. ( Political and Economic reasons )

I'm currently studying backend development in Rust! This bot was implemented using the [Teloxide](https://github.com/teloxide/teloxide) library and Rust!

If you want to know more about my skills you can select \"Skills\" in the main menu.";
                    bot.edit_message_text(chat.id, id, about_me)
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup( back_button().await ) // back button
                    .await?;
                },
                "back" => {
                    bot.edit_message_text(chat.id, id, "Welcome to my portifolio [ㅤ](https://www.jokesforfunny.com/wp-content/uploads/2021/06/0596bdb89b60fe771acd2f5972a9d3e3-1158x1536.jpg)")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(home_keyboard())
                        .await?;
                },
                "social media" => {
                    bot.edit_message_text(chat.id, id, "
Social Media!

Follow me on X, i'm trying to post some things! 🤝")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(social_media_buttons().await)
                        .await?;
                },
                "jobs" => {
                    bot.edit_message_text(chat.id, id, "Each button will show you a little bit about my experience in each company!")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(jobs_button().await)
                        .await?;
                },
                "skills" => {
                    bot.edit_message_text(chat.id, id, "Each button will show you a little bit about my experience in each skill!")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(skills_button().await)
                        .await?;
                }

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


use clap::{Parser, Subcommand};
use rand::seq::IteratorRandom;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// reply to a job offer command
    Offer {
        /// the name of the recruiter to address the message to
        #[arg(short, long)]
        name: String,

        /// specify a category like (gracious,polite,short,etc..)
        /// to check the categoties use --category-list
        #[arg(short, long)]
        category: Option<String>,

        /// if specified will list all the categories available for the replies
        #[arg(long)]
        category_list: bool,
    },
}

fn main() {
    let args = Args::parse();
    let (messages, categories) = init_messages();
    match args.commands {
        Commands::Offer {
            name: _,
            category: _,
            category_list,
        } if category_list => {
            // list all the categories
            println!("{:?}", categories);
        }
        Commands::Offer {
            name,
            category: Some(category),
            category_list: _,
        } => {
            // find the message by category
            let msg_content = messages
                .iter()
                .find_map(|m| {
                    m.categories
                        .iter()
                        .find(|&c| *c == category)
                        .and_then(|_| Some(m.content.clone()))
                })
                .unwrap_or_else(|| get_random_message(messages));
            let message = format!("Hi {name}. {msg_content}");
            arboard::Clipboard::new()
                .expect("could not load the clipboard")
                .set_text(message)
                .expect("could not set the message to the clipboard");
        }
        Commands::Offer {
            name,
            category: _,
            category_list: _,
        } => {
            let msg_content = get_random_message(messages);
            let message = format!("Hi {name}. {msg_content}");
            arboard::Clipboard::new()
                .expect("could not load the clipboard")
                .set_text(message)
                .expect("could not set the message to the clipboard");
        }
    }
}

fn get_random_message(messages: Vec<Message>) -> String {
    messages
        .into_iter()
        .choose(&mut rand::rng())
        .and_then(|m| Some(m.content.clone()))
        .expect("the messages should not be empty")
}

fn init_messages() -> (Vec<Message>, Vec<String>) {
    let mut categories = vec![];
    let mut messages = vec![];

    for (m, c) in RAW_MESSAGES.into_iter() {
        let mut cats = c.iter().map(|&c| c.to_string()).collect::<Vec<_>>();
        categories.append(&mut cats);
        messages.push(Message {
            content: m.to_string(),
            categories: c.iter().map(|&c| c.to_string()).collect::<Vec<_>>(),
        });
    }

    (messages, categories)
}

const RAW_MESSAGES: [(&str, &[&str]); 10] = [
    (
        "Thanks so much for reaching out and considering me for the opportunity. I’m not exploring new roles at the moment, but I’d love to stay in touch for the future. I really appreciate your message!",
        &["gracious", "future-focused"],
    ),
    (
        "I appreciate you thinking of me for this role. It sounds like an exciting opportunity, but I’m currently not looking to make a move. Please don’t hesitate to reach out again down the line!",
        &["polite", "compliment"],
    ),
    (
        "Thank you for the message! I’m not exploring new roles right now, but I’m always open to staying connected for future opportunities.",
        &["short", "courteous"],
    ),
    (
        "I’m currently not considering new roles, but I truly appreciate your message. Please feel free to reach out again in the future — I’d be happy to reconnect when the timing is right.",
        &["open"],
    ),
    (
        "Thanks so much for reaching out. I’m currently focused on my current role and not exploring other opportunities right now. That said, I’d love to keep the door open for future conversations.",
        &["context"],
    ),
    (
        "Really appreciate you getting in touch! At the moment, I’m not looking to make a move, but I value the connection and hope we can stay in touch.",
        &["friendly", "appreciative"],
    ),
    (
        "Thank you for considering me for this opportunity. I’m not in a position to pursue new roles at the moment, but I appreciate your outreach and would be glad to connect again in the future.",
        &["formal"],
    ),
    (
        "Thanks for reaching out! The role sounds interesting, but it’s not quite the right fit for me right now. Still, I’d love to stay connected in case something more aligned pops up in the future.",
        &["dont-align"],
    ),
    (
        "I appreciate your message and the opportunity. I’m not currently looking to change roles, but I’m happy to stay in touch for future possibilities.",
        &["clear", "professional"],
    ),
    (
        "Thank you for reaching out — I’m currently committed to a long-term project and not exploring new roles right now. That said, I’d love to keep in touch for potential opportunities in the future.",
        &["commitment"],
    ),
];

struct Message {
    content: String,
    categories: Vec<String>,
}

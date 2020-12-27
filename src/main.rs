extern crate failure;
extern crate futures;
extern crate handlebars;
extern crate irc;
extern crate linkify;
extern crate pickledb;
extern crate radix64;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate regex;

use futures::prelude::*;
use handlebars::Handlebars;
use irc::client::prelude::*;
use linkify::LinkFinder;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use radix64::STD;
use rand::seq::IteratorRandom;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Serialize, Deserialize)]
struct Smoker {
    smokes: i32,
    last: u64,
}

const USERNAME: &'static str = "pybot-rs";
const CHANNELS: &'static [&'static str] = &["#bot"];
const NETWORK: &'static str = "irc.darwin.network";
const PASSWORD: &'static str = "";
const FILENAME: &str = "/home/conrad/theo";

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // Configure the database
    let db = PickleDb::load(
        "pybot.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );
    let mut db = match db {
        Ok(db) => db,
        Err(_) => PickleDb::new(
            "pybot.db",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        ),
    };
    let mut retry_count = 999;
    loop {
        let channels: Vec<_> = CHANNELS.iter().map(|s| s.to_string()).collect();
        let config = Config {
            nickname: Some(USERNAME.to_owned()),
            password: Some(PASSWORD.to_owned()),
            use_tls: Some(true),
            server: Some(NETWORK.to_owned()),
            channels: channels,
            port: Some(6697),
            ..Config::default()
        };

        let mut client = Client::from_config(config).await?;
        let mut authenticated = false;

        client.send_cap_ls(NegotiationVersion::V302).unwrap();
        let mut stream = client.stream()?;

        while let Some(message) = stream.next().await.transpose()? {
            print!("{}", message);
            if message.to_string().contains("KICK #") && message.to_string().contains("pybot-rs") {
                client.send_quit(format!("GOODBYE FOREVER"))?;
                break;
            }
            authenticate(&client, &message, &mut authenticated)?;
            abuse2(&client, &message)?;
            smoke(&client, &message, &mut db)?;
            // link(&message)?;
            mktpl(&client, &message, &mut db)?;
            mkword(&client, &message, &mut db)?;
            theo(&client, &message)?;
            abuse(&client, &message, &mut db);
        }
        retry_count = retry_count - 1;
        if retry_count == 0 {
            break;
        }
    }
    Ok(())
}

fn authenticate(
    client: &irc::client::Client,
    message: &irc::proto::Message,
    mut authenticated: &bool,
) -> std::result::Result<(), failure::Error> {
    if authenticated != &true {
        // Handle CAP LS
        if message.to_string().contains("sasl=PLAIN") {
            client.send_sasl_plain().unwrap();
            print!("sasl plain available");
        }
        if message.to_string().contains("AUTHENTICATE +") {
            let toencode = format!("{}\0{}\0{}", USERNAME, USERNAME, PASSWORD);
            let encoded = STD.encode(&toencode);
            client.send_sasl(encoded).unwrap();
            print!("prompt to authenticate");
        }
        if message.to_string().contains("Authentication successful") {
            authenticated = &true;
            client.identify()?;
        }
    }

    Ok(())
}

fn abuse2(
    client: &irc::client::Client,
    message: &irc::proto::Message,
) -> std::result::Result<(), failure::Error> {
    let channel = get_channel(message);
    let splitstring = format!("PRIVMSG {} :abuse_old", channel);
    let pybotstring = format!("PRIVMSG {} :pybot-rs", channel);
    let evan = message.to_string().contains(":abuse_old daddy")
        || message.to_string().contains(":abuse_old evan");
    let shivaram = message.to_string().contains(":abuse_old shivaram");
    let vivi = message.to_string().contains(":abuse_old vivi");
    let comradeblue = message.to_string().contains(":abuse_old comradeblue");
    let wrmsr = message.to_string().contains(":abuse_old wrmsr");
    let ed = message.to_string().contains(":abuse_old ed");
    let carmen = message.to_string().contains(":abuse_old carmen");
    let garrick = message.to_string().contains(":abuse_old garrick");
    let pybot = message.to_string().contains(&pybotstring);
    let msgstr = message.to_string();

    let mut abuse_presets = HashMap::new();
    abuse_presets.insert(String::from("evan"), String::from("evan"));

    if evan {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(channel, format!("{} loves rust", trimmed))
            .unwrap();
    }
    if vivi {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client.send_privmsg(channel, format!("{} is planning on becoming a front end developer because he loves JavaScript so much", trimmed)).unwrap();
    }
    if shivaram {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();

        if rand::random() {
            client
                .send_privmsg(channel, format!("{} loves plan 9 C", trimmed))
                .unwrap();
        } else {
            client
                .send_privmsg(
                    channel,
                    format!(
                        "{} doesn't believe in the importance of american military hegemony",
                        trimmed
                    ),
                )
                .unwrap();
        }
    }
    if comradeblue {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(channel, format!("{} isn't a real programmer", trimmed))
            .unwrap();
    }
    if wrmsr {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(channel, format!("{} #1 nancy pelosi fan", trimmed))
            .unwrap();
    }
    if ed {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(
                channel,
                format!("\x0352{}: ARE THOSE FEATURES DONE YET??? HOW ARE YOUR OKRs LOOKING? Look, we're going to need you to stack rank your team mmmmmmkayyy?\x03", trimmed),
            )
            .unwrap();
    }
    if pybot {
        client.send_privmsg(channel, "sux to suck, luser").unwrap();
    }
    if carmen {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(channel, format!("\x0375{} loves android\x03", trimmed))
            .unwrap();
    }
    if garrick {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client
            .send_privmsg(
                channel,
                format!("{} loves mutability and keeping state", trimmed),
            )
            .unwrap();
    }
    if message.to_string().contains(":abuse_old")
        && !evan
        && !vivi
        && !pybot
        && !shivaram
        && !comradeblue
        && !wrmsr
        && !ed
        && !carmen
        && !garrick
    {
        let splitmsg: Vec<&str> = msgstr.split(&splitstring).collect();
        let username = splitmsg[1];
        let trimmed = username.trim();
        client.send_privmsg(channel, format!("{} loves JavaScript", trimmed))?;
    }

    Ok(())
}

fn smoke(
    client: &irc::client::Client,
    message: &irc::proto::Message,
    db: &mut PickleDb,
) -> std::result::Result<(), failure::Error> {
    let channel = get_channel(message);
    let splitstring = format!("PRIVMSG {} smoke", channel);
    if message.to_string().contains(&splitstring) {
        let msgstr = message.to_string();
        let splitmsg: Vec<&str> = msgstr.split("!").collect();
        let username = splitmsg[0].trim_start_matches(":");
        if db.get::<Smoker>(&username).is_none() {
            let epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let new_smoker = Smoker {
                smokes: 1,
                last: epoch,
            };
            db.set(&username, &new_smoker).unwrap();
            client.send_privmsg(channel, format!("That's smoke #{} for {} so far today... This brings you to a grand total of {} smoke{}. Keep up killing yourself with cancer!", new_smoker.smokes, username, new_smoker.smokes, "s"))?;
        } else {
            let epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut smoker = db.get::<Smoker>(&username).unwrap();
            smoker.smokes = smoker.smokes + 1;
            smoker.last = epoch;
            db.set(&username, &smoker).unwrap();
            client.send_privmsg(channel, format!("That's smoke #{} for {} so far today... This brings you to a grand total of {} smoke{}. Keep up killing yourself with cancer!", smoker.smokes, username, smoker.smokes, "s"))?;
        }
    }

    Ok(())
}

fn theo(
    client: &irc::client::Client,
    message: &irc::proto::Message,
) -> std::result::Result<(), failure::Error> {
    let channel = get_channel(message);
    let theo_pattern = format!("PRIVMSG {} theo", channel);
    let theo = message.to_string().contains(&theo_pattern.to_string());

    if theo {
        client.send_privmsg(channel, format!("theo: {}", find_theo().to_string()))?;
    }

    Ok(())
}

// TODO
fn link(message: &irc::proto::Message) -> std::result::Result<(), failure::Error> {
    let finder = LinkFinder::new();
    let msg = &message.to_string();
    let links: Vec<_> = finder.links(msg).collect();
    println!("{:?}", links);

    Ok(())
}

fn get_channel(message: &irc::proto::Message) -> &str {
    for channel in CHANNELS.iter() {
        if message.to_string().contains(channel) {
            return channel;
        }
    }
    ""
}

fn find_theo() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn mktpl(
    client: &irc::client::Client,
    message: &irc::proto::Message,
    db: &mut PickleDb,
) -> std::result::Result<(), failure::Error> {
    let channel = get_channel(message);
    let mktpl_pattern = format!("PRIVMSG {} :mktpl ", channel);
    let is_mktpl = message.to_string().contains(&mktpl_pattern.to_string());

    if is_mktpl {
        let msgstr = message.to_string();
        let mktpl_cmd: Vec<&str> = msgstr.split(&mktpl_pattern).collect();
        if !db.lexists("tpl") {
            db.lcreate("tpl")?;
        }
        db.ladd("tpl", &mktpl_cmd[1].trim()).unwrap();
        client.send_privmsg(channel, format!("mktpl added: {}", mktpl_cmd[1]))?;
    }

    Ok(())
}

fn mkword(
    client: &irc::client::Client,
    message: &irc::proto::Message,
    db: &mut PickleDb,
) -> std::result::Result<(), failure::Error> {
    // TODO: mkword and mktpl are basically the sasame fn
    let channel = get_channel(message);
    let mkword_pattern = format!("PRIVMSG {} :mkword ", channel);
    let is_mkword = message.to_string().contains(&mkword_pattern.to_string());

    if is_mkword {
        let msgstr = message.to_string();
        let mkword_cmd: Vec<&str> = msgstr.split(&mkword_pattern).collect();
        let mkword_kv: Vec<&str> = mkword_cmd[1].split(" ").collect();
        if !db.lexists(&mkword_kv[0]) {
            db.lcreate(&mkword_kv[0])?;
        }
        db.ladd(&mkword_kv[0], &mkword_kv[1].trim()).unwrap();
        client.send_privmsg(
            channel,
            format!("mkword added: {}:{}", mkword_kv[0], mkword_kv[1]),
        )?;
    }

    Ok(())
}

fn abuse(
    client: &irc::client::Client,
    message: &irc::proto::Message,
    db: &mut PickleDb,
) -> std::result::Result<(), failure::Error> {
    let channel = get_channel(message);
    let abuse_pattern = format!("PRIVMSG {} :abuse ", channel);
    let is_abuse = message.to_string().contains(&abuse_pattern.to_string());

    if is_abuse {
        let msgstr = message.to_string();
        let abuse_cmd: Vec<&str> = msgstr.split(&abuse_pattern).collect();
        let name = abuse_cmd[1].trim();

        let tp_db_len = db.llen("tpl");
        if tp_db_len > 0 {
            let tpl_list = db.liter("tpl");
            let tpl = tpl_list.choose(&mut rand::thread_rng()).unwrap();
            let tpl_string = tpl.get_item::<String>().unwrap();

            let re = Regex::new(r"([{][{][a-zA-Z]+[}][}])+").unwrap();
            let matches = re.find_iter(&tpl_string);
            let mut replacements = Map::new();
            replacements.insert("name".to_string(), name.into());

            for m in matches {
                let word_type = m.as_str().trim_matches(|c| c == '{' || c == '}');
                if word_type.contains("name") {
                    continue;
                }
                let word_replace = db
                    .liter(word_type)
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .get_item::<String>()
                    .unwrap();

                replacements.insert(word_type.to_owned(), word_replace.into());
            }

            let reg = Handlebars::new();
            client.send_privmsg(
                channel,
                format!(
                    "{}",
                    reg.render_template(&tpl_string, &json!(replacements))?
                ),
            )?;
        }
    }

    Ok(())
}

use chrono::{Datelike, Local};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("💭 Random Life Advice Generator 💭");


    let today = Local::now().weekday().to_string();


    let advice_bank = match today.as_str() {
        "Monday" => vec![
            "Start slow. Coffee first, life later.",
            "It’s Monday — fake it ‘til you make it.",
            "Remember: even code needs a reboot sometimes.",
        ],
        "Tuesday" => vec![
            "Tuesday is just Monday’s sequel. You can survive this.",
            "Write your goals like Rust code — make them safe and clear.",
            "You’re not late; you’re just compiling your courage.",
        ],
        "Wednesday" => vec![
            "Midweek reminder: you're halfway to greatness.",
            "Keep going — even infinite loops end somehow.",
            "Bugs in life? Debug with kindness.",
        ],
        "Thursday" => vec![
            "Tomorrow’s Friday, but don’t rush your present moment.",
            "Refactor your thoughts — clean mind, clean code.",
            "If you can’t find motivation, print!(\"try again\");",
        ],
        "Friday" => vec![
            "Deploy your happiness — it’s Friday!",
            "Your weekend commit is ready to push.",
            "Error 404: Responsibility not found.",
        ],
        "Saturday" => vec![
            "Relax mode: ON. Productivity: OFF.",
            "Don’t plan. Just exist beautifully today.",
            "Your only task: recharge your soul battery.",
        ],
        "Sunday" => vec![
            "Reset your system. Tomorrow’s a new build.",
            "Take a breath — even servers rest on Sundays.",
            "Plan less, dream more.",
        ],
        _ => vec!["Time is weird, but advice is timeless."],
    };


    let mut rng = thread_rng();
    let advice = advice_bank.choose(&mut rng).unwrap();

    println!("\n📅 Today is: {}", today);
    println!("💡 Advice of the Day:\n→ {}", advice);
}

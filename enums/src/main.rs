enum Instrument {
    Guitar,
    Piano,
    Drums
}

struct InstAuthor {
    instrument: Instrument,
    name: String
}

enum Instrument_2 {
    Guitar(String),
    Piano(String),
    Drums(String)
}

struct Example {

}

struct Another {

}

enum Examples {
    First(Example),
    Second(Another)
}

struct KeyData {
    key: u32
}

struct WindowResizeData {
    x: u32,
    y: u32
}

enum Events {
    KeyPressed(KeyData),
    KeyReleased(KeyData),
    WindowResized(WindowResizeData),
    WindowMoved(u32, u32),
}

impl Events {
    fn call(&self) {
        println!("Calling from call()");
    }
}

fn accept_inst(instrument: Instrument) {

}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn to_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25
    }
}

#[derive(Debug)]
enum Races {
    Tour,
    British
}

enum Sports {
    Cycling(Races),
    Football,
    Basketball,
    Volleyball,
}

fn winners(sports: Sports) {
    match sports {
        Sports::Cycling(race) => {
            println!("Cycling race! {:?}", race);
        },
        Sports::Football => {},
        Sports::Basketball => {},
        Sports::Volleyball => {}
    }
}

fn increment_value(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    accept_inst(Instrument::Guitar);

    let ins = InstAuthor {
        instrument: Instrument::Piano,
        name: String::from("Piano")
    };

    let ins_2 = Instrument_2::Guitar(String::from("Guitar"));

    let example = Examples::First(Example{});
    let another = Examples::Second(Another{});

    let event = Events::KeyPressed(KeyData{key: 30});
    let resize = Events::WindowResized(WindowResizeData{x: 640, y: 480});


    println!("Hello, world!");

    resize.call();

    let optional: Option<u32> = None;
    let number = Some(5);

    let cents = to_cents(Coins::Dime);
    println!("Cents: {cents}");

    let sports = Sports::Cycling(Races::Tour);
    winners(sports);

    let value: Option<i32> = None;
    let value_2 = Some(30);

    let incremented_value = increment_value(value);

}

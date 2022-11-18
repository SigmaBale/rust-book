fn main() {

    enum IPAddrKind {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let home = IPAddrKind::V4(192,168,8,1);

    let loopback = IPAddrKind::V6(String::from("::1"));

    fn check_value (option: Option<i32>) -> Option<i32> {
        match option {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = check_value(five);
    let none = check_value(None);

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        Texas,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    let money = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", money);

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("This quarter is from {:#?}", state);
                25
            }
        }
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}


    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => String::from("0"),
            Message::Write(m) => String::from(m),
            Message::ChangeColor(r, g, b) => format!("r: {}, g: {}, b: {}", r, g, b),
            other => String::from("N/A"), // default match value
        }
    }
}

fn main() {
    let home = IPAddrKind::V4(127, 0, 0, 1);
    let loopback = IPAddrKind::V6(String::from("::1"));

    println!(
        "IP Addresses:: {}, {}",
        route(&home),
        route(&loopback)
    );

    let m = Message::Write(String::from("Hi dere how u doin"));

    println!("message: {}", m.call());

    // Option<T> gives idea of optional value
    // enum Option<T> {
    //   Some(T),
    //   None 
    // }

    let some_number = Some(5);
    let some_char = Some("e");
    let absent_num: Option<i32> = None;
    let other_absent_num: Option<i32> = Some(66);

    println!(
        "{}, {}",
        is_num_present(absent_num),
        is_num_present(other_absent_num)
    )
    // can't add an Option<i32> with an i32 b/c that's a dumb idea anyways

}

fn route(ip_addr: &IPAddrKind) -> String {
    match ip_addr {
        IPAddrKind::V4(f, _, _, _) => f.to_string(),
        IPAddrKind::V6(f) => String::from(f),
    }
}

fn is_num_present(num: Option<i32>) -> bool {
    match num {
        Some(_) => true,
        None => false,
    }
}
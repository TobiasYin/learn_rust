fn main() {
    let mut v = vec![Some(1), Some(2), Some(3), None, Some(4)];
    // for Some(data) in v{
    //     println!("some: {}", data)
    // }
    loop {
        let t = v.pop();

        match t {
            Some(Some(x)) => println!("{}", x),
            Some(None) => println!("inner None"),
            None => {
                println!("outer None");
                break;
            }
        }
    }
    let mut v = vec![Some(1), Some(2), Some(3), None, Some(4)];


    while let Some(Some(data)) = v.pop() {
        println!("some: {}", data);
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 11 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

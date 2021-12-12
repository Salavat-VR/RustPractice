use trait_objects::{Button, Draw, Screen};

mod lib;

struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("i have drown a select box");
    }
}


fn main() {

    let screen  = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 10,
                height: 12,
                options: vec![String::from("i love Kyiv"), String::from("everything is right")],
            }),
            Box::new(Button{
                width: 100,
                height: 500,
                label: "tap me".to_string()
            })
        ]
    };


    screen.run();


}

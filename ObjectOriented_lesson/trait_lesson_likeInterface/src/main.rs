use gui::Draw;

// 另一个使用gui的crate中，在SelectBox结构体上实现Draw trait
struct SelectBox
{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox
{
    fn draw(&self)
    {
        // code to actually draw a select box
    }
}

use gui::{Screen, Button};

// 使用trait对象来存储实现了相同trait的不同类型的值
fn main() 
{
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width :75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

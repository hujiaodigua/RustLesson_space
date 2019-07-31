/*定义通用行为的trait*/

// 定义一个带有draw方法的trait
pub trait Draw
{
    fn draw(&self);
}

// 定义一个Screen结构体，带有一个字段components
pub struct Screen
{
    pub components: Vec<Box<dyn Draw>>,
}

// 在Screen结构体上，定义一个run方法，该方法会对其components上的每一个组件调用draw方法
impl Screen
{
    pub fn run(&self)
    {
        for component in self.components.iter()
        {
            component.draw();
        }
    }
}

/*实现trait*/

// 一个实现了Draw trait的Button结构体
pub struct Button
{
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button
{
    fn draw(&self)
    {
        // 实际绘制按钮的代码
    }
}

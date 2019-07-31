// AveragedCollection结构体维护了一个整型列表和集合中所有元素的平均值
pub struct AveragedCollection
{
    list: Vec<i32>,
    average: f64,
}

// 在add结构体上实现了add,remove,average公有方法
impl AveragedCollection
{
    pub fn add(&mut self, value: i32)
    {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>
    {
        let result = self.list.pop();
        match result{
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64
    {
        self.average
    }

    pub update_average(&mut self)
    {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

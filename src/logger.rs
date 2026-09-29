

pub trait Logger {
    fn println(&self, message: &str);
    fn print(&self, message: &str);
}


pub struct DefaultLogger;
impl Logger for DefaultLogger {
    fn println(&self, message: &str) {
        println!("{}", message);
    }

    fn print(&self, message: &str) {
        print!("{}", message);
    }
}
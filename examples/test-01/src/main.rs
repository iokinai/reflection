use reflection::call_function;
use reflection::call;
use reflection::Reflection;
use reflection::Field;

#[derive(Reflection)]
struct RefTest;

#[call_function]
impl RefTest {
    pub fn a(&self, a: String) -> String {
        a
    }

    pub fn d(&self, _a: i32) {

    }

    pub fn times(&self, text: String, times: i32) {
        for _ in 0..times {
            println!("{}", text);
        }
    }

    pub fn rand(&self) -> String {
        String::from("hello world")
    }

    pub fn gggg(&self) {
        println!("zxcursed")
    }
}


fn main() {
    static RT: RefTest = RefTest;

    RefTest::struct_name();

    call!(RT, "gggg", ());
}


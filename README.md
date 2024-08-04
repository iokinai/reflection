# Reflection library for Rust

### Features

- Get name and fields of the struct
- Call method in runtime by name

### Get name and fields of the struct

In order to get name and fields of the struct in runtime add the `#[derive(Reflection)]` macro to your struct:

```rust

#[derive(Reflection)]
use reflection::Reflection;
use reflection::Field;

struct User {
    pub name: String,
    pub age: u8,
}

fn main() {
    let user = User { name: "iokinai", age: 18 };
    let struct_name = User::struct_name();
    let fields = User::fields();
}

```

### Call method in runtime by name

## RESTRICTIONS: WORKS ONLY FOR STATIC VARIABLES, FOR NOW ONLY FOR NON-STATIC METHODS AND ONLY WITH METHODS INSIDE THE DECORATED WITH `#[call_function]` IMPL BLOCK

Note: for ref arguments, pass the value without `&`

In order to call a method in runtime by name, add the `#[call_function]` macro to an `impl` block. (Note: to an `IMPL` block, not the struct itself), and then use `call!` macro to call a function:

```rust 

use reflection::call_function;
use reflection::call;

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

```

- For functions with arguments except `self` and with the return value:

`call!(<object>, <function name (&str, not String)>, <return type>, <arguments separated by comma, without self>)`:

```rust

fn main() {
    static RT: RefTest = RefTest;

    
    call!(RT, "a", String, "hello world".to_string()); // This call returns String
}

```

- For functions with arguments except `self` and without the return value:

`call!(<object>, <function name (&str, not String)>, (), <arguments separated by comma, without self>)`:

```rust

fn main() {
    static RT: RefTest = RefTest;

    
    call!(RT, "d", (), 42); // This call returns ()
    call!(RT, "times", (), "hello world".to_string(), 10); // This call return () too
}

```

- For functions without arguments except `self` and with the return value:

`call!(<object>, <function name (&str, not String)>, <return type>)`:

```rust

fn main() {
    static RT: RefTest = RefTest;

    
    call!(RT, "rand", String); // This call returns String
}

```

- For functions without arguments except `self` and without the return value:

`call!(<object>, <function name (&str, not String)>, ())`:


```rust

fn main() {
    static RT: RefTest = RefTest;

    
    call!(RT, "gggg", ()); // This call return ()
}


```
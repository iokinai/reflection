#[macro_export]
macro_rules! call {
    ($obj:expr, $name:expr, $ret_type:ty, $($arg:expr),*) => {{
        let __call_func_map = $obj.get_function_map();
        let __call_func = __call_func_map.get(&$name.to_string())
            .expect("Function not found");

        let __call_args: Vec<Box<dyn std::any::Any>> = vec![$(Box::new($arg)),*];

        let __call_result = __call_func(&__call_args);

        let __result: $ret_type = if let Some(val) = __call_result.downcast::<$ret_type>().ok() {
            *val
        } else {
            panic!("Expected return type did not match")
        };

        __result
    }};

    ($obj:expr, $name:expr, (), $($arg:expr),*) => {{
        let __call_func_map = $obj.get_function_map();
        let __call_func = __call_func_map.get(&$name.to_string())
            .expect("Function not found");

        let __call_args: Vec<Box<dyn std::any::Any>> = vec![$(Box::new($arg)),*];

        __call_func(&__call_args);

        ()
    }};

    ($obj:expr, $name:expr, $ret_type:ty) => {{
        let __call_func_map = $obj.get_function_map();
        let __call_func = __call_func_map.get(&$name.to_string())
            .expect("Function not found");


        let __call_result = __call_func(&[Box::new(&$obj)]);

        let __result: $ret_type = if let Some(val) = __call_result.downcast::<$ret_type>().ok() {
            *val
        } else {
            panic!("Expected return type did not match")
        };

        __result
    }};

    ($obj:expr, $name:expr, ()) => {{
        let __call_func_map = $obj.get_function_map();
        let __call_func = __call_func_map.get(&$name.to_string())
            .expect("Function not found");

        let __call_result = __call_func(&[Box::new(&$obj)]);

        ()
    }};
}
#[cfg(test)]
mod test {
    use my_proc_macro::function_to_string;

    const OUTPUT:&str = "";

    #[function_to_string]
    fn some_fn_for_ai(_wtv_param:&str) {
        //This is awesome function, give AI to guess and output in a structured manner
        println!("{}", output)
    }

    #[test]
    fn tests_proc_macro() {
        let x = some_fn_for_ai("some_input");
        dbg!(x);
    }
}
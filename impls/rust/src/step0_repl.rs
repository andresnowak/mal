fn read(input: &String) -> &String {
    return input;
}

fn eval(input: &String) -> &String {
    return input;
}

fn print(input: &String) -> &String {
    return input;
}

pub fn rep(input: &String) -> &String {
    let ast = read(input);
    let result = eval(ast);

    return print(result);
}

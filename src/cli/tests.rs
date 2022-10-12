use crate::compiler::AmberCompiler;

macro_rules! test_amber {
    ($code:expr, $result:expr) => {
        {
            match AmberCompiler::new($code.to_string(), None).test_eval() {
                Ok(result) => assert_eq!(result.trim(), $result),
                Err(err) => panic!("ERROR: {}", err.message.unwrap())
            }

        }
    };
}

macro_rules! test_amber_err {
    ($code:expr, $result:expr) => {
        {
            match AmberCompiler::new($code.to_string(), None).test_eval() {
                Ok(result) => panic!("PASSED: Expected error, got {}", result),
                Err(err) => assert_eq!(err.message.unwrap(), $result)
            }
        }
    };
}

#[test]
fn hello_world() {
    test_amber!("echo 'Hello World'", "Hello World");
}

#[test]
fn hello_world_error() {
    test_amber_err!("echo Hello World", "Variable 'Hello' does not exist");
}

#[test]
fn add() {
    test_amber!("echo 15 + 45", "60");
}

#[test]
fn mul() {
    test_amber!("echo 3 * 4", "12");
}

#[test]
fn div() {
    test_amber!("echo 21 / 3", "7");
}

#[test]
fn sub() {
    test_amber!("echo 21 - 7", "14");
}

#[test]
fn text() {
    test_amber!("echo 'Hello World'", "Hello World");
}

#[test]
fn bool() {
    test_amber!("echo true", "1");
    test_amber!("echo false", "0");
}

#[test]
fn number() {
    test_amber!("echo 15", "15");
}

#[test]
fn variable() {
    let code = "
        let x = 42
        echo x
        x = 21
        echo x
    ";
    test_amber!(code, "42\n21");
}

#[test]
fn nested_string_interp() {
    let code = "
        let x = 'welcome {'to'} wonderful {'world'}'
        echo x
    ";
    test_amber!(code, "welcome to wonderful world");
}

#[test]
fn complex_arithmetic() {
    let code = "
        let x = 21
        let y = 2
        let z = 3
        echo x + y * z
    ";
    test_amber!(code, "27");
}

#[test]
fn very_complex_arithmetic() {
    let code = "
        let x = 21
        let y = 2
        let z = 6
        let a = 4
        echo x + y * z / a
    ";
    test_amber!(code, "24");
}

#[test]
fn paranthesis() {
    let code = "
        let x = 21
        let y = 2
        let z = 6
        let a = 2
        echo (x + y) * z / a
    ";
    test_amber!(code, "69");
}

#[test]
fn command_interpolation() {
    let code = "
        echo $echo {$echo Hello World$}$
    ";
    test_amber!(code, "Hello World");
}

#[test]
fn command_inception() {
    let code = "
        ${${${$echo Hello World$}$}$}$
    ";
    test_amber!(code, "Hello World");
}

#[test]
fn comment() {
    let code = "
        # this is a comment
        let a = 42 # this is a comment as well
    ";
    test_amber!(code, "");
}

#[test]
fn compare_eq_texts() {
    let code = "
        let x = 'Hello World'
        let y = 'Hello World'
        echo x == y
    ";
    test_amber!(code, "1");
}

#[test]
fn compare_eq_numbers() {
    let code = "
        let x = 42
        let y = 42
        echo x == y
    ";
    test_amber!(code, "1");
}

#[test]
fn compare_neq_numbers() {
    let code = "
        let x = 42
        let y = 24
        echo x != y
    ";
    test_amber!(code, "1");
}

#[test]
fn if_statements() {
    let code = "
        let x = 42
        let y = 24
        if x == y {
            echo x
        } else {
            echo y
        }
    ";
    test_amber!(code, "24");
}

#[test]
fn if_statements_else() {
    let code = "
        let x = 42
        let y = 24
        if x == y {
            echo x
        }
        else {
            echo y
        }
    ";
    test_amber!(code, "24");
}

#[test]
fn if_statement_chain() {
    let code = "
        let x = 42
        let y = 24
        if {
            x == y {
                echo x
            }
            else {
                echo y
            }
        }
    ";
    test_amber!(code, "24");
}

#[test]
fn shorthand_add_text() {
    let code = "
        let x = 'Hello '
        x += 'World'
        echo x
    ";
    test_amber!(code, "Hello World");
}

#[test]
fn shorthand_add() {
    let code = "
        let x = 16
        x += 48
        echo x
    ";
    test_amber!(code, "64");
}

#[test]
fn shorthand_sub() {
    let code = "
        let x = 64
        x -= 16
        echo x
    ";
    test_amber!(code, "48");
}

#[test]
fn shorthand_mul() {
    let code = "
        let x = 16
        x *= 4
        echo x
    ";
    test_amber!(code, "64");
}

#[test]
fn shorthand_div() {
    let code = "
        let x = 21
        x /= 3
        echo x
    ";
    test_amber!(code, "7");
}

#[test]
fn if_statements_singleline() {
    let code = "
        let x = 42
        let y = 24
        if x == y => echo x
        else => echo y
    ";
    test_amber!(code, "24");
}

#[test]
fn if_statements_else_singleline() {
    let code = "
        let x = 42
        let y = 24
        if x == y => echo x
        else => echo y
    ";
    test_amber!(code, "24");
}

#[test]
fn if_statement_chain_singleline() {
    let code = "
        let x = 42
        let y = 24
        if {
            x == y => echo x
            else => echo y
        }
    ";
    test_amber!(code, "24");
}

#[test]
fn ternary_conditional_simple() {
    let code = "
        let a = 12 > 24
            then 42
            else 24
        echo a
    ";
    test_amber!(code, "24");
}

#[test]
fn ternary_conditional_inline() {
    let code = "
        let a = 12 > 24 then 42 else 24
        echo a
    ";
    test_amber!(code, "24");
}

#[test]
fn ternary_conditional_nested() {
    let code = "
        let a = 24 > 12
            then (12 > 24
                then 42
                else 24)
            else (12 > 6
                then 24
                else 12)
        echo a
    ";
    test_amber!(code, "24");
}

#[test]
fn infinite_loop() {
    let code = "
        let a = 0
        loop {
            a += 1
            if a == 5 {
                continue
            }
            $printf \"{a} \"$
            if a == 10 {
                break
            }
        }
    ";
    test_amber!(code, "1 2 3 4 6 7 8 9 10");
}

#[test]
fn modulo_operator() {
    let code = "
        let a = 10 % 3
        echo a
    ";
    test_amber!(code, "1");
}

#[test]
fn modulo_shorthand() {
    let code = "
        let a = 10
        a %= 3
        echo a
    ";
    test_amber!(code, "1");
}

#[test]
fn function() {
    let code = "
        fun test() {
            echo 'Hello World'
        }
        echo test()
    ";
    test_amber!(code, "Hello World");
}

#[test]
fn function_with_args() {
    let code = "
        fun test(a, b) {
            echo a
            echo b
        }
        echo test('Hello', 'World')
    ";
    test_amber!(code, "Hello\nWorld");
}

#[test]
fn function_with_args_different_types() {
    let code = "
        fun test(a, b) {
            echo a + b
        }
        echo test('Hello', 'World')
        echo test(11, 42)
    ";
    test_amber!(code, "HelloWorld\n53");
}

#[test]
fn function_with_typed_args() {
    let code = "
        fun test(a: Num, b: Num) {
            echo a + b
        }
        echo test(11, 42)
    ";
    test_amber!(code, "53");
}
#[test]
fn function_with_typed_different_args() {
    let code = "
        fun test(a: Num, b: Text) {
            echo a
            echo b
        }
        echo test(11, 'Hello')
    ";
    test_amber!(code, "11\nHello");
}

#[test]
fn function_with_typed_args_text() {
    let code = "
        fun test(a: Text, b: Text) {
            echo a + b
        }
        echo test('Hello', 'World')
    ";
    test_amber!(code, "HelloWorld");
}

#[test]
fn import_existing_file() {
    let code = "
        import * from 'tests/str/trim.ab'
        echo trim('    Hello World     ')
    ";
    test_amber!(code, "Hello World");
}

#[test]
fn import_existing_nested_file() {
    let code = "
        import * from 'tests/is_even.ab'
        echo is_even(10)
    ";
    test_amber!(code, "even");
}
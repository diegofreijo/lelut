// use std::io::Write;

// use lelut::{
//     compiler::Compiler,
//     vm::{RuntimeError, VM},
// };

// #[derive(Debug)]
// pub struct Output {
//     pub contents: String,
// }

// impl Output {
//     pub fn new() -> Self {
//         Output {
//             contents: String::new(),
//         }
//     }
// }

// impl Write for Output {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.contents.push_str(&std::str::from_utf8(buf).unwrap());

//         Ok(buf.len())
//     }
//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }

// pub fn assert_expression(exp_source: &str, expected: &str) {
//     let source = format!("print {};", exp_source);
//     let mut compiler = Compiler::from_source(&source);
//     let frame = compiler.compile();

//     assert!(!compiler.had_error);

//     let mut vm = VM::new();
//     let mut stdout = Output::new();

//     vm.run_main(&frame, &mut stdout).unwrap();

//     assert_eq!(stdout.contents.trim_end_matches("\n"), expected);
// }

// pub fn assert_script_output(script_source: &str, expected: &str) {
//     let source = format!("{}", script_source);
//     let mut compiler = Compiler::from_source(&source);
//     let frame = compiler.compile();

//     assert!(!compiler.had_error);

//     let mut vm = VM::new();
//     let mut stdout = Output::new();

//     vm.run_main(&frame, &mut stdout).unwrap();

//     assert_eq!(stdout.contents.trim_end_matches("\n"), expected);
// }

// pub fn assert_script_error(script_source: &str, expected_error_message: &str) {
//     let source = format!("{}", script_source);
//     let mut compiler = Compiler::from_source(&source);
//     let frame = compiler.compile();

//     assert!(!compiler.had_error);

//     let mut vm = VM::new();
//     let mut stdout = Output::new();

//     let result = vm.run_main(&frame, &mut stdout);
//     match result.expect_err("This script should have failed") {
//         RuntimeError::Other(msg) => assert_eq!(msg.trim_end_matches("\n"), expected_error_message),
//         err => panic!("Not the expected error: {:?}", err),
//     }
// }

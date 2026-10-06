
fn test_result_1() {
    let x: Result<i32, String> = Ok(10);
    let y: Result<i32, String> = Err("failed".to_string());
    println!("x:{x:?}, y:{y:?}");
    println!("x:{:?}, y:{:?}", x.is_ok(), y.is_ok());

    println!("x:{:?} x:{:?}", y.as_ref().unwrap_or(&100), y.as_ref().unwrap_or_else(|err|{
        eprintln!("error: {err}");
        &1000
    } ));
    let cause = y.as_ref().expect_err("y is not Err");
    println!("cause: {cause}");

    let code = y.as_ref().map_err(|err| if err == "failed" {100} else {2});
    println!("code: {code:?}");

    let err_msg = "12312".to_owned();
    let z = x.as_ref().and_then(|v| {
        if *v > 0 {
            Ok(*v * 2)
        } else {
            Err(&err_msg)
        }
    });
    println!("z: {:?}", z.unwrap_or_default());

    println!("x: {:?} {:?}",x.as_ref().ok(), y.as_ref().err());

    y.expect("y is an Err");
}

pub(crate) fn test_result_all() {
    test_result_1();
}
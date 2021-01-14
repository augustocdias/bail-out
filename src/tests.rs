#[test]
fn test_ensure() {
    let error = assure!(false, "error");
    assert_eq!(error, Err("error"));

    let error = assure!(true, "error");
    assert!(error.is_ok());
}

#[test]
fn test_ensure_or() {
    let error = assure_or!(false, "ok", "error");
    assert_eq!(error, Err("error"));

    let error = assure_or!(true, "ok", "error");
    assert_eq!(error, Ok("ok"));
}

#[test]
fn test_ensure_not() {
    let error = assure_not!(false, "error");
    assert!(error.is_ok());

    let error = assure_not!(true, "error");
    assert_eq!(error, Err("error"));
}

#[test]
fn test_ensure_not_or() {
    let error = assure_not_or!(false, "ok", "error");
    assert_eq!(error, Ok("ok"));

    let error = assure_not_or!(true, "ok", "error");
    assert_eq!(error, Err("error"));
}

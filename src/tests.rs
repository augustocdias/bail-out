#[test]
fn test_ensure() {
    let error = ensure!(false, "error");
    assert_eq!(error, Err("error"));

    let error = ensure!(true, "error");
    assert!(error.is_ok());
}

#[test]
fn test_ensure_or() {
    let error = ensure_or!(false, "ok", "error");
    assert_eq!(error, Err("error"));

    let error = ensure_or!(true, "ok", "error");
    assert_eq!(error, Ok("ok"));
}

#[test]
fn test_ensure_not() {
    let error = ensure_not!(false, "error");
    assert!(error.is_ok());

    let error = ensure_not!(true, "error");
    assert_eq!(error, Err("error"));
}

#[test]
fn test_ensure_not_or() {
    let error = ensure_not_or!(false, "ok", "error");
    assert_eq!(error, Ok("ok"));

    let error = ensure_not_or!(true, "ok", "error");
    assert_eq!(error, Err("error"));
}

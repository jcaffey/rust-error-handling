use std::collections::HashMap;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::Context;

// As a general rule, library errors should be defined in an enum with thiserror
// so that consuming applications can properly handle different error types.
//
// Application level errors should typically use an anyhow::Result so that
// functions which handle results of different error types can be unwrapped using ?

// For errors that are common or live in a library - use an enum with thiserror.
#[derive(thiserror::Error, Debug)]
pub enum TcrApiError {
    #[error("Missing field: {0}")]
    FieldMissing(String),
}

// test data
fn get_data() -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    map.insert("1".into(), "a".into());
    map
}

// for errors that are specific to a certain task or are a one-time thing, just use the anyhow!
// macro.
// By returning an anyhow::Result we only have to worry about the Ok type. The error type is
// converted for us when we unwrap with ?
fn one_off_errors() -> Result<String> {
    let data = get_data();
    let val = data
        .get("doesnt-exist")
        .ok_or(anyhow!("some error that isnt common or doesnt need to be categorized"))?
        .to_string();

    let _works = "cant-parse".parse::<u8>()?;

    Ok(val)
}

fn potentially_common_error() -> Result<String> { // note this is an anyhow::Result
    let data = get_data();
    let val = data
        .get("doesnt-exist")
        .ok_or(TcrApiError::FieldMissing("field name".into()))? // anyhow::Result will properly handle a custom
        // error like TcrApiError
        .to_string();


    // We can still unwrap the result with ? because we are using an anyhow::Result
    let _works = "cant-parse".parse::<u8>()?;

    Ok(val)
}

// dont do this...
// note that we are returning a std::result::Result and not an anyhow::Result
// the Error type is defined as our custom error type so any error we don't have defined in
// TcrApiError will fail.
fn cant_have_result_with_different_error_types() -> std::result::Result<String, TcrApiError> {
    let data = get_data();
    let val = data
        .get("doesnt-exist")
        .ok_or(TcrApiError::FieldMissing("field name".into()))? // this works, but what if another
        // error can occur in this function? - see comment below
        .to_string();


    // This line would break the requirements our return type - which expects a result with a
    // TcrApiError
    // We cannot unwrap with the ? operator anymore.
    // using an anyhow::Result fixes this - see other examples
    // let boom = "cant-parse".parse::<u8>()?;

    Ok(val)
}


// to avoid looking at a log line that just says "Something bad happened."
// we can attach context to an error.
fn errors_with_context() -> Result<String> {
    let data = get_data();
    let val = data
        .get("doesnt-exist")
        .ok_or(anyhow!("the actual error"))
        .context("context of what is going on")?
        .to_string();

    Ok(val)
}


fn specific_error_with_context() -> Result<String> {
    let data = get_data();
    let val = data
        .get("doesnt-exist")
        .ok_or(TcrApiError::FieldMissing("doesnt-exist".into()))
        .context("parsing name of TCR API endpoint")?
        .to_string();

    Ok(val)
}

fn main() {
    let one_off = one_off_errors();
    println!("one_off: {:?}", one_off);

    println!("---------------");

    let common = potentially_common_error();
    println!("common: {:?}", common);

    println!("---------------");

    let limited = cant_have_result_with_different_error_types();
    println!("limited: {:?}", limited);

    println!("---------------");

    let result_with_context = errors_with_context();
    println!("general with context: {:?}", result_with_context);

    println!("---------------");

    let specific_with_context = specific_error_with_context();
    println!("specific with context: {:?}", specific_with_context);
}

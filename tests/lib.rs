extern crate odbc_sys;
extern crate odbc_safe;

use odbc_safe::*;

#[test]
fn query_result() {
    let env = Environment::allocate().warning_as_error().unwrap();
    let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();
    Connection::with_parent(&env).warning_as_error().unwrap();
}

#[test]
fn wrong_datasource() {
    let env = Environment::allocate().warning_as_error().unwrap();
    let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();
    let dbc = Connection::with_parent(&env).warning_as_error().unwrap();
    let dbc = dbc.connect(b"DoesntExist".as_ref(), b"".as_ref(), b"".as_ref())
        .map_error(|_| ())
        .warning_as_error()
        .unwrap_err();
}

#[test]
fn diagnostics() {

    let expected = if cfg!(target_os = "windows") {
        "[Microsoft][ODBC Driver Manager] Data source name not found and no default driver \
         specified"
    } else {
        "[unixODBC][Driver Manager]Data source name not found, and no default driver specified"
    };

    use std::str;

    let env = Environment::allocate().warning_as_error().unwrap();
    let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();

    let dbc = Connection::with_parent(&env).warning_as_error().unwrap();
    let dbc = dbc.connect(b"DoesntExist".as_ref(), b"".as_ref(), b"".as_ref());
    if let Error(d) = dbc {
        let mut message = [0; 512];
        match d.diagnostics(1, &mut message) {
            DiagReturn::Success(rec) => {
                let message = str::from_utf8(&mut message[..(rec.text_length as usize)]).unwrap();
                assert_eq!(expected, message);
            }
            _ => panic!("Error retriving diagnostics Diagnostics"),
        }
    }
}

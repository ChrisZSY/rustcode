fn bar() -> Result<u32, &'static str> {
    Ok(0)
}

fn foo() -> Result<i32, &'static str> {
    match bar() {
        Ok(a) =>Ok(a as i32),
        Err(e) =>Err(e),
    }
    //let a = bar() ?;
    //Ok(a as i32)
}
#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

fn do_read_file() -> Result<(), Error> {
    let data = std::fs::read("./foo")?;
    let data_str = std::str::from_utf8(&data).unwrap();

    println!("{:?}", data_str);
    Ok(())
}


fn main() -> Result<(), Error>  {
    println!("{:?}", foo());
    do_read_file()?;
    println!("test do read file");
    do_read_file()?;
    do_read_file()?;
    do_read_file()?;
    println!("Hello, world!");
    Ok(())
}

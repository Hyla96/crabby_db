use crabby_db::save_on_file;

fn main() {
    let path = ".store";
    let text = "just a text";
    let k = save_on_file(text, path);

    let _: () = match k {
        Ok(file) => file,
        Err(error) => println!("{}", error),
    };
}

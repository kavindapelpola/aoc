use crate::App;

pub fn run() {
    let app = App::new();
    let mut total = 0;
    for line in app.input {
        println!("{}", line);
    }
    println!("Part A: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;
}

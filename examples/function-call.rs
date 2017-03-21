struct Human<'a> {
    name: &'a str
}

impl<'a> Human<'a> {
    fn say(&self, msg: &str) {
        println!("{}: {}", self.name, msg)
    }
}

fn main() {
    let donald = Human {name: "Donald"};

    donald.say("you're gonna love it");
    donald.say("it's tremendous");
}

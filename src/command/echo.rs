impl Command for Echo {
    fn execute(&self) -> String {
        return self.args.join(" ");
    }
}
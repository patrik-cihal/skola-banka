use banka_delta::*;

fn main() {
    let app = App::new();

    app.test();
}

struct App {
    bank: Bank,
    test_user_id: UserId,
}

impl App {
    pub fn new() -> App {
        let mut bank = Bank::default();
        let test_user = User::new("Patrik".to_string(), "Cihal".to_string());
        let test_user_id = bank.register_user(test_user);

        App { bank, test_user_id }
    }

    pub fn test(mut self) {
        self.bank
            .events()
            .subscribe(SubscribeEvent::CreateAccount, |account| {
                println!("{:?} has been created", account);
            });
        UIInterface::new(self.bank, Some(self.test_user_id)).main_menu();
    }
}

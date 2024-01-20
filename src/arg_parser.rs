pub trait ArgParser {
    fn is_client(&self) -> bool;
    fn is_server(&self) -> bool;
}

pub struct DefaultArgParser {
    is_client: bool,
    is_server: bool,
}

impl DefaultArgParser {
    pub fn new(args: Vec<String>) -> DefaultArgParser {
        let mut is_client = true;
        let mut is_server = false;
        if args.into_iter().find(|v| v == "server").is_some() {
            is_client = false;
            is_server = true;
        }

        return DefaultArgParser {
            is_client,
            is_server,
        };
    }
}

impl ArgParser for DefaultArgParser {
    fn is_client(&self) -> bool {
        return self.is_client;
    }

    fn is_server(&self) -> bool {
        return self.is_server;
    }
}

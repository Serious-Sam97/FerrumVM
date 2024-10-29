//Account
pub struct Account {
    balance: U256,
    nonce: U256,
    storage: HashMap<U256, U256>,
    code: Vec<u8>,
}

impl Account {
    pub fn new(balance: U256, nonce: U256, code: Vec<u8>) -> Self {
        Account {
            balance,
            nonce,
            storage: HashMap::new(),
            code,
        }
    }

    pub fn get_balance(&self) -> U256 {
        self.balance
    }

    pub fn set_balance(&mut self, value: U256) {
        self.balance = value;
    }

    pub fn get_nonce(&self) -> U256 {
        self.nonce
    }

    pub fn set_nonce(&mut self, value: U256) {
        self.nonce = value;
    }

    pub fn get_storage(&self, key: U256) -> U256 {
        self.storage.get(&key).copied().unwrap_or_default()
    }

    pub fn set_storage(&mut self, key: U256, value: U256) {
        self.storage.insert(key, value);
    }

    pub fn get_code(&self) -> &[u8] {
        &self.code
    }

    pub fn set_code(&mut self, code: Vec<u8>) {
        self.code = code;
    }
}
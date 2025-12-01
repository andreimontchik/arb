use {crate::AddressType, solana_sdk::pubkey::Pubkey, std::collections::HashSet};

#[derive(Debug)]
pub struct MessageFilter {
    accounts: HashSet<AddressType>,
    owners: HashSet<AddressType>,
}

impl MessageFilter {
    pub fn new() -> Self {
        MessageFilter {
            accounts: HashSet::new(),
            owners: HashSet::new(),
        }
    }

    pub fn add_account(&mut self, account: &Pubkey) {
        self.accounts.insert(account.to_bytes());
    }

    pub fn add_owner(&mut self, owner: &Pubkey) {
        self.owners.insert(owner.to_bytes());
    }

    pub fn is_registered(&self, owner: &[u8], account: &[u8]) -> bool {
        self.owners.contains(owner) || self.accounts.contains(account)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, solana_sdk::pubkey::Pubkey};

    #[test]
    fn test_account_filtering() {
        let mut message_filter = MessageFilter::new();

        let account1 = Pubkey::new_unique();
        let account2 = Pubkey::new_unique();
        let owner1 = Pubkey::new_unique();
        let owner2 = Pubkey::new_unique();

        assert!(!message_filter.is_registered(&owner1.to_bytes(), &account1.to_bytes()));
        assert!(!message_filter.is_registered(&owner1.to_bytes(), &account2.to_bytes()));
        assert!(!message_filter.is_registered(&owner2.to_bytes(), &account1.to_bytes()));
        assert!(!message_filter.is_registered(&owner2.to_bytes(), &account2.to_bytes()));

        message_filter.add_owner(&owner1);
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account1.to_bytes()));
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account2.to_bytes()));
        assert!(!message_filter.is_registered(&owner2.to_bytes(), &account1.to_bytes()));
        assert!(!message_filter.is_registered(&owner2.to_bytes(), &account2.to_bytes()));

        message_filter.add_account(&account1);
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account1.to_bytes()));
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account2.to_bytes()));
        assert!(message_filter.is_registered(&owner2.to_bytes(), &account1.to_bytes()));
        assert!(!message_filter.is_registered(&owner2.to_bytes(), &account2.to_bytes()));

        message_filter.add_account(&account2);
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account1.to_bytes()));
        assert!(message_filter.is_registered(&owner1.to_bytes(), &account2.to_bytes()));
        assert!(message_filter.is_registered(&owner2.to_bytes(), &account1.to_bytes()));
        assert!(message_filter.is_registered(&owner2.to_bytes(), &account2.to_bytes()));
    }
}

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod rust_hackathon {
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Role {
        SuperAdmin,
        Admin,
        Moderator,
        User,
    }

    impl Default for Role {
        fn default() -> Self {
            Role::User
        }
    }

    //impl Clone for Role {
    //    fn clone(&self) -> Self {
    //        match self {
    //            Role::SuperAdmin => Role::SuperAdmin,
    //            Role::Admin => Role::Admin,
    //            Role::Moderator => Role::Moderator,
    //            Role::User => Role::User,
    //        }
    //    }
    //}

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct RustHackathon {
        /// Stores the account of the contract creator.
        account: Mapping<AccountId, Balance>,
        /// Stores the role of the account.
        role: Role,
    }

    impl RustHackathon {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(account_role: Role) -> Self {
            Self { account: Mapping::default(), role: account_role }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Role::User)
        }

        /// Returns the role of the account.
        #[ink(message)]
        pub fn get_role(&self) -> Role {
            self.role
        }

        // Set the role if the role is not user
        //#[ink(message)]
        //pub fn set_role(&self, role_to_set: Role, client: &mut RustHackathon) {
        //    if self.role != Role::User {
        //        client.role = role_to_set;
        //    }
        //}
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let rust_hackathon = RustHackathon::default();
            assert_eq!(rust_hackathon.get_role(), Role::User);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let rust_hackathon = RustHackathon::new(Role::SuperAdmin);
            assert_eq!(rust_hackathon.get_role(), Role::SuperAdmin)
        }

        #[ink::test]
        fn it_not_works() {
            let rust_hackathon = RustHackathon::new(Role::Moderator);
            assert_ne!(rust_hackathon.get_role(), Role::Admin)
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = RustHackathonRef::default();

            // When
            let contract_account_id = client
                .instantiate("rust_hackathon", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<RustHackathonRef>(contract_account_id.clone())
                .call(|rust_hackathon| rust_hackathon.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = RustHackathonRef::new(false);
            let contract_account_id = client
                .instantiate("rust_hackathon", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<RustHackathonRef>(contract_account_id.clone())
                .call(|rust_hackathon| rust_hackathon.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<RustHackathonRef>(contract_account_id.clone())
                .call(|rust_hackathon| rust_hackathon.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<RustHackathonRef>(contract_account_id.clone())
                .call(|rust_hackathon| rust_hackathon.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}

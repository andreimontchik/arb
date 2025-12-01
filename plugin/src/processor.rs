mod arbitrage_controller;
mod message_persister;

use {
    anyhow::Result,
    common::{
        message::{AccountUpdateMessage, BlockUpdateMessage, Message},
        LiquidityPool, TokenAccount,
    },
    log::{error, info},
    solana_sdk::pubkey::Pubkey,
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::Receiver,
            Arc, Mutex,
        },
        thread, time,
    },
    thiserror::Error,
};
pub use {arbitrage_controller::ArbitrageController, message_persister::MessagePersister};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("({msg})")]
    InvalidAddress { msg: String },
    #[error("({})", Pubkey::from(*address).to_string())]
    UnrecognizedAccount { address: Pubkey },
    #[error("({msg})")]
    InvalidAccountType { msg: String },
    #[error("({msg})")]
    InvalidMessageType { msg: String },
    #[error("({msg})")]
    ProcessingError { msg: String },
    #[error("({msg})")]
    DeserializationError { msg: String },
    #[error("({msg})")]
    IOError { msg: String },
}

pub trait Processor {
    fn new(config_file_name: &str) -> Self
    where
        Self: Sized;

    fn register_token_account(&mut self, msg: TokenAccount) -> Result<()>;

    fn update_block(&mut self, msg: BlockUpdateMessage) -> Result<()>;

    fn update_account(&mut self, msg: AccountUpdateMessage) -> Result<()>;

    fn update_liquidity_pool(&mut self, msg: LiquidityPool) -> Result<()>;

    fn process(&mut self, msg: Message) -> Result<()> {
        let result = match msg {
            Message::TokenAccountConfiguration(msg) => self.register_token_account(msg),
            Message::BlockUpdate(msg) => self.update_block(msg),
            Message::AccountUpdate(msg) => self.update_account(msg),
            Message::LiquidityPoolConfiguration(msg) => self.update_liquidity_pool(msg),
            _ => unimplemented!(),
        };

        result
    }
}

#[derive(Debug)]
pub struct ProcessorManager {
    receiver: Arc<Mutex<Receiver<Message>>>,
    receiver_thread_handle: Option<thread::JoinHandle<()>>,
    should_run: Arc<AtomicBool>,
}

impl ProcessorManager {
    pub fn new(rcv: Receiver<Message>) -> ProcessorManager {
        ProcessorManager {
            receiver: Arc::new(Mutex::new(rcv)),
            receiver_thread_handle: None,
            should_run: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start<T: Processor>(&mut self, processor_config: String) {
        info!("Starting ProcessorManager...");
        let receiver_handle = Arc::clone(&self.receiver);
        let should_run = Arc::clone(&self.should_run);
        let receiver_thread_handle = std::thread::spawn(move || {
            let mut processor: Box<dyn Processor> = Box::new(T::new(&processor_config));
            info!("Entering the Processor loop.");
            while should_run.load(Ordering::Relaxed) {
                match receiver_handle.lock() {
                    Ok(receiver) => match receiver.recv() {
                        Ok(msg) => {
                            if let Err(error) = processor.process(msg) {
                                error!("Failed to process message! {:?}", error)
                            }
                        }
                        Err(error) => error!("Failed to receive message! {:?}", error),
                    },
                    Err(error) => error!("Failed to obtain receiver! {:?}", error),
                }
            }
            info!("Exited the Processor loop.");
        });

        // Give some time to the Processor thread to take off.
        thread::sleep(time::Duration::from_secs(1));
        if receiver_thread_handle.is_finished() {
            error!("The Processor thread finished unexpectedy!");
            if let Err(panic) = receiver_thread_handle.join() {
                panic!("The Processor thread panicked! {:?}", panic);
            }
        } else {
            self.receiver_thread_handle = Some(receiver_thread_handle);
        }

        info!("The ProcessorManager started.");
    }

    pub fn stop(&mut self) {
        info!("Stopping the ProcessorManager...");
        self.should_run.store(false, Ordering::Relaxed);
        if let Some(handle) = self.receiver_thread_handle.take() {
            if let Err(err) = handle.join() {
                error!("Error joining the Channel Receiver thread handle {:?}", err);
            }
        }
        info!("ProcessorManager stopped.");
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        anyhow::bail,
        common::{message::AccountUpdateMessage, test_util::tests::create_liquidity_pool, AccountType},
        solana_sdk::{pubkey::Pubkey, signature::Signature},
    };
    struct MockedProcessor {
        add_account_called: bool,
        add_token_called: bool,
        update_block_called: bool,
        update_account_called: bool,
    }

    impl Processor for MockedProcessor {
        fn new(_config: &str) -> Self
        where
            Self: Sized,
        {
            MockedProcessor {
                add_account_called: false,
                add_token_called: false,
                update_block_called: false,
                update_account_called: false,
            }
        }

        fn register_token_account(&mut self, _msg: TokenAccount) -> Result<()> {
            self.add_token_called = true;
            Ok(())
        }

        fn update_block(&mut self, _msg: BlockUpdateMessage) -> Result<()> {
            self.update_block_called = true;
            Ok(())
        }

        fn update_account(&mut self, _msg: AccountUpdateMessage) -> Result<()> {
            self.update_account_called = true;
            Ok(())
        }

        fn update_liquidity_pool(&mut self, _msg: LiquidityPool) -> Result<()> {
            self.add_account_called = true;
            Ok(())
        }
    }

    struct MockedFaultyProcessor {}

    impl Processor for MockedFaultyProcessor {
        fn new(_config: &str) -> Self
        where
            Self: Sized,
        {
            Self {}
        }

        fn register_token_account(&mut self, _msg: TokenAccount) -> Result<()> {
            // Noop for now
            Ok(())
        }

        fn update_block(&mut self, _msg: BlockUpdateMessage) -> Result<()> {
            bail!(ProcessorError::ProcessingError {
                msg: "Failed to update block!".to_string(),
            })
        }

        fn update_account(&mut self, _msg: AccountUpdateMessage) -> Result<()> {
            bail!(ProcessorError::UnrecognizedAccount {
                address: Pubkey::new_unique(),
            })
        }

        fn update_liquidity_pool(&mut self, _msg: LiquidityPool) -> Result<()> {
            bail!(ProcessorError::UnrecognizedAccount {
                address: Pubkey::new_unique(),
            })
        }
    }

    #[test]
    fn test_process() {
        let mut processor = MockedProcessor::new("");
        assert!(!processor.add_account_called);
        assert!(!processor.update_account_called);

        // Add account
        assert!(processor
            .update_liquidity_pool(create_liquidity_pool(AccountType::OrcaWhirlpoolAccount))
            .is_ok());
        assert!(processor.add_account_called);
        assert!(!processor.update_account_called);

        // Process account update
        assert!(processor
            .update_account(AccountUpdateMessage {
                slot: 1,
                address: Pubkey::new_unique(),
                data: Vec::<u8>::new(),
                txn_signature: Some(Signature::new_unique()),
            })
            .is_ok());
        assert!(processor.add_account_called);
        assert!(processor.update_account_called);
    }

    #[test]
    fn test_failed_add_account() {
        let msg = Message::LiquidityPoolConfiguration(create_liquidity_pool(
            AccountType::OrcaWhirlpoolAccount,
        ));

        let mut processor = MockedFaultyProcessor {};
        assert!(processor.process(msg).is_err())
    }

    #[test]
    fn test_failed_update_account() {
        let address = Pubkey::new_unique();
        let msg = Message::AccountUpdate(AccountUpdateMessage {
            slot: 1,
            address,
            data: Vec::<u8>::new(),
            txn_signature: Some(Signature::new_unique()),
        });

        let mut processor = MockedFaultyProcessor {};
        assert!(processor.process(msg).is_err())
    }
}

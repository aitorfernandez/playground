use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use thiserror::Error;
use tokio::{sync::mpsc, time::sleep};

#[derive(Debug, Error)]
enum AccountError {
    #[error("Error acquiring lock")]
    Lock,
    #[error("Channel error")]
    Channel,
}

// Account manages the balance when notified by the ledger service and also notifies customers.
struct Account {
    balance: Arc<Mutex<i64>>,
    customer_notifier: mpsc::Sender<i64>,
}

impl Account {
    pub fn new(customer_notifier: mpsc::Sender<i64>) -> Self {
        Self {
            balance: Arc::new(Mutex::new(0)),
            customer_notifier,
        }
    }

    async fn manage_balance(&self, update: LedgerUpdate) -> Result<(), AccountError> {
        {
            let mut balance = self.balance.lock().map_err(|_| AccountError::Lock)?;
            // Since balance is already a shared, mutable reference, any modifications
            // are stored directly within self.balance.
            *balance += update.amount;
        }

        let balance = *self.balance.lock().map_err(|_| AccountError::Lock)?;
        self.customer_notifier
            .send(balance)
            .await
            .map_err(|_| AccountError::Channel)?;

        Ok(())
    }
}

struct LedgerUpdate {
    amount: i64,
}

async fn service_ledger(account: Arc<Account>) {
    let updates = vec![
        LedgerUpdate { amount: 100 },
        LedgerUpdate { amount: -30 },
        LedgerUpdate { amount: -30 },
    ];

    for update in updates {
        if let Err(e) = account.manage_balance(update).await {
            eprint!("Error updating balance account. {e}");
        }
        sleep(Duration::from_secs(1)).await;
    }
}

async fn notify_customer(mut receiver: mpsc::Receiver<i64>) {
    while let Some(balance) = receiver.recv().await {
        println!("Account update with balance: {balance}");
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    let account = Arc::new(Account::new(tx.clone()));
    let account_clone = Arc::clone(&account);

    tokio::spawn(async move {
        service_ledger(account_clone).await;
    });

    tokio::spawn(async move {
        notify_customer(rx).await;
    });

    sleep(Duration::from_secs(5)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_manage_balance() {
        // Arrange
        let (tx, mut rx) = mpsc::channel(12);
        let account = Arc::new(Account::new(tx.clone()));

        // Act
        account
            .manage_balance(LedgerUpdate { amount: 100 })
            .await
            .expect("Failed managing balance");

        // Assert
        let notification = rx.recv().await.expect("Failed getting notification");
        assert_eq!(notification, 100);
    }
}

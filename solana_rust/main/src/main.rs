// Hesap işlemleri için trait tanımı
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
    fn balance(&self) -> f64;
}

// Banka hesabı yapısı
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

// BankAccount için Account trait'inin implementasyonu
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("{} TL yatırıldı. Yeni bakiye: {} TL", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            println!("{} TL çekildi. Yeni bakiye: {} TL", amount, self.balance);
            true
        } else {
            println!("Yetersiz bakiye! Mevcut bakiye: {} TL", self.balance);
            false
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

// BankAccount için constructor ve diğer metodlar
impl BankAccount {
    fn new(account_number: String, holder_name: String, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: initial_balance,
        }
    }

    fn display_info(&self) {
        println!("Hesap Bilgileri:");
        println!("Hesap No: {}", self.account_number);
        println!("Hesap Sahibi: {}", self.holder_name);
        println!("Bakiye: {} TL", self.balance);
        println!("------------------------");
    }
}

fn main() {
    // İki farklı hesap oluştur
    let mut account1 = BankAccount::new(
        String::from("1234567890"),
        String::from("Ahmet Yılmaz"),
        1000.0,
    );

    let mut account2 = BankAccount::new(
        String::from("9876543210"),
        String::from("Ayşe Demir"),
        2000.0,
    );

    // Hesap bilgilerini göster
    account1.display_info();
    account2.display_info();

    // Para yatırma işlemi
    account1.deposit(500.0);

    // Para çekme işlemi
    account2.withdraw(1500.0);

    // Son durumu göster
    println!("\nSon Durum:");
    account1.display_info();
    account2.display_info();
}


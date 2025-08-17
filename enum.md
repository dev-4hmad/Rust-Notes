# Enum
- Used to represent one value out of multiple possible variants.
- Only one variant is active at a time.

enum PaymentMethod {
    Cash,
    CreditCard(String),
    BankTransfer(u32),
}

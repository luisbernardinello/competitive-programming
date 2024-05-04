package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

type BankAccount struct {
	AccountNumber int     
	Balance       float64 
	mux           sync.Mutex
}

type Bank struct {
	accounts []*BankAccount
	mux      sync.Mutex
}

func NewBank() *Bank {
	return &Bank{}
}

func (b *Bank) OpenAccount(initialBalance float64) {
	b.mux.Lock()
	defer b.mux.Unlock()

	newAccountNumber := len(b.accounts) + 1
	newAccount := &BankAccount{AccountNumber: newAccountNumber, Balance: initialBalance}
	b.accounts = append(b.accounts, newAccount)
}

func (b *Bank) CloseAccount(accountNumber int) {
	b.mux.Lock()
	defer b.mux.Unlock()

	for i, account := range b.accounts {
		if account.AccountNumber == accountNumber {
			b.accounts = append(b.accounts[:i], b.accounts[i+1:]...)
			return
		}
	}
}

func (b *Bank) GetTotalBalance() float64 {
	b.mux.Lock()
	defer b.mux.Unlock()

	totalBalance := 0.0
	for _, account := range b.accounts {
		account.mux.Lock()
		totalBalance += account.Balance
		account.mux.Unlock()
	}
	return totalBalance
}

func (b *Bank) Transfer(sourceAccountNumber, destinationAccountNumber int, amount float64) error {
	b.mux.Lock()
	defer b.mux.Unlock()

	sourceAccount := b.findAccountByNumber(sourceAccountNumber)
	destinationAccount := b.findAccountByNumber(destinationAccountNumber)

	if sourceAccount == nil || destinationAccount == nil {
		return fmt.Errorf("source or destination account not found")
	}

	sourceAccount.mux.Lock()
	defer sourceAccount.mux.Unlock()

	destinationAccount.mux.Lock()
	defer destinationAccount.mux.Unlock()

	if sourceAccount.Balance < amount {
		return fmt.Errorf("insufficient funds")
	}

	sourceAccount.Balance -= amount
	destinationAccount.Balance += amount

	return nil
}

func (b *Bank) Withdraw(accountNumber int, amount float64) error {
	b.mux.Lock()
	defer b.mux.Unlock()

	account := b.findAccountByNumber(accountNumber)
	if account == nil {
		return fmt.Errorf("account not found")
	}

	account.mux.Lock()
	defer account.mux.Unlock()

	if account.Balance < amount {
		return fmt.Errorf("insufficient funds")
	}

	account.Balance -= amount
	return nil
}

func (b *Bank) findAccountByNumber(accountNumber int) *BankAccount {
	for _, account := range b.accounts {
		if account.AccountNumber == accountNumber {
			return account
		}
	}
	return nil
}

func main() {
	rand.Seed(time.Now().UnixNano())
	bank := NewBank()

	for i := 0; i < 5; i++ {
		bank.OpenAccount(1000)
	}

	
	for i := 0; i < 5; i++ {
		go func() {
			for {
				accountNumber := rand.Intn(5) + 1
				depositAmount := float64(rand.Intn(101))
				bank.OpenAccount(depositAmount)
				time.Sleep(time.Duration(rand.Intn(1000)) * time.Millisecond)
			}
		}()
		go func() {
			for {
				accountNumber := rand.Intn(5) + 1
				withdrawalAmount := float64(rand.Intn(201))
				err := bank.Withdraw(accountNumber, withdrawalAmount)
				if err != nil {
					fmt.Println(err)
				}
				time.Sleep(time.Duration(rand.Intn(1000)) * time.Millisecond)
			}
		}()
		go func() {
			for {
				sourceAccountNumber := rand.Intn(5) + 1
				destinationAccountNumber := rand.Intn(5) + 1
				transferAmount := float64(rand.Intn(101))
				err := bank.Transfer(sourceAccountNumber, destinationAccountNumber, transferAmount)
				if err != nil {
					fmt.Println(err)
				}
				time.Sleep(time.Duration(rand.Intn(1000)) * time.Millisecond)
			}
		}()
	}

	
	go func() {
		for {
			fmt.Printf("Total Balance in the Bank: $%.2f\n", bank.GetTotalBalance())
			time.Sleep(5 * time.Second)
		}
	}()

	time.Sleep(60 * time.Second)
}

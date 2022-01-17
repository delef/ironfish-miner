package task

import "math/big"

type CreateMinersFeeRequest struct {
	TypeIs   string
	SpendKey string
	Amount   big.Int
	Memo     string
}

type CreateMinersFeeResponse struct {
	TypeIs                      string
	serializedTransactionPosted uint8
}

// func HandleCreateMinersFee(minerFeeFeq CreateMinersFeeRequest) CreateMinersFeeResponse {
// 	// Generate a public address from the miner's spending key
// 	minerPublicAddress := NewPublicAddress(spendKey).public_address

// 	minerNote := new Note(minerPublicAddress, amount, memo)

// 	transaction := new Transaction()
// 	transaction.receive(spendKey, minerNote)

// 	const postedTransaction = transaction.post_miners_fee()

// 	const serializedTransactionPosted = Buffer.from(postedTransaction.serialize())

// 	minerNote.free()
// 	transaction.free()
// 	postedTransaction.free()

// 	return CreateMinersFeeResponse{ Type: "createMinersFee", serializedTransactionPosted }
//   }

//   func NewPublicAddress(privateKey string) {

//   }

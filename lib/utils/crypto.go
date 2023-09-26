package utils

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
)

// Generates a random 32-byte key or an error if one occurs
func GenerateRandomKey() (string, error) {
	bytes := make([]byte, 32)

	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}

	return hex.EncodeToString(bytes), nil
}

// EncryptData encrypts the provided data using AES-256-GCM
// Implementation taken from: https://www.melvinvivas.com/how-to-encrypt-and-decrypt-data-using-aes
func EncryptData(key string, data []byte) ([]byte, error) {
	keyBytes, err := hex.DecodeString(key)

	if err != nil {
		return nil, err
	}

	block, err := aes.NewCipher(keyBytes)

	if err != nil {
		return nil, err
	}

	aesGCM, err := cipher.NewGCM(block)

	if err != nil {
		return nil, err
	}

	nonce := make([]byte, aesGCM.NonceSize())

	return aesGCM.Seal(nonce, nonce, data, nil), nil
}

// DecryptData decrypts the provided data using AES-256-GCM
// Implementation taken from: https://www.melvinvivas.com/how-to-encrypt-and-decrypt-data-using-aes
func DecryptData(key string, data []byte) ([]byte, error) {
	keyBytes, err := hex.DecodeString(key)

	if err != nil {
		return []byte{}, err
	}

	block, err := aes.NewCipher(keyBytes)

	if err != nil {
		return []byte{}, err
	}

	aesGCM, err := cipher.NewGCM(block)

	if err != nil {
		return []byte{}, err
	}

	nonceSize := aesGCM.NonceSize()

	if len(data) < nonceSize {
		return []byte{}, err
	}

	nonce, ciphertext := data[:nonceSize], data[nonceSize:]

	return aesGCM.Open(nil, nonce, ciphertext, nil)
}

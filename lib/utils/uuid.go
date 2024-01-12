package utils

import "github.com/google/uuid"

func CreateUUID() string {
	return uuid.New().String()
}

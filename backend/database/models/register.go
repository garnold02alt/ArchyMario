package models

import (
	"encoding/hex"
	"math/rand"

	"github.com/matthewhartstonge/argon2"
)

type Register struct {
	Id       interface{} `json:"_id,omitempty" bson:"_id,omitempty"`
	Username string      `json:"username" bson:"username"`
	Password string      `json:"password" bson:"password"`
	Email    string      `json:"email" bson:"email"`
	Token    string      `json:"token" bson:"token"`
}

func hashPassword(password string) (*string, error) {
	argon := argon2.DefaultConfig()
	hash, err := argon.HashEncoded([]byte(password))
	if err != nil {
		return nil, err
	}
	s := string(hash)
	return &s, nil
}

func NewRegister(username, password, email string) (*Register, error) {
	//Generate token
	tokenBytes := make([]byte, 32)
	_, err := rand.Read(tokenBytes)
	if err != nil {
		return nil, err
	}
	token := hex.EncodeToString(tokenBytes)
	//Hash password
	hash, err := hashPassword(password)
	if err != nil {
		return nil, err
	}
	return &Register{
		Username: username,
		Password: *hash,
		Email:    email,
		Token:    token,
	}, nil
}

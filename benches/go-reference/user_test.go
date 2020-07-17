package main

import (
	"bytes"
	"git.sr.ht/~sircmpwn/go-bare"
	"log"
	"testing"
)

type UserRole uint

const (
	Admin  UserRole = 0
	Normal          = 1
	Guest           = 2
)

type Session struct {
	Token   []byte
	Expires uint
}

type User struct {
	ID      uint
	Name    string
	Email   string
	Role    UserRole
	Session *Session
}

func makeAdmin() ([]byte, User) {
	session := Session{
		Token:   []byte("a2b08ecd0a0dc594ebccd607033e79262d1fa049a6d44165631b10028f97b611"),
		Expires: 42424242,
	}

	admin := User{
		ID:      42,
		Name:    "Jane Doe",
		Email:   "jdoe@example.com",
		Role:    Admin,
		Session: &session,
	}

	marshalled, err := bare.Marshal(&admin)

	if err != nil {
		log.Fatalf("Failed to marshal: %s", err)
	}

	return marshalled, admin
}

func makeGuest() ([]byte, User) {
	guest := User{
		ID:      112,
		Name:    "John Smith",
		Email:   "john@example.com",
		Role:    Guest,
		Session: nil,
	}

	marshalled, err := bare.Marshal(&guest)

	if err != nil {
		log.Fatalf("Failed to marshal: %s", err)
	}

	return marshalled, guest
}

func BenchmarkAdminSerialize(b *testing.B) {
	s, admin := makeAdmin()
	var buf bytes.Buffer
	buf.Grow(128)

	b.ResetTimer()
	b.SetBytes(int64(len(s)))
	for n := 0; n < b.N; n++ {
		w := bare.NewWriter(&buf)
		bare.MarshalWriter(w, admin)
		buf.Reset()
	}
}

func BenchmarkAdminDeserialize(b *testing.B) {
	s, _ := makeAdmin()
	var output User

	b.ResetTimer()
	b.SetBytes(int64(len(s)))
	for n := 0; n < b.N; n++ {
		_ = bare.Unmarshal(s, &output)
	}
}

func BenchmarkGuestSerialize(b *testing.B) {
	s, guest := makeGuest()
	var buf bytes.Buffer
	buf.Grow(128)

	b.ResetTimer()
	b.SetBytes(int64(len(s)))
	for n := 0; n < b.N; n++ {
		w := bare.NewWriter(&buf)
		bare.MarshalWriter(w, guest)
		buf.Reset()
	}
}

func BenchmarkGuestDeserialize(b *testing.B) {
	s, _ := makeGuest()
	var output User

	b.ResetTimer()
	b.SetBytes(int64(len(s)))
	for n := 0; n < b.N; n++ {
		_ = bare.Unmarshal(s, &output)
	}
}

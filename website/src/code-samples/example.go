package main

import (
	"context"
	"fmt"
	"log"
	"time"
)

// User represents a user in the system
type User struct {
	ID        int64     `json:"id"`
	Username  string    `json:"username"`
	Email     string    `json:"email"`
	CreatedAt time.Time `json:"created_at"`
	IsActive  bool      `json:"is_active"`
}

// UserService handles user operations
type UserService interface {
	GetUser(ctx context.Context, id int64) (*User, error)
	CreateUser(ctx context.Context, user *User) error
	DeleteUser(ctx context.Context, id int64) error
}

type userService struct {
	db Database
}

// NewUserService creates a new user service
func NewUserService(db Database) UserService {
	return &userService{db: db}
}

// GetUser retrieves a user by ID
func (s *userService) GetUser(ctx context.Context, id int64) (*User, error) {
	user := &User{}

	query := `SELECT id, username, email, created_at, is_active
	          FROM users WHERE id = $1`

	err := s.db.QueryRow(ctx, query, id).Scan(
		&user.ID,
		&user.Username,
		&user.Email,
		&user.CreatedAt,
		&user.IsActive,
	)

	if err != nil {
		return nil, fmt.Errorf("failed to get user: %w", err)
	}

	return user, nil
}

// ProcessUsers demonstrates error handling and goroutines
func ProcessUsers(ctx context.Context, service UserService, ids []int64) error {
	results := make(chan *User, len(ids))
	errors := make(chan error, len(ids))

	for _, id := range ids {
		go func(userID int64) {
			user, err := service.GetUser(ctx, userID)
			if err != nil {
				errors <- err
				return
			}
			results <- user
		}(id)
	}

	// Collect results
	var users []*User
	for i := 0; i < len(ids); i++ {
		select {
		case user := <-results:
			users = append(users, user)
		case err := <-errors:
			log.Printf("Error processing user: %v", err)
		case <-ctx.Done():
			return ctx.Err()
		}
	}

	fmt.Printf("Processed %d users successfully\n", len(users))
	return nil
}

// Constants and variables
const (
	MaxRetries     = 3
	DefaultTimeout = 30 * time.Second
)

var (
	ErrUserNotFound = fmt.Errorf("user not found")
	ErrInvalidInput = fmt.Errorf("invalid input")
)

func main() {
	ctx := context.Background()

	// Simulated database
	db := &mockDB{}
	service := NewUserService(db)

	// Create a new user
	user := &User{
		Username:  "johndoe",
		Email:     "john@example.com",
		CreatedAt: time.Now(),
		IsActive:  true,
	}

	if err := service.CreateUser(ctx, user); err != nil {
		log.Fatalf("Failed to create user: %v", err)
	}

	fmt.Println("User created successfully!")
}

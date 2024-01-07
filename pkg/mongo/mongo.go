package mongo

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"justshake/cocktails/pkg/logger"
)

type Connection struct {
	Logger *logger.Interface
	Client *mongo.Client
}

func New(url string, logger *logger.Interface) (*Connection, error) {
	// Set client options
	l := *logger
	clientOptions := options.Client().ApplyURI("mongodb://localhost:27017")

	// Connect to MongoDB
	client, err := mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		l.Fatal(err)
	}

	// Check the connection
	err = client.Ping(context.TODO(), nil)
	if err != nil {
		l.Fatal(err)
	}

	fmt.Println("Connected to MongoDB!")

	return &Connection{
		Logger: logger,
		Client: client}, nil
}

// Close -.
func (p *Connection) Close() {
	if p.Client != nil {
		err := p.Client.Disconnect(context.TODO())
		if err != nil {
			(*p.Logger).Fatal(err)
		} else {
			fmt.Println("Connection to MongoDB closed.")
		}
	}
}

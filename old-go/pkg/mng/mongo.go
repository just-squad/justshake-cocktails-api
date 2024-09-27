package mng

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/bsoncodec"
	"go.mongodb.org/mongo-driver/bson/bsonrw"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"justshake/cocktails/config"
	"justshake/cocktails/pkg/logger"
	"reflect"
)

type Connection struct {
	Logger   logger.Interface
	Client   *mongo.Client
	Database string
}

func New(cfg config.Mongo, logger logger.Interface) (*Connection, error) {
	// Set client options
	cl := options.Client()
	clientOptions := cl.SetRegistry(registerUUIDTypeInRegistry(cl.Registry)).ApplyURI(cfg.ConnectionString)

	// Connect to MongoDB
	client, err := mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		logger.Fatal(err)
	}

	// Check the connection
	err = client.Ping(context.TODO(), nil)
	if err != nil {
		logger.Fatal(err)
	}

	fmt.Println("Connected to MongoDB!")

	return &Connection{
		Logger:   logger,
		Client:   client,
		Database: cfg.Database}, nil
}

// Close -.
func (p *Connection) Close() {
	if p.Client != nil {
		err := p.Client.Disconnect(context.TODO())
		if err != nil {
			p.Logger.Fatal(err)
		} else {
			fmt.Println("Connection to MongoDB closed.")
		}
	}
}

var tUUID = reflect.TypeOf(uuid.UUID{})

// add UUID codec to registry
func registerUUIDTypeInRegistry(registry *bsoncodec.Registry) *bsoncodec.Registry {
	if registry == nil {
		registry = bson.NewRegistry()
	}

	registry.RegisterTypeEncoder(tUUID, bsoncodec.ValueEncoderFunc(uuidEncodeValue))
	registry.RegisterTypeDecoder(tUUID, bsoncodec.ValueDecoderFunc(uuidDecodeValue))

	return registry
}

func uuidEncodeValue(ec bsoncodec.EncodeContext, vw bsonrw.ValueWriter, val reflect.Value) error {
	if !val.IsValid() || val.Type() != tUUID {
		return bsoncodec.ValueEncoderError{Name: "uuidEncodeValue", Types: []reflect.Type{tUUID}, Received: val}
	}
	b := val.Interface().(uuid.UUID)
	return vw.WriteBinaryWithSubtype(b[:], bson.TypeBinaryUUID)
}

func uuidDecodeValue(dc bsoncodec.DecodeContext, vr bsonrw.ValueReader, val reflect.Value) error {
	if !val.CanSet() || val.Type() != tUUID {
		return bsoncodec.ValueDecoderError{Name: "uuidDecodeValue", Types: []reflect.Type{tUUID}, Received: val}
	}

	switch vrType := vr.Type(); vrType {
	case bson.TypeBinary:
		data, subtype, err := vr.ReadBinary()
		if err != nil {
			return err
		}
		if subtype != bson.TypeBinaryUUID {
			return fmt.Errorf("unsupported binary subtype %v for UUID", subtype)
		}
		uuid, err := uuid.FromBytes(data)
		if err != nil {
			return err
		}
		val.Set(reflect.ValueOf(uuid))
		return nil
	case bson.TypeNull:
		return vr.ReadNull()
	case bson.TypeUndefined:
		return vr.ReadUndefined()
	default:
		return fmt.Errorf("cannot decode %v into a UUID", vrType)
	}
}

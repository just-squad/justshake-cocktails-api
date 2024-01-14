package config

import (
	"fmt"
	"github.com/ilyakaznacheev/cleanenv"
	"os"
)

type (
	// Config -.
	Config struct {
		App   `yaml:"app"`
		HTTP  `yaml:"http"`
		Log   `yaml:"logger"`
		Mongo `yaml:"mongodb"`
		Tg    `yaml:"tg"`
	}

	// App -.
	App struct {
		Name    string `env-required:"true" yaml:"name"    env:"APP_NAME"`
		Version string `env-required:"true" yaml:"version" env:"APP_VERSION"`
	}

	// HTTP -.
	HTTP struct {
		Port string `env-required:"true" yaml:"port" env:"HTTP_PORT"`
	}

	// Log -.
	Log struct {
		Level string `env-required:"true" yaml:"log_level"   env:"LOG_LEVEL"`
	}

	// Mongo - Конфигурация для подключения к Mongodb серверу
	Mongo struct {
		ConnectionString string `env-required:"true" yaml:"connection_string" env:"MONGO_CONNECTION_STRING"`
		Database         string `env-required:"true" yaml:"database" env:"MONGO_DATABASE"`
	}

	Tg struct {
		Token string `yaml:"token" env:"TG_BOT_TOKEN"`
	}
)

// NewConfig returns app config.
func NewConfig() (*Config, error) {
	cfg := &Config{}

	err := cleanenv.ReadConfig("./config/config.yml", cfg)
	if err != nil {
		return nil, fmt.Errorf("config error: %w", err)
	}

	if _, err := os.Stat("./config/secret-config.yml"); err == nil {
		err = cleanenv.ReadConfig("./config/secret-config.yml", cfg)
		if err != nil {
			return nil, fmt.Errorf("config error: %w", err)
		}
	}

	err = cleanenv.ReadEnv(cfg)
	if err != nil {
		return nil, err
	}

	return cfg, nil
}

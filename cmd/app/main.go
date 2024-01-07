package main

import (
	"justshake/cocktails/config"
	"justshake/cocktails/internal/app"
	"log"
)

func main() {
	// Конфигурация
	cfg, err := config.NewConfig()
	if err != nil {
		log.Fatalf("Config error: %s", err)
	}

	// Run
	app.Run(cfg)
}

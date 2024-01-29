package app

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"justshake/cocktails/config"
	v1 "justshake/cocktails/internal/controller/http/v1"
	"justshake/cocktails/internal/infrastructure/repositories"
	"justshake/cocktails/internal/use_cases/cocktails"
	"justshake/cocktails/internal/use_cases/users"
	"justshake/cocktails/pkg/httpserver"
	"justshake/cocktails/pkg/logger"
	"justshake/cocktails/pkg/mng"
	"os"
	"os/signal"
	"syscall"
)

// Run creates objects via constructors.
func Run(cfg *config.Config) {
	var err error

	// Конфигурайция логгера
	l := logger.New(cfg.Log.Level)

	// Конфигурация репозитория
	mongo, err := mng.New(cfg.Mongo, l)
	if err != nil {
		l.Fatal(fmt.Errorf("app - Run - mng.New: %w", err))
	}
	defer mongo.Close()

	// Use case
	cocktailsUseCase := cocktails.New(repositories.NewCocktailsRepository(mongo, l))
	usersUseCase := users.New(repositories.NewUsersRepository(mongo, l))

	// Старт телеграм бота
	tgb, err := newBot(cfg,
		cocktailsUseCase,
		usersUseCase,
		l)
	if err != nil {
		l.Fatal(fmt.Errorf("app - Run - newBot: %w", err))
	}
	go tgb.startBot()
	defer tgb.stopBot()

	// HTTP Server
	handler := gin.New()
	v1.NewRouter(handler, l, cocktailsUseCase)
	httpServer := httpserver.New(handler, httpserver.Port(cfg.HTTP.Port))

	defer func() {
		l.Info("app - Run - httpServer.Shutdown: Завершаем работу приложения")
		// Shutdown
		err = httpServer.Shutdown()
		if err != nil {
			l.Error(fmt.Errorf("app - Run - httpServer.Shutdown: %w", err))
		}
	}()

	// Waiting signal
	interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt, syscall.SIGTERM)

	for {
		select {
		case s := <-interrupt:
			l.Info("app - Run - signal: " + s.String())
			return
		case err = <-httpServer.Notify():
			l.Error(fmt.Errorf("app - Run - httpServer.Notify: %w", err))
		}
	}
}

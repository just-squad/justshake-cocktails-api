package app

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	tele "gopkg.in/telebot.v3"
	"justshake/cocktails/config"
	"justshake/cocktails/internal/domain/models"
	"justshake/cocktails/internal/use_cases"
	"justshake/cocktails/pkg/logger"
	"strconv"
	"time"
)

type telegramBot struct {
	botCfg      tele.Settings
	log         logger.Interface
	stopSignal  chan bool
	botInstance *tele.Bot
	use_cases.Cocktails
}

func newBot(
	cfg *config.Config,
	cocktails *use_cases.CocktailsUseCase,
	l logger.Interface) (*telegramBot, error) {
	pref := tele.Settings{
		Token:  cfg.Tg.Token,
		Poller: &tele.LongPoller{Timeout: 10 * time.Second},
	}
	stopSignal := make(chan bool)

	return &telegramBot{
		botCfg:     pref,
		log:        l,
		stopSignal: stopSignal,
		Cocktails:  cocktails,
	}, nil
}

func (tgb *telegramBot) startBot() {
	tgb.log.Info("Запускаем бота\n")
	b, err := tele.NewBot(tgb.botCfg)
	if err != nil {
		tgb.log.Fatal(err)
		return
	}
	tgb.botInstance = b

	tgb.log.Info("Создаем переменные\n")
	var (
		// Main menu
		mainMenu     = &tele.ReplyMarkup{}
		btnCocktails = mainMenu.Data("Список коктейлей", "cocktails", "0")
		cocktailPage = &tele.ReplyMarkup{}
		btnCocktail  = cocktailPage.Data("", "cocktail")
	)
	mainMenu.Inline(
		mainMenu.Row(btnCocktails),
	)

	tgb.log.Info("Регистрируем команды\n")

	b.Handle("/start", func(c tele.Context) error {
		return c.Send("Вас приветствует JustShake бот, который вам поможет найти самый вкусный и классный коктель, который вы пробовали", mainMenu)
	})

	b.Handle(&btnCocktails, func(c tele.Context) error {
		parsedData, err := strconv.ParseInt(c.Update().Callback.Data, 10, 64)
		if err != nil {
			tgb.log.Error(err)
		}
		res, err := tgb.Cocktails.GetNames(context.TODO(), use_cases.GetNamesRequest{Pagination: models.Pagination{
			Page:         parsedData,
			ItemsPerPage: 10,
		}})
		if err != nil {
			tgb.log.Error(err)
		}
		var cocktailsList = &tele.ReplyMarkup{}
		for _, it := range res.Items {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: btnCocktail.Unique,
					Text:   it.Name,
					Data:   it.Id.String(),
				},
			})
		}
		if parsedData == 0 {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: "cocktails",
					Text:   "👉",
					Data:   strconv.FormatInt(parsedData+1, 10),
				}})
		} else if (res.TotalItems / 10) <= 10 {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: "cocktails",
					Text:   "👈",
					Data:   strconv.FormatInt(parsedData-1, 10),
				}})
		} else {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: "cocktails",
					Text:   "👈",
					Data:   strconv.FormatInt(parsedData-1, 10),
				},
				{
					Unique: "cocktails",
					Text:   "👉",
					Data:   strconv.FormatInt(parsedData+1, 10),
				}})
		}

		err = c.Delete()
		return c.Send("Коктели:", cocktailsList)
	})

	b.Handle("/allcocktails", func(c tele.Context) error {

		return c.Send("Все коктейли")
	})

	b.Handle(&btnCocktail, func(c tele.Context) error {
		uuid, _ := uuid.Parse(c.Update().Callback.Data)
		res, err := tgb.Cocktails.GetById(context.TODO(), use_cases.GetByIdRequest{Id: uuid})
		if err != nil {
			tgb.log.Error(err)
		}
		resultString := fmt.Sprintf("🍸<b>Коктейль:</b> %+v\n", res.RussianName)
		resultString = resultString + fmt.Sprintf("<b>Английское название:</b> %+v\n", res.Name)
		resultString = resultString + fmt.Sprintf("\n<b>Ингредиенты:</b>\n")
		for _, element := range res.CompositionElements {
			resultString = resultString + fmt.Sprintf("👉 %+v %+v%+v\n", element.Name, element.Count, element.Unit)
		}
		resultString = resultString + fmt.Sprintf("\n<b>Требуемые инструменты:</b>\n")
		for _, element := range res.Tools {
			resultString = resultString + fmt.Sprintf("👉 %+v %+v%+v\n", element.Name, element.Count, element.Unit)
		}
		resultString = resultString + fmt.Sprintf("\n<b>Способ приготовления:</b>\n")
		for i, element := range res.Recipe.Steps {
			resultString = resultString + fmt.Sprintf("%+v. %+v\n", i+1, element)
		}
		resultString = resultString + fmt.Sprintf("\n<b>История под этого коктейль:</b>\n")
		resultString = resultString + res.History
		resultString = resultString + fmt.Sprintf("\n\n<b>Теги:</b>\n")
		for _, element := range res.Tags {
			resultString = resultString + fmt.Sprintf("#%+v ", element.Name)
		}

		err = c.Delete()
		if err != nil {
			return err
		}
		return c.Send(resultString, &tele.SendOptions{ParseMode: tele.ModeHTML})
	})

	tgb.log.Info("Запускаем бота\n")
	b.Start()
}

func (tgb *telegramBot) stopBot() {
	tgb.log.Info("Завершаем работу бота\n")
	tgb.botInstance.Stop()
}

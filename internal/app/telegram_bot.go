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
	"math"
	"strconv"
	"time"
)

// telegramBot Структура с описанием бота и его функциями
type telegramBot struct {
	botCfg      tele.Settings
	log         logger.Interface
	stopSignal  chan bool
	botInstance *tele.Bot
	use_cases.Cocktails
}

// Управляющие кнопки (buttons)
var (
	// Main menu
	mainMenuPage  = &tele.ReplyMarkup{}
	cocktailPage  = &tele.ReplyMarkup{}
	selectPageBtn = tele.InlineButton{Unique: "selectpage"}
	cocktailBtn   = cocktailPage.Data("", "cocktail")
	cocktailsBtn  = mainMenuPage.Data("Список коктейлей", "cocktails", "0")
)

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
	var err error
	tgb.botInstance, err = tele.NewBot(tgb.botCfg)
	if err != nil {
		tgb.log.Fatal(err)
		return
	}

	tgb.log.Info("Регистрируем команды\n")

	tgb.botInstance.Handle("/start", func(c tele.Context) error {
		return c.Send("Вас приветствует JustShake бот, который вам поможет найти самый вкусный и классный коктель, который вы пробовали", mainMenuPage)
	})

	tgb.botInstance.Handle("/menu", func(c tele.Context) error {
		mainMenuPage.Inline(mainMenuPage.Row(cocktailsBtn))
		return c.Send("Основное меню", mainMenuPage)
	})

	tgb.botInstance.Handle(&cocktailsBtn, func(c tele.Context) error {
		parsedPage, err := strconv.ParseInt(c.Update().Callback.Data, 10, 64)
		if err != nil {
			tgb.log.Error(err)
		}
		itemsPerPage := int64(10)
		res, err := tgb.Cocktails.GetNames(context.TODO(), use_cases.GetNamesRequest{Pagination: models.Pagination{
			Page:         parsedPage,
			ItemsPerPage: itemsPerPage,
		}})
		if err != nil {
			tgb.log.Error(err)
		}
		var cocktailsList = &tele.ReplyMarkup{}
		for _, it := range res.Items {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: cocktailBtn.Unique,
					Text:   it.Name,
					Data:   it.Id.String(),
				},
			})
		}
		if parsedPage == 0 {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems), {
					Unique: cocktailsBtn.Unique,
					Text:   "👉",
					Data:   strconv.FormatInt(parsedPage+1, 10),
				}})
		} else if int64(len(res.Items)) < itemsPerPage {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: cocktailsBtn.Unique,
					Text:   "👈",
					Data:   strconv.FormatInt(parsedPage-1, 10),
				},
				getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems),
			})
		} else {
			cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
				{
					Unique: cocktailsBtn.Unique,
					Text:   "👈",
					Data:   strconv.FormatInt(parsedPage-1, 10),
				},
				getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems), {
					Unique: cocktailsBtn.Unique,
					Text:   "👉",
					Data:   strconv.FormatInt(parsedPage+1, 10),
				}})
		}

		err = c.Delete()
		return c.Send("Коктели:", cocktailsList)
	})

	tgb.botInstance.Handle(&cocktailBtn, func(c tele.Context) error {
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

	tgb.botInstance.Handle(&selectPageBtn, func(c tele.Context) error {
		parsedTotalItems, err := strconv.ParseInt(c.Update().Callback.Data, 10, 64)
		if err != nil {
			tgb.log.Error(err)
		}
		var result = &tele.ReplyMarkup{}
		var localPages []tele.InlineButton
		for i := int64(0); i < parsedTotalItems; i++ {
			if i != 0 && (i)%4 == 0 {
				result.InlineKeyboard = append(result.InlineKeyboard, localPages)
				localPages = []tele.InlineButton{}
			}
			localPages = append(localPages, tele.InlineButton{
				Unique: cocktailsBtn.Unique,
				Text:   strconv.FormatInt(i+1, 10),
				Data:   strconv.FormatInt(i, 10),
			})
		}
		if len(localPages) != 0 {
			result.InlineKeyboard = append(result.InlineKeyboard, localPages)
			localPages = []tele.InlineButton{}
		}
		err = c.Delete()
		return c.Send("Доступные страницы:", result)
	})

	tgb.log.Info("Запускаем бота\n")
	tgb.botInstance.Start()
}

func (tgb *telegramBot) stopBot() {
	tgb.log.Info("Завершаем работу бота\n")
	tgb.botInstance.Stop()
}

func getPagedInlineButton(pageNum int64, itemsPerPage int64, totalItems int64) tele.InlineButton {
	selectPageBtn.Data = fmt.Sprintf("%+v", float64(totalItems/itemsPerPage+1))
	selectPageBtn.Text = fmt.Sprintf("%+v/%+v", pageNum+1, math.Ceil(float64(totalItems/itemsPerPage))+1)
	return selectPageBtn
}

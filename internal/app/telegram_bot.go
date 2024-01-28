package app

import (
	"context"
	"fmt"
	"github.com/eko/gocache/lib/v4/cache"
	"github.com/eko/gocache/store/go_cache/v4"
	"github.com/google/uuid"
	gocache "github.com/patrickmn/go-cache"
	tele "gopkg.in/telebot.v3"
	"justshake/cocktails/config"
	"justshake/cocktails/internal/domain/models"
	"justshake/cocktails/internal/use_cases"
	"justshake/cocktails/pkg/logger"
	"math"
	"strconv"
	"strings"
	"time"
)

// telegramBot Структура с описанием бота и его функциями
type telegramBot struct {
	botCfg      tele.Settings
	log         logger.Interface
	stopSignal  chan bool
	botInstance *tele.Bot
	use_cases.Cocktails
	memoryCache *cache.Cache[[]byte]
}

type PreviousPage int

const (
	MainMenu      PreviousPage = 0
	CocktailsList              = 1
	Search                     = 2
)

const (
	WrongCommandErrorMessage string = "Я не могу распознать вашу команду. Введите /menu для перемещения в главное меню"
)

type cocktailButtonRequest struct {
	Id           uuid.UUID    `json:"i"`
	PreviousPage PreviousPage `json:"p"`
	PreviousData string       `json:"d"`
}

// Управляющие кнопки (buttons)
var (
	// Main menu
	mainMenuPage           = &tele.ReplyMarkup{}
	cocktailPage           = &tele.ReplyMarkup{}
	selectPageNumberBtn    = tele.InlineButton{Unique: "selectpage"}
	cocktailBtn            = cocktailPage.Data("", "cocktail")
	cocktailsBtn           = mainMenuPage.Data("📋 Список коктейлей", "cocktails", "0")
	searchByNameBtn        = mainMenuPage.Data("🔎 Поиск по названию", "searchbyname")
	mainMenuBtn            = mainMenuPage.Data("", "mainmenu")
	searchByNameProcessBtn = mainMenuPage.Data("", "searchbynameprocess")
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
	inMemoryCache := configureMemoryCache()

	return &telegramBot{
		botCfg:      pref,
		log:         l,
		stopSignal:  stopSignal,
		Cocktails:   cocktails,
		memoryCache: inMemoryCache,
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
	mainMenuPage.Inline(mainMenuPage.Row(cocktailsBtn), mainMenuPage.Row(searchByNameBtn))

	tgb.log.Info("Регистрируем команды\n")

	tgb.botInstance.Handle("/start", func(c tele.Context) error {
		return c.Send("Вас приветствует JustShake бот, который вам поможет найти самый вкусный и классный коктель, который вы пробовали", mainMenuPage)
	})
	tgb.botInstance.Handle("/menu", tgb.showMainMenuPage)
	tgb.botInstance.Handle(&mainMenuBtn, tgb.showMainMenuPage)

	tgb.botInstance.Handle(&cocktailsBtn, tgb.showCocktailsListPage)

	tgb.botInstance.Handle(&cocktailBtn, tgb.showCocktailPage)

	tgb.botInstance.Handle(&selectPageNumberBtn, tgb.showSelectPAgeNumberPage)

	tgb.botInstance.Handle(&searchByNameBtn, tgb.showSearchByNamePage)

	tgb.botInstance.Handle(&searchByNameProcessBtn, tgb.searchByNameResultPage)

	tgb.botInstance.Handle(tele.OnText, func(c tele.Context) error {
		cachedValue, err := tgb.memoryCache.Get(context.TODO(), strconv.FormatInt(c.Sender().ID, 10))
		if err != nil {
			tgb.log.Error(err)
			return c.Send(WrongCommandErrorMessage)
		}

		switch string(cachedValue) {
		case "searchbyname":
			return tgb.searchByNameResultPage(c)
		default:
			return c.Send(WrongCommandErrorMessage)
		}
	})

	tgb.log.Info("Запускаем бота\n")
	tgb.botInstance.Start()
}

func (tgb *telegramBot) stopBot() {
	tgb.log.Info("Завершаем работу бота\n")
	tgb.botInstance.Stop()
}

func getPagedInlineButton(pageNum int64, itemsPerPage int64, totalItems int64) tele.InlineButton {
	selectPageNumberBtn.Data = fmt.Sprintf("%+v", float64(totalItems/itemsPerPage+1))
	selectPageNumberBtn.Text = fmt.Sprintf("%+v/%+v", pageNum+1, math.Ceil(float64(totalItems/itemsPerPage))+1)
	return selectPageNumberBtn
}

func configureMemoryCache() *cache.Cache[[]byte] {
	gocacheClient := gocache.New(10*time.Minute, 20*time.Minute)
	gocacheStore := go_cache.NewGoCache(gocacheClient)
	cacheManager := cache.New[[]byte](gocacheStore)
	return cacheManager
}

func (tgb *telegramBot) showMainMenuPage(c tele.Context) error {
	return c.EditOrSend("Основное меню:", mainMenuPage)
}

func (tgb *telegramBot) showCocktailsListPage(c tele.Context) error {
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
		prepareData := cocktailButtonRequest{
			Id:           it.Id,
			PreviousPage: CocktailsList,
			PreviousData: strconv.FormatInt(parsedPage, 10),
		}
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			{
				Unique: cocktailBtn.Unique,
				Text:   it.Name,
				Data:   fmt.Sprintf("%+v %+v %+v", prepareData.Id, prepareData.PreviousPage, prepareData.PreviousData),
			},
		})
	}
	nextButton := tele.InlineButton{
		Unique: cocktailsBtn.Unique,
		Text:   "👉",
		Data:   strconv.FormatInt(parsedPage+1, 10),
	}
	prevButton := tele.InlineButton{
		Unique: cocktailsBtn.Unique,
		Text:   "👈",
		Data:   strconv.FormatInt(parsedPage-1, 10),
	}
	if parsedPage == 0 {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems),
			nextButton})
	} else if int64(len(res.Items)) < itemsPerPage {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			prevButton,
			getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems),
		})
	} else {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			prevButton,
			getPagedInlineButton(parsedPage, itemsPerPage, res.TotalItems),
			nextButton})
	}
	cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{{Text: "👈 Назад", Unique: mainMenuBtn.Unique}})

	return c.EditOrSend("Коктели:", cocktailsList)
}

func (tgb *telegramBot) showCocktailPage(c tele.Context) error {
	var request cocktailButtonRequest
	data := strings.Split(c.Update().Callback.Data, " ")
	id, _ := uuid.Parse(data[0])
	request.Id = id
	prevPage, _ := strconv.Atoi(data[1])
	request.PreviousPage = PreviousPage(prevPage)
	if len(data) == 3 {
		request.PreviousData = data[2]
	}

	res, err := tgb.Cocktails.GetById(context.TODO(), use_cases.GetByIdRequest{Id: request.Id})
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

	inlineButtons := &tele.ReplyMarkup{}
	var buttonName string
	if request.PreviousPage == CocktailsList {
		buttonName = cocktailsBtn.Unique
	} else if request.PreviousPage == MainMenu {
		buttonName = mainMenuBtn.Unique
	} else if request.PreviousPage == Search {
		buttonName = searchByNameProcessBtn.Unique
	}
	returnBtn := inlineButtons.Data("👈 Назад", buttonName, request.PreviousData)
	inlineButtons.Inline(tele.Row{returnBtn})
	return c.EditOrSend(resultString, &tele.SendOptions{ParseMode: tele.ModeHTML}, inlineButtons)
}

func (tgb *telegramBot) showSelectPAgeNumberPage(c tele.Context) error {
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

	return c.EditOrSend("Доступные страницы:", result)
}

func (tgb *telegramBot) showSearchByNamePage(c tele.Context) error {
	err := tgb.memoryCache.Set(context.TODO(), strconv.FormatInt(c.Update().Callback.Sender.ID, 10), []byte(searchByNameBtn.Unique))
	if err != nil {
		return err
	}
	return c.EditOrSend("Введите часть названия или полное название для поиска", &tele.ReplyMarkup{
		Placeholder: "Параметры поиска",
	})
}

func (tgb *telegramBot) searchByNameResultPage(c tele.Context) error {
	var searchText string
	if c.Data() != "" {
		searchText = c.Data()
	} else if c.Text() != "" {
		searchText = c.Text()
	}
	res, err := tgb.Cocktails.GetByFilter(context.TODO(), use_cases.GetByFilterRequest{
		RussianNames: []string{searchText},
		Names:        []string{searchText},
		Pagination: models.Pagination{
			Page:         0,
			ItemsPerPage: 100,
		},
	})
	if err != nil {
		tgb.log.Error(err)
		return c.Send(WrongCommandErrorMessage)
	}
	var cocktailsList = &tele.ReplyMarkup{}
	for _, it := range res.Items {
		prepareData := cocktailButtonRequest{
			Id:           it.Id,
			PreviousPage: Search,
			PreviousData: searchText,
		}
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			{
				Unique: cocktailBtn.Unique,
				Text:   it.RussianName,
				Data:   fmt.Sprintf("%+v %+v %+v", prepareData.Id, prepareData.PreviousPage, prepareData.PreviousData),
			},
		})
	}
	cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
		{
			Unique: mainMenuBtn.Unique,
			Text:   "👈 Назад",
		},
	})

	return c.EditOrSend("Найденные коктели:", cocktailsList)
}

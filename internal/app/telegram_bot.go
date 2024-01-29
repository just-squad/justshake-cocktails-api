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
	"justshake/cocktails/internal/use_cases/cocktails"
	"justshake/cocktails/internal/use_cases/users"
	"justshake/cocktails/pkg/logger"
	"math"
	"slices"
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
	cocktails.Cocktails
	users.Users
	memoryCache *cache.Cache[[]byte]
}

type PreviousPage int

const (
	MainMenu              PreviousPage = 0
	CocktailsList                      = 1
	FavoriteCocktailsList              = 2
	Search                             = 3
)

const (
	cocktailItemsPerPage     int64  = 10
	backButtonText           string = "👈 Назад"
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
	mainMenuPage             = &tele.ReplyMarkup{}
	cocktailPage             = &tele.ReplyMarkup{}
	profilePage              = &tele.ReplyMarkup{}
	selectPageNumberBtn      = tele.InlineButton{Unique: "selectpage"}
	cocktailBtn              = cocktailPage.Data("", "cocktail")
	cocktailsBtn             = mainMenuPage.Data("📋 Список коктейлей", "cocktails", "0")
	searchByNameBtn          = mainMenuPage.Data("🔎 Поиск по названию", "searchbyname")
	mainMenuBtn              = mainMenuPage.Data("", "mainmenu")
	searchByNameProcessBtn   = mainMenuPage.Data("", "searchbynameprocess")
	profileMenuBtn           = profilePage.Data("🗄 Личная страница", "profile")
	registerUserRequestBtn   = profilePage.Data("🔑 Регистрация", "registerrequest")
	registerUserConfirmedBtn = profilePage.Data("Подтвердить регистрацию", "registerconfirmed")
	removeUserRequestBtn     = profilePage.Data("🗑 Удалить учетную запись", "deleteuserprofile")
	removeUserConfirmedBtn   = profilePage.Data("Подтвердить удаление", "deleteuserprofileconfirmed")
	addToFavoriteBtn         = profilePage.Data("🤍", "addtofavorite")
	removeFromFavoriteBtn    = profilePage.Data("❤️", "removefromfavorite")
	favoriteCocktailsBtn     = profilePage.Data("❤ Показать избранное", "showfavoritecocktails")
)

func newBot(
	cfg *config.Config,
	cocktails *cocktails.UseCase,
	users *users.UseCase,
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
		Users:       users,
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

	tgb.log.Info("Регистрируем команды\n")

	tgb.botInstance.Handle("/start", func(c tele.Context) error {
		return c.Send("Вас приветствует JustShake бот, который вам поможет найти самый вкусный и классный коктель, который вы пробовали", mainMenuPage)
	})
	tgb.botInstance.Handle("/menu", tgb.showMainMenuPage)
	tgb.botInstance.Handle(&mainMenuBtn, tgb.showMainMenuPage)
	tgb.botInstance.Handle(&cocktailsBtn, tgb.showCocktailsListPage)
	tgb.botInstance.Handle(&cocktailBtn, tgb.showCocktailPage)
	tgb.botInstance.Handle(&selectPageNumberBtn, tgb.showSelectPageNumberPage)
	tgb.botInstance.Handle(&searchByNameBtn, tgb.showSearchByNamePage)
	tgb.botInstance.Handle(&searchByNameProcessBtn, tgb.searchByNameResultPage)
	tgb.botInstance.Handle(&registerUserRequestBtn, tgb.showRegisterUserRequestPage)
	tgb.botInstance.Handle(&registerUserConfirmedBtn, tgb.doRegisterUser)
	tgb.botInstance.Handle(&removeUserRequestBtn, tgb.showRemoveUserRequestPage)
	tgb.botInstance.Handle(&removeUserConfirmedBtn, tgb.doRemoveUser)
	tgb.botInstance.Handle(&addToFavoriteBtn, tgb.addCocktailToFavorite)
	tgb.botInstance.Handle(&removeFromFavoriteBtn, tgb.removeCocktailFromFavorite)
	tgb.botInstance.Handle(&profileMenuBtn, tgb.showProfilePage)
	tgb.botInstance.Handle(&favoriteCocktailsBtn, tgb.showFavoriteCocktailsPage)
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

func getPagedInlineButton(pageNum int64, itemsPerPage int64, totalItems int64, previousPage string) tele.InlineButton {
	selectPageNumberBtn.Data = fmt.Sprintf("%+v %+v", float64(totalItems/itemsPerPage+1), previousPage)
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
	res, err := tgb.Users.IsExist(context.TODO(), users.GetByTelegramIdRequest{Id: c.Sender().ID})
	if err != nil && err.Error() != "mongo: no documents in result" {
		tgb.log.Error(err)
		return err
	}
	var accountBtn tele.Btn
	if !res {
		accountBtn = registerUserRequestBtn
	} else {
		accountBtn = profileMenuBtn
	}
	mainMenuPage.Inline(mainMenuPage.Row(cocktailsBtn), mainMenuPage.Row(searchByNameBtn), mainMenuPage.Row(accountBtn))
	return c.EditOrSend("Основное меню:", mainMenuPage)
}

func (tgb *telegramBot) showCocktailsListPage(c tele.Context) error {
	pageNumber, err := strconv.ParseInt(c.Update().Callback.Data, 10, 64)
	if err != nil {
		tgb.log.Error(err)
	}
	res, err := tgb.Cocktails.GetNames(context.TODO(), cocktails.GetNamesRequest{Pagination: models.Pagination{
		Page:         pageNumber,
		ItemsPerPage: cocktailItemsPerPage,
	}})
	if err != nil {
		tgb.log.Error(err)
	}
	cocktailsList, _ := tgb.prepareCocktailsListReplyMarkup(pageNumber,
		res,
		cocktailsBtn.Unique,
		CocktailsList,
		mainMenuBtn.Unique)

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

	res, err := tgb.Cocktails.GetById(context.TODO(), cocktails.GetByIdRequest{Id: request.Id})
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
	} else if request.PreviousPage == FavoriteCocktailsList {
		buttonName = favoriteCocktailsBtn.Unique
	} else if request.PreviousPage == MainMenu {
		buttonName = mainMenuBtn.Unique
	} else if request.PreviousPage == Search {
		buttonName = searchByNameProcessBtn.Unique
	}
	inlineButtonsRow := tele.Row{}
	inlineButtonsRow = append(inlineButtonsRow, inlineButtons.Data(backButtonText, buttonName, request.PreviousData))
	userExist, err := tgb.Users.IsExist(context.TODO(), users.GetByTelegramIdRequest{Id: c.Sender().ID})
	if err != nil {
		tgb.log.Error(err)
		userExist = false
	}
	if userExist {
		user, err := tgb.Users.GetByTelegramId(context.TODO(), users.GetByTelegramIdRequest{Id: c.Sender().ID})
		if err != nil {
			tgb.log.Error(err)
			return nil
		}
		if slices.Contains(user.FavoriteCocktails, id) {
			removeFromFavoriteBtn.Data = c.Update().Callback.Data
			inlineButtonsRow = append(inlineButtonsRow, removeFromFavoriteBtn)
		} else {
			addToFavoriteBtn.Data = c.Update().Callback.Data
			inlineButtonsRow = append(inlineButtonsRow, addToFavoriteBtn)
		}
	}
	inlineButtons.Inline(inlineButtonsRow)
	return c.EditOrSend(resultString, &tele.SendOptions{ParseMode: tele.ModeHTML}, inlineButtons)
}

func (tgb *telegramBot) showSelectPageNumberPage(c tele.Context) error {
	data := strings.Split(c.Update().Callback.Data, " ")
	parsedTotalItems, err := strconv.ParseInt(data[0], 10, 64)
	prevPage := data[1]

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
			Unique: prevPage,
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

func (tgb *telegramBot) showProfilePage(c tele.Context) error {
	returnBtn := tele.Btn{
		Unique: mainMenuBtn.Unique,
		Text:   backButtonText,
	}
	profilePage.Inline(profilePage.Row(favoriteCocktailsBtn), profilePage.Row(removeUserRequestBtn), profilePage.Row(returnBtn))
	return c.EditOrSend("Личный кабинет:", profilePage)
}

func (tgb *telegramBot) showRegisterUserRequestPage(c tele.Context) error {
	resultStr := fmt.Sprintf("Подтверждая регистрацию, вы соглашаетесь на то, что мы сохраняем ваш идентификатор пользователя Telegram. Другую информацию мы не собираем.\n\n")
	resultStr = resultStr + fmt.Sprintf("У вас появляется возможность сохранять любимые коктейли в свою личную подборку, чтобы проще было их искать.\n\n")
	resultStr = resultStr + fmt.Sprintf("В любой момент вы можете полностью удалить свой профиль.\n")
	resultStr = resultStr + fmt.Sprintf("Приятного использования ☺️")
	resultMarkup := &tele.ReplyMarkup{}
	returnBtn := tele.Btn{
		Unique: mainMenuBtn.Unique,
		Text:   backButtonText,
	}
	resultMarkup.Inline(resultMarkup.Row(registerUserConfirmedBtn), resultMarkup.Row(returnBtn))
	return c.EditOrSend(resultStr, &tele.SendOptions{ParseMode: tele.ModeHTML}, resultMarkup)
}

func (tgb *telegramBot) doRegisterUser(c tele.Context) error {
	err := tgb.Users.Create(context.TODO(), users.CreateUserRequest{
		Id:         uuid.Nil,
		TelegramId: c.Sender().ID,
	})
	if err != nil {
		tgb.log.Error(err)
		return err
	}

	alertResp := &tele.CallbackResponse{ShowAlert: true, Text: "Вы успешно зарегистрированы"}
	err = c.Respond(alertResp)
	if err != nil {
		tgb.log.Error(err)
		return err
	}
	return tgb.showMainMenuPage(c)
}

func (tgb *telegramBot) showRemoveUserRequestPage(c tele.Context) error {
	resultStr := fmt.Sprintf("Вы точно хотите удалить свой профиль?.\n\n")
	resultStr = resultStr + fmt.Sprintf("Все избранные коктейли будут удалены. 😔\n")
	resultMarkup := &tele.ReplyMarkup{}
	returnBtn := tele.Btn{
		Unique: profileMenuBtn.Unique,
		Text:   backButtonText,
	}
	resultMarkup.Inline(resultMarkup.Row(removeUserConfirmedBtn), resultMarkup.Row(returnBtn))
	return c.EditOrSend(resultStr, &tele.SendOptions{ParseMode: tele.ModeHTML}, resultMarkup)
}

func (tgb *telegramBot) doRemoveUser(c tele.Context) error {
	err := tgb.Users.Delete(context.TODO(), users.DeleteUserRequest{
		TelegramId: c.Sender().ID,
	})
	if err != nil {
		tgb.log.Error(err)
		return err
	}

	alertResp := &tele.CallbackResponse{ShowAlert: true, Text: "Ваш профиль был успешно удален"}
	err = c.Respond(alertResp)
	if err != nil {
		tgb.log.Error(err)
		return err
	}
	return tgb.showMainMenuPage(c)
}

func (tgb *telegramBot) addCocktailToFavorite(c tele.Context) error {
	var request cocktailButtonRequest
	data := strings.Split(c.Update().Callback.Data, " ")
	id, _ := uuid.Parse(data[0])
	request.Id = id

	err := tgb.Users.AddCocktailToFavorite(context.TODO(), users.FavoriteCocktailRequest{
		CocktailId: request.Id,
		TelegramId: c.Sender().ID,
	})
	if err != nil {
		tgb.log.Error(err)
		return err
	}

	return tgb.showCocktailPage(c)
}

func (tgb *telegramBot) removeCocktailFromFavorite(c tele.Context) error {
	var request cocktailButtonRequest
	data := strings.Split(c.Update().Callback.Data, " ")
	id, _ := uuid.Parse(data[0])
	request.Id = id

	err := tgb.Users.RemoveCocktailFromFavorite(context.TODO(), users.FavoriteCocktailRequest{
		CocktailId: request.Id,
		TelegramId: c.Sender().ID,
	})
	if err != nil {
		tgb.log.Error(err)
		return err
	}

	return tgb.showCocktailPage(c)
}

func (tgb *telegramBot) showFavoriteCocktailsPage(c tele.Context) error {
	pageNumber, err := strconv.ParseInt(c.Update().Callback.Data, 10, 64)
	if err != nil {
		tgb.log.Error(err)
	}
	ctx := context.TODO()
	userInfo, err := tgb.Users.GetByTelegramId(ctx, users.GetByTelegramIdRequest{Id: c.Sender().ID})
	if err != nil {
		tgb.log.Error(err)
		return err
	}
	res, err := tgb.Cocktails.GetNames(ctx, cocktails.GetNamesRequest{
		Ids: userInfo.FavoriteCocktails,
		Pagination: models.Pagination{
			Page:         pageNumber,
			ItemsPerPage: cocktailItemsPerPage,
		}})
	if err != nil {
		tgb.log.Error(err)
		return err
	}
	cocktailsList, _ := tgb.prepareCocktailsListReplyMarkup(pageNumber,
		res,
		favoriteCocktailsBtn.Unique,
		FavoriteCocktailsList,
		profileMenuBtn.Unique)

	return c.EditOrSend("Любимые коктели:", cocktailsList)
}

func (tgb *telegramBot) searchByNameResultPage(c tele.Context) error {
	var searchText string
	if c.Data() != "" {
		searchText = c.Data()
	} else if c.Text() != "" {
		searchText = c.Text()
	}
	res, err := tgb.Cocktails.GetByFilter(context.TODO(), cocktails.GetByFilterRequest{
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
			Text:   backButtonText,
		},
	})

	return c.EditOrSend("Найденные коктели:", cocktailsList)
}

func (tgb *telegramBot) prepareCocktailsListReplyMarkup(pageNumber int64,
	cocktailNames cocktails.GetNamesResponse,
	cocktailsListBtnName string,
	previousPageAfterCocktail PreviousPage,
	previousPage string) (*tele.ReplyMarkup, error) {
	var cocktailsList = &tele.ReplyMarkup{}
	for _, it := range cocktailNames.Items {
		prepareData := cocktailButtonRequest{
			Id:           it.Id,
			PreviousPage: previousPageAfterCocktail,
			PreviousData: strconv.FormatInt(pageNumber, 10),
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
		Unique: cocktailsListBtnName,
		Text:   "👉",
		Data:   strconv.FormatInt(pageNumber+1, 10),
	}
	prevButton := tele.InlineButton{
		Unique: cocktailsListBtnName,
		Text:   "👈",
		Data:   strconv.FormatInt(pageNumber-1, 10),
	}

	pagedInlineButton := getPagedInlineButton(pageNumber, cocktailItemsPerPage, cocktailNames.TotalItems, cocktailsListBtnName)
	if pageNumber == 0 {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			pagedInlineButton,
			nextButton})
	} else if int64(len(cocktailNames.Items)) < cocktailItemsPerPage {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			prevButton,
			pagedInlineButton,
		})
	} else {
		cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{
			prevButton,
			pagedInlineButton,
			nextButton})
	}
	cocktailsList.InlineKeyboard = append(cocktailsList.InlineKeyboard, []tele.InlineButton{{Text: backButtonText, Unique: previousPage}})

	return cocktailsList, nil
}

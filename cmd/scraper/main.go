package main

import (
	// import Colly
	"fmt"
	"github.com/gocolly/colly"
	"justshake/cocktails/config"
	cocktail_aggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/internal/infrastructure/repositories"
	"justshake/cocktails/pkg/logger"
	"justshake/cocktails/pkg/mng"
	"net/http"
	"os"
	"strconv"
	"strings"
)

// var scrapingUrl = "https://ru.inshaker.com"

func main() {
	// Тестирование парсинга локальных файлов
	dir, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	basePath := "file://" + dir + "/cmd/scraper/html-example/"
	fmt.Println(basePath)

	t := &http.Transport{}
	t.RegisterProtocol("file", http.NewFileTransport(http.Dir("/")))

	c := colly.NewCollector(
		// Cache responses to prevent multiple download of pages
		// even if the collector is restarted
		colly.CacheDir("./cmd/scraper/.inshaker-cache"),
	)
	// set a valid User-Agent header
	c.UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"
	// Для указания транспорта
	c.WithTransport(t)

	// Create another collector to scrape cocktail details
	detailCollector := c.Clone()

	var cocktails []cocktail_aggregate.Cocktail
	c.OnHTML("div .cocktail-item", func(e *colly.HTMLElement) {
		url := e.ChildAttrs(".cocktail-item-preview", "href")[0]

		cock := cocktail_aggregate.Cocktail{}
		cock.Url = url

		if url == "/cocktails/368-dzhin-tonik" {
			detailCollector.OnHTML("h1.common-name", func(element *colly.HTMLElement) {
				cock.RussianName = strings.TrimSpace(element.Text)
			})
			detailCollector.OnHTML("div.name-en", func(element *colly.HTMLElement) {
				cock.Name = strings.TrimSpace(element.Text)
			})
			detailCollector.OnHTML("ul.tags", func(element *colly.HTMLElement) {
				element.ForEach("li", func(i int, element *colly.HTMLElement) {
					cock.Tags = append(cock.Tags, cocktail_aggregate.Tag{Name: strings.TrimSpace(element.Text)})
				})
			})
			detailCollector.OnHTML("div.common-content-plain.common-wave.common-content", func(element *colly.HTMLElement) {
				cock.History = strings.TrimSpace(element.ChildText("p"))
			})
			detailCollector.OnHTML("div.ingredient-tables", func(element *colly.HTMLElement) {
				element.ForEach("table", func(i int, element *colly.HTMLElement) {
					if i == 0 {
						element.ForEach("tr", func(i int, element *colly.HTMLElement) {
							if i == 0 {
								return
							}
							am, err := strconv.Atoi(element.ChildText("td.amount"))
							if err != nil {
								// ... handle error
								fmt.Printf("%+v", err)
							}
							cock.CompositionElements = append(cock.CompositionElements, cocktail_aggregate.CocktailItem{
								Name:  strings.TrimSpace(element.ChildText("td.name")),
								Count: am,
								Unit:  strings.TrimSpace(element.ChildText("td.unit")),
							})
						})
					} else {
						element.ForEach("tr", func(i int, element *colly.HTMLElement) {
							if i == 0 {
								return
							}
							am, err := strconv.Atoi(element.ChildText("td.amount"))
							if err != nil {
								// ... handle error
								fmt.Printf("%+v", err)
							}
							cock.Tools = append(cock.Tools, cocktail_aggregate.CocktailItem{
								Name:  strings.TrimSpace(element.ChildText("td.name")),
								Count: am,
								Unit:  strings.TrimSpace(element.ChildText("td.unit")),
							})
						})
					}
				})
			})
			detailCollector.OnHTML("div.common-content-plain.recipe > ul.steps", func(element *colly.HTMLElement) {
				element.ForEach("li", func(i int, element *colly.HTMLElement) {
					cock.Recipe.Steps = append(cock.Recipe.Steps, strings.TrimSpace(element.Text))
				})
			})
			detailCollector.OnRequest(func(r *colly.Request) {
				fmt.Println("Visiting", r.URL.String())
			})

			err := detailCollector.Visit(basePath + cock.Url + ".html")
			//err := detailCollector.Visit(scrapingUrl + cock.Url)
			if err != nil {
				fmt.Printf("Не найдена страница %+v \n", cock.Url)
				return
			}
		}

		cocktails = append(cocktails, cock)
	})

	// Before making a request print "Visiting ..."
	c.OnRequest(func(r *colly.Request) {
		fmt.Println("Visiting", r.URL.String())
	})

	// scraping logic...
	err = c.Visit(basePath + "base-cocktails.html")
	//err := c.Visit(scrapingUrl)
	if err != nil {
		return
	}
	c.Wait()

	cfg, err := config.NewConfig()
	l := logger.New(cfg.Log.Level)
	mongo, err := mng.New(cfg.Mongo, l)
	if err != nil {
		l.Fatal(fmt.Errorf("app - Run - mng.New: %w", err))
	}
	defer mongo.Close()
	repo := repositories.New(mongo, l)

	for _, c2 := range cocktails {
		err := repo.Create(c2)
		if err != nil {
			fmt.Printf("Err %+v", err)
		}
	}
}

package v1

import (
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/models"
	"justshake/cocktails/internal/use_cases"
	"net/http"

	"github.com/gin-gonic/gin"

	"justshake/cocktails/pkg/logger"
)

type cocktailsRoutes struct {
	c use_cases.Cocktails
	l logger.Interface
}

func newCocktailsRoutes(handler *gin.RouterGroup, t use_cases.Cocktails, l logger.Interface) {
	r := &cocktailsRoutes{t, l}

	h := handler.Group("/cocktails")
	{
		h.POST("/get-by-id", r.getById)
		h.POST("/get-by-filter", r.getByFilter)
	}
}

type (
	getByIdApiRequest struct {
		Id uuid.UUID `json:"id" binding:"required" example:"836e6133-6683-4cf9-b7e2-ef8cb4bf44a7"`
	}
	cocktailApiResponse struct {
		Id                  uuid.UUID                 `json:"id"`
		Name                string                    `json:"name"`
		RussianName         string                    `json:"russian_name"`
		CountryOfOrigin     string                    `json:"country_of_origin"`
		History             string                    `json:"history"`
		Tags                []tagApiResponse          `json:"tags"`
		Tools               []cocktailItemApiResponse `json:"tools"`
		CompositionElements []cocktailItemApiResponse `json:"composition_elements"`
		Recipe              recipeApiResponse         `json:"recipe"`
	}
	tagApiResponse struct {
		Name string `json:"name"`
	}
	cocktailItemApiResponse struct {
		Name  string `json:"name"`
		Count int    `json:"count"`
		Unit  string `json:"unit"`
	}
	recipeApiResponse struct {
		Steps []string `json:"steps"`
	}
	getByFilterApiRequest struct {
		Ids        []uuid.UUID `json:"ids"`
		Names      []string    `json:"names"`
		Pagination pagination  `json:"pagination"`
	}
	pagination struct {
		Page         int64 `json:"page"`
		ItemsPerPage int64 `json:"items_per_page"`
	}
	getByFilterApiResponse struct {
		Items      []cocktailResponseItem `json:"items"`
		TotalItems int64                  `json:"total-items"`
	}
	cocktailResponseItem struct {
		Id              uuid.UUID        `json:"id"`
		Name            string           `json:"name"`
		RussianName     string           `json:"russian_name"`
		CountryOfOrigin string           `json:"country_of_origin"`
		History         string           `json:"history"`
		Tags            []tagApiResponse `json:"tags"`
	}
)

// @Summary     Получение информации о коктейле по идентификатору
// @Description Получить информацию о коктейле
// @ID          get-by-id
// @Tags  	    cocktails
// @Accept      json
// @Produce     json
// @Param       request body getByIdApiRequest true "Параметр содержащий идентификатор коктейля"
// @Success     200 {object} cocktailApiResponse
// @Failure     500 {object} response
// @Router      /v1/cocktails/get-by-id [post]
func (r *cocktailsRoutes) getById(c *gin.Context) {
	var request getByIdApiRequest
	if err := c.ShouldBindJSON(&request); err != nil {
		r.l.Error(err, "http - v1 - getById")
		errorResponse(c, http.StatusBadRequest, "invalid request body")

		return
	}
	cocktail, err := r.c.GetById(c.Request.Context(), use_cases.GetByIdRequest{Id: request.Id})
	if err != nil {
		r.l.Error(err, "http - v1 - history")
		errorResponse(c, http.StatusInternalServerError, "database problems")

		return
	}
	r.l.Debug(cocktail.Id)

	c.JSON(http.StatusOK, cocktailApiResponse{
		Id:                  cocktail.Id,
		Name:                cocktail.Name,
		RussianName:         cocktail.RussianName,
		CountryOfOrigin:     cocktail.CountryOfOrigin,
		History:             cocktail.History,
		Tags:                mapToTagApiResponse(cocktail.Tags),
		Tools:               mapToCocktailItemApiResponse(cocktail.Tools),
		CompositionElements: mapToCocktailItemApiResponse(cocktail.CompositionElements),
		Recipe:              mapToRecipeApiResponse(cocktail.Recipe),
	})
}

// @Summary     Получение списка коктейлей по фильтру
// @Description Получение списка коктейлей по фильтру
// @ID          getByFilter
// @Tags  	    cocktails
// @Accept      json
// @Produce     json
// @Param       request body getByFilterApiRequest true "Фильтр для получения списка коктейлей"
// @Success     200 {object} getByFilterApiResponse
// @Failure     400 {object} response
// @Failure     500 {object} response
// @Router      /v1/cocktails/get-by-filter [post]
func (r *cocktailsRoutes) getByFilter(c *gin.Context) {
	var request getByFilterApiRequest
	if err := c.ShouldBindJSON(&request); err != nil {
		r.l.Error(err, "http - v1 - getByFilter")
		errorResponse(c, http.StatusBadRequest, "invalid request body")

		return
	}

	cocktails, err := r.c.GetByFilter(
		c.Request.Context(),
		use_cases.GetByFilterRequest{Ids: request.Ids,
			Names: request.Names,
			Pagination: models.Pagination{
				Page:         request.Pagination.Page,
				ItemsPerPage: request.Pagination.ItemsPerPage,
			}},
	)
	if err != nil {
		r.l.Error(err, "http - v1 - doTranslate")
		errorResponse(c, http.StatusInternalServerError, "translation service problems")

		return
	}

	c.JSON(http.StatusOK, getByFilterApiResponse{
		Items:      mapToCocktailResponseItem(cocktails.Items),
		TotalItems: cocktails.TotalItems,
	})
}

// Package docs Code generated by swaggo/swag. DO NOT EDIT
package docs

import "github.com/swaggo/swag"

const docTemplate = `{
    "schemes": {{ marshal .Schemes }},
    "swagger": "2.0",
    "info": {
        "description": "{{escape .Description}}",
        "title": "{{.Title}}",
        "contact": {},
        "version": "{{.Version}}"
    },
    "host": "{{.Host}}",
    "basePath": "{{.BasePath}}",
    "paths": {
        "/v1/cocktails/get-by-filter": {
            "post": {
                "description": "Получение списка коктейлей по фильтру",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "cocktails"
                ],
                "summary": "Получение списка коктейлей по фильтру",
                "operationId": "getByFilter",
                "parameters": [
                    {
                        "description": "Фильтр для получения списка коктейлей",
                        "name": "request",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/v1.getByFilterApiRequest"
                        }
                    }
                ],
                "responses": {
                    "200": {
                        "description": "OK",
                        "schema": {
                            "$ref": "#/definitions/v1.getByFilterApiResponse"
                        }
                    },
                    "400": {
                        "description": "Bad Request",
                        "schema": {
                            "$ref": "#/definitions/v1.response"
                        }
                    },
                    "500": {
                        "description": "Internal Server Error",
                        "schema": {
                            "$ref": "#/definitions/v1.response"
                        }
                    }
                }
            }
        },
        "/v1/cocktails/get-by-id": {
            "post": {
                "description": "Получить информацию о коктейле",
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "tags": [
                    "cocktails"
                ],
                "summary": "Получение информации о коктейле по идентификатору",
                "operationId": "get-by-id",
                "parameters": [
                    {
                        "description": "Параметр содержащий идентификатор коктейля",
                        "name": "request",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/v1.getByIdApiRequest"
                        }
                    }
                ],
                "responses": {
                    "200": {
                        "description": "OK",
                        "schema": {
                            "$ref": "#/definitions/v1.cocktailApiResponse"
                        }
                    },
                    "500": {
                        "description": "Internal Server Error",
                        "schema": {
                            "$ref": "#/definitions/v1.response"
                        }
                    }
                }
            }
        }
    },
    "definitions": {
        "v1.cocktailApiResponse": {
            "type": "object",
            "properties": {
                "composition_elements": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/v1.cocktailItemApiResponse"
                    }
                },
                "country_of_origin": {
                    "type": "string"
                },
                "history": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "name": {
                    "type": "string"
                },
                "recipe": {
                    "$ref": "#/definitions/v1.recipeApiResponse"
                },
                "russian_name": {
                    "type": "string"
                },
                "tags": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/v1.tagApiResponse"
                    }
                },
                "tools": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/v1.cocktailItemApiResponse"
                    }
                }
            }
        },
        "v1.cocktailItemApiResponse": {
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer"
                },
                "name": {
                    "type": "string"
                },
                "unit": {
                    "type": "string"
                }
            }
        },
        "v1.cocktailResponseItem": {
            "type": "object",
            "properties": {
                "country_of_origin": {
                    "type": "string"
                },
                "history": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "name": {
                    "type": "string"
                },
                "russian_name": {
                    "type": "string"
                },
                "tags": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/v1.tagApiResponse"
                    }
                }
            }
        },
        "v1.getByFilterApiRequest": {
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "names": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "pagination": {
                    "$ref": "#/definitions/v1.pagination"
                }
            }
        },
        "v1.getByFilterApiResponse": {
            "type": "object",
            "properties": {
                "items": {
                    "type": "array",
                    "items": {
                        "$ref": "#/definitions/v1.cocktailResponseItem"
                    }
                },
                "total-items": {
                    "type": "integer"
                }
            }
        },
        "v1.getByIdApiRequest": {
            "type": "object",
            "required": [
                "id"
            ],
            "properties": {
                "id": {
                    "type": "string",
                    "example": "836e6133-6683-4cf9-b7e2-ef8cb4bf44a7"
                }
            }
        },
        "v1.pagination": {
            "type": "object",
            "properties": {
                "items_per_page": {
                    "type": "integer"
                },
                "page": {
                    "type": "integer"
                }
            }
        },
        "v1.recipeApiResponse": {
            "type": "object",
            "properties": {
                "steps": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "v1.response": {
            "type": "object",
            "properties": {
                "error": {
                    "type": "string",
                    "example": "message"
                }
            }
        },
        "v1.tagApiResponse": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            }
        }
    }
}`

// SwaggerInfo holds exported Swagger Info so clients can modify it
var SwaggerInfo = &swag.Spec{
	Version:          "",
	Host:             "",
	BasePath:         "",
	Schemes:          []string{},
	Title:            "",
	Description:      "",
	InfoInstanceName: "swagger",
	SwaggerTemplate:  docTemplate,
	LeftDelim:        "{{",
	RightDelim:       "}}",
}

func init() {
	swag.Register(SwaggerInfo.InstanceName(), SwaggerInfo)
}

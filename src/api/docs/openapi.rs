use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn openapi_json() -> impl IntoResponse {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    Json(json!({
        "openapi": "3.1.1",

        "info": {
            "title": "osu-mirror-rs Documentation",
            "version": VERSION,
            "description": "This is the API documentation for the osu-mirror-rs an osu! beatmap mirror.",
        },

        "tags": [
            {
                "name": "System",
                "description": "Server health and system endpoints"
            },
            {
                "name": "osu!v1 api",
                "description": "osu!api v1 compatible endpoints"
            },
            {
                "name": "osu!v2 api",
                "description": "osu!api v2 compatible endpoints"
            },
        ],

        "paths": {
            "/health": {
                "get": {
                    "tags": ["System"],
                    "summary": "Health check",
                    "responses": {
                        "200": { "description": "Service is alive" }
                    }
                }
            },
            "/status": {
                "get": {
                    "tags": ["System"],
                    "summary": "Server status",
                    "responses": {
                        "200": {
                            "description": "Status information",
                            "content": {
                                "application/json": {
                                    "schema": { "type": "object" }
                                }
                            }
                        }
                    }
                }
            },

            "/v1/search": {
                "get": {
                    "tags": ["osu!v1 api"],
                    "summary": "Search beatmapsets",
                    "parameters": [
                        {
                            "name": "q",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string" },
                            "description": "Search query (artist, title, creator, md5, etc)"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Search results",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": { "type": "object" }
                                    }
                                }
                            }
                        }
                    }
                }
            },


            "/v1/beatmapsets/{id}": {
                "get": {
                    "tags": ["osu!v1 api"],
                    "summary": "Get beatmapset",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Beatmapset found",
                            "content": {
                                "application/json": {
                                    "schema": { "type": "array", "items": { "type": "object" } }
                                }
                            }
                        },
                        "404": { "description": "Not found" }
                    }
                }
            },

            "/v1/beatmaps/{id}": {
                "get": {
                    "tags": ["osu!v1 api"],
                    "summary": "Get beatmap",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Beatmap found",
                            "content": {
                                "application/json": {
                                    "schema": { "type": "array", "items": { "type": "object" } }
                                }
                            }
                        },
                        "404": { "description": "Not found" }
                    }
                }
            },

            "/v1/beatmaps/md5/{md5}": {
                "get": {
                    "tags": ["osu!v1 api"],
                    "summary": "Get beatmap by MD5 hash",
                    "parameters": [
                        {
                            "name": "md5",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Beatmap found",
                            "content": {
                                "application/json": { "schema": { "type": "array", "items": { "type": "object" } } }
                            }
                        },
                        "404": { "description": "Not found" }
                    }
                }
            },

            "/d/{id}": {
                "get": {
                    "summary": "Download beatmapset (.osz)",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer" }
                        },
                        {
                            "name": "nv",
                            "in": "query",
                            "schema": { "type": "string" },
                            "required": false,
                            "description": "No video flag (0/1/true/false)"
                        },
                        {
                            "name": "novideo",
                            "in": "query",
                            "schema": { "type": "string" },
                            "required": false,
                            "description": "Alias of nv"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Beatmapset download",
                            "content": {
                                "application/x-osu-beatmap-archive": {
                                    "schema": { "type": "string", "format": "binary" }
                                }
                            }
                        },
                        "404": { "description": "Not found" }
                    }
                }
            }
        }
    }))
}

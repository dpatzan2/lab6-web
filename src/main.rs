//IMPORTACIONES
use actix_web::{
    delete, get, post, put, patch,
    web, App, HttpResponse, HttpServer, Responder,
};
use actix_cors::Cors;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection
};
use serde::{Deserialize, Serialize};
use futures_util::stream::TryStreamExt;


// MODELOS PRINCIPALES PARA CONSUMIR Y CREAR NUEVOS DATOS
#[derive(Debug, Serialize, Deserialize)]
struct Match {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<mongodb::bson::oid::ObjectId>,

    pub homeTeam: String,
    pub awayTeam: String,
    pub matchDate: String,
    pub goals: i32,
    pub yellowCards: i32,
    pub redCards: i32,
    pub extraTime: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewMatch {
    pub homeTeam: String,
    pub awayTeam: String,
    pub matchDate: String,
}

// Mi conexión a la db
async fn get_collection() -> Collection<Match> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("test_db");
    db.collection::<Match>("matches")
}

// GET /matches
#[get("/matches")]
async fn get_matches() -> impl Responder {
    let collection = get_collection().await;
    let cursor_result = collection.find(doc! {}).await;

    match cursor_result {
        Ok(mut cursor) => {
            let mut results = Vec::new();
            while let Ok(Some(m)) = cursor.try_next().await {
                let id_str = m.id.map(|oid| oid.to_hex()).unwrap_or_default();
                
                let response_obj = serde_json::json!({
                    "id": format!("'{}'", id_str),
                    "homeTeam": m.homeTeam,
                    "awayTeam": m.awayTeam,
                    "matchDate": m.matchDate,
                    "goals": m.goals,
                    "yellowCards": m.yellowCards,
                    "redCards": m.redCards,
                    "extraTime": m.extraTime
                });

                println!("Partido: {:?}", response_obj);

                results.push(response_obj);
            }
            HttpResponse::Ok().json(results)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error al buscar los partidos: {e}")),
    }
}

// GET /matches/{id}
#[get("/matches/{id}")]
async fn get_match_by_id(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let match_id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&match_id_str);

    match match_id_objid {
        Ok(obj_id) => {
            match collection.find_one(doc! { "_id": obj_id }).await {
                Ok(Some(found_match)) => {
                    let id_str = found_match
                        .id
                        .map(|oid| oid.to_hex())
                        .unwrap_or_default();

                    let response_obj = serde_json::json!({
                        "id": id_str,
                        "homeTeam": found_match.homeTeam,
                        "awayTeam": found_match.awayTeam,
                        "matchDate": found_match.matchDate,
                        "goals": found_match.goals,
                        "yellowCards": found_match.yellowCards,
                        "redCards": found_match.redCards,
                        "extraTime": found_match.extraTime
                    });

                    HttpResponse::Ok().json(response_obj)
                }
                Ok(None) => HttpResponse::NotFound().body("No se encontró el partido con ese ID"),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Error al buscar: {e}"))
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().body("El ID no es válido como ObjectId"),
    }
}


// POST /matches
#[post("/matches")]
async fn create_match(new_match: web::Json<NewMatch>) -> impl Responder {
    let collection = get_collection().await;

    let mut match_doc = Match {
        id: None, 
        homeTeam: new_match.homeTeam.clone(),
        awayTeam: new_match.awayTeam.clone(),
        matchDate: new_match.matchDate.clone(),
        goals: 0,
        yellowCards: 0,
        redCards: 0,
        extraTime: 0,
    };

    match collection.insert_one(&match_doc).await {
        Ok(insert_result) => {
            if let Some(obj_id) = insert_result.inserted_id.as_object_id() {
                match_doc.id = Some(obj_id);
            }
            HttpResponse::Created().json(match_doc)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error al insertar: {e}")),
    }
}

// PUT /matches/{id}
#[put("/matches/{id}")]
async fn update_match(
    path: web::Path<String>,
    updated_data: web::Json<NewMatch>,
) -> impl Responder {
    let collection = get_collection().await;
    let match_id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&match_id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let update_doc = doc! {
            "$set": {
                "homeTeam": &updated_data.homeTeam,
                "awayTeam": &updated_data.awayTeam,
                "matchDate": &updated_data.matchDate,
            }
        };

        let update_result = collection.update_one(filter, update_doc).await;
        match update_result {
            Ok(res) => {
                if res.matched_count == 1 {
                    HttpResponse::Ok().body("Partido actualizado exitosamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al actualizar: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

// DELETE /matches/{id}
#[delete("/matches/{id}")]
async fn delete_match(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let match_id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&match_id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let delete_result = collection.delete_one(filter).await;
        match delete_result {
            Ok(res) => {
                if res.deleted_count == 1 {
                    HttpResponse::Ok().body("Partido eliminado exitosamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al eliminar: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

// PATCH /matches/{id}/goals
#[patch("/matches/{id}/goals")]
async fn update_match_goals(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let update_doc = doc! { "$inc": { "goals": 1 }};

        let result = collection.update_one(filter, update_doc).await;
        match result {
            Ok(r) => {
                if r.matched_count == 1 {
                    HttpResponse::Ok().body("Gol registrado correctamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al registrar gol: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

// PATCH /matches/{id}/yellowcards
#[patch("/matches/{id}/yellowcards")]
async fn update_match_yellow_card(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let update_doc = doc! { "$inc": { "yellowCards": 1 }};

        let result = collection.update_one(filter, update_doc).await;
        match result {
            Ok(r) => {
                if r.matched_count == 1 {
                    HttpResponse::Ok().body("Tarjeta amarilla registrada correctamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al registrar tarjeta amarilla: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

// PATCH /matches/{id}/redcards
#[patch("/matches/{id}/redcards")]
async fn update_match_red_card(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let update_doc = doc! { "$inc": { "redCards": 1 }};

        let result = collection.update_one(filter, update_doc).await;
        match result {
            Ok(r) => {
                if r.matched_count == 1 {
                    HttpResponse::Ok().body("Tarjeta roja registrada correctamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al registrar tarjeta roja: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

// PATCH /matches/{id}/extratime
#[patch("/matches/{id}/extratime")]
async fn update_match_extra_time(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;
    let id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&id_str);

    if let Ok(obj_id) = match_id_objid {
        let filter = doc! { "_id": obj_id };
        let update_doc = doc! { "$inc": { "extraTime": 1 }};

        let result = collection.update_one(filter, update_doc).await;
        match result {
            Ok(r) => {
                if r.matched_count == 1 {
                    HttpResponse::Ok().body("Tiempo extra establecido correctamente")
                } else {
                    HttpResponse::NotFound().body("No se encontró el partido con ese ID")
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("Error al establecer tiempo extra: {e}")),
        }
    } else {
        HttpResponse::BadRequest().body("El ID no es válido como ObjectId")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .service(
                web::scope("/api")
                    .service(get_matches)
                    .service(get_match_by_id)
                    .service(create_match)
                    .service(update_match)
                    .service(delete_match)
                    .service(update_match_goals)
                    .service(update_match_yellow_card)
                    .service(update_match_red_card)
                    .service(update_match_extra_time)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
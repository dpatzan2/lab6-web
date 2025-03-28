
//IMPORTACIONES
use actix_web::{
    delete, get, post, put,
    web, App, HttpResponse, HttpServer, Responder,
    http::header::ContentType
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
    pub id: Option<ObjectId>, 
    pub homeTeam: String,
    pub awayTeam: String,
    pub matchDate: String,
}



#[derive(Debug, Serialize, Deserialize)]
struct NewMatch {
    pub homeTeam: String,
    pub awayTeam: String,
    pub matchDate: String,
}

// Mi conexcion a la db
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
    let mut cursor = match collection.find(doc! {}).await {
        Ok(cur) => cur,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error al buscar: {}", e)),
    };

    let mut results: Vec<Match> = Vec::new();
    while let Some(res) = cursor.try_next().await.unwrap() {
        results.push(res);
    }

    HttpResponse::Ok().json(results)
}

#[get("/matches/{id}")]
async fn get_match_by_id(path: web::Path<String>) -> impl Responder {
    let collection = get_collection().await;

    let match_id_str = path.into_inner();
    let match_id_objid = ObjectId::parse_str(&match_id_str);

    match match_id_objid {
        Ok(obj_id) => {
            match collection.find_one(doc! { "_id": obj_id }).await {
                Ok(Some(found_match)) => HttpResponse::Ok().json(found_match),
                Ok(None) => HttpResponse::NotFound().body("No se encontró el partido con ese ID"),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error al buscar: {}", e)),
            }
        },
        Err(_) => HttpResponse::BadRequest().body("El ID no es válido como ObjectId"),
    }
}

// POST /matches
#[post("/matches")]
async fn create_match(new_match: web::Json<NewMatch>) -> impl Responder {
    let collection = get_collection().await;
    let match_doc = Match {
        id: None,
        homeTeam: new_match.homeTeam.clone(),
        awayTeam: new_match.awayTeam.clone(),
        matchDate: new_match.matchDate.clone(),
    };

    let insert_result = collection.insert_one(&match_doc).await;
    match insert_result {
        Ok(result) => {
            let obj_id = result.inserted_id
                .as_object_id()
                .map(|oid| oid.to_hex()) 
                .unwrap_or_default();
            HttpResponse::Created().body(format!("Partido creado con ID: {}", obj_id))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error al insertar: {}", e)),
    }
}

// PUT /matches/{id}
#[put("/matches/{id}")]
async fn update_match(
    path: web::Path<String>,
    updated_data: web::Json<NewMatch>,
) -> impl Responder {
    let collection = get_collection().await;

    // Convertimos el {id} de String a ObjectId
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
                    .service(create_match)
                    .service(update_match)
                    .service(delete_match)
                    .service(get_match_by_id)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
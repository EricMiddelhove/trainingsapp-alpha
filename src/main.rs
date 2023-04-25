use std::{collections::HashMap, env, str::FromStr};

use actix_web::{
    http,
    web::{self, Header},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use mongodb::{
    bson::{self, doc},
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::__private::de::IdentifierDeserializer;

#[derive(serde::Deserialize, serde::Serialize)]
struct Trainingsplan {
    owner: String,
    name: String,
    description: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct InputTrainingsplan {
    email: String,
    name: String,
    description: String,
    auth_token: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct GetTrainingsplan {
    id: String,
    auth_token: String,
    email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct OutputTrainingsplan {
    name: String,
    description: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct AuthenticateRequest {
    verify_secret: String,
    auth_token: String,
    email: String,
}

async fn get_new_client() -> Client {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare())
            .await
            .expect("Failed to parse options");

    let client = Client::with_options(options).expect("Failed to initialize client.");

    client
}

async fn authenticate_request(auth_token: String, email: &String) -> String {
    // Authenticate with auth server

    println!("Authenticating request...");

    let verify_secret = env::var("VERIFY_SECRET").expect("VERIFY_SECRET must be set");
    let authenticate_url = env::var("AUTHENTICATE_URL").expect("AUTHENTICATE_URL must be set");

    let mut map = HashMap::new();
    map.insert("verify_secret", verify_secret);
    map.insert("auth_token", auth_token);
    map.insert("email", email.to_string());

    let client = surf::Client::new();
    let req = client
        .post(authenticate_url)
        .body_json(&map)
        .expect("req failed")
        .recv_string();

    // futures::future::try_join(req).await.expect("req failed");

    println!("Awaiting response...");
    let res = req.await.expect("req failed");

    println!("Response: {}", res);

    res
}

async fn get_trainingsplan(req: HttpRequest) -> impl Responder {
    let auth_token = req
        .headers()
        .get("auth-token")
        .unwrap()
        .to_str()
        .expect("auth-token header is not a string")
        .to_string();

    let request_email: String = req
        .headers()
        .get("email")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let id = req.query_string().split("=").collect::<Vec<&str>>()[1];

    let is_authenticated: bool = authenticate_request(auth_token, &request_email)
        .await
        .parse()
        .unwrap();

    if is_authenticated == false {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let client = get_new_client().await;
    let coll = client
        .database("trainingsplans")
        .collection("trainingsplans");

    let id = bson::oid::ObjectId::from_str(&id).expect("Invalid id");

    let result = coll.find_one(doc! {"_id": id}, None).await;

    let trainingsplan: Trainingsplan = result.unwrap().expect("No trainingsplan found");

    if trainingsplan.owner != request_email {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let output_trainingsplan = OutputTrainingsplan {
        name: trainingsplan.name,
        description: trainingsplan.description,
    };

    HttpResponse::Ok().json(output_trainingsplan)
}

async fn post_trainingsplan(info: web::Json<InputTrainingsplan>) -> impl Responder {
    let auth_token = info.auth_token.clone();
    let request_email: String = info.email.clone();

    let is_authenticated = authenticate_request(auth_token, &request_email).await;
    println!("is_authenticated: {}", is_authenticated);

    if is_authenticated == "false" {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let trainingsplan = Trainingsplan {
        name: info.name.clone(),
        description: info.description.clone(),
        owner: info.email.clone(),
    };

    let doc = bson::to_document(&trainingsplan).unwrap();

    let client = get_new_client().await;
    let coll = client
        .database("trainingsplans")
        .collection("trainingsplans");

    let result = coll.insert_one(doc, None);
    let inserted_id = result.await.unwrap().inserted_id;

    HttpResponse::Ok().body(inserted_id.to_string())
}

async fn index() -> &'static str {
    "This server is online!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Environment: ");
    println!("MONGO_URI: {}", env::var("MONGO_URI").unwrap());
    println!("IP_ADDRESS: {}", env::var("IP_ADDRESS").unwrap());
    println!();

    println!("Server is running on port 3000...");

    let ip_address = env::var("IP_ADDRESS").unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/trainingsplan", web::post().to(post_trainingsplan))
            .route("/trainingsplan", web::get().to(get_trainingsplan))
    })
    .bind((ip_address, 3000))?
    .run()
    .await
}

use async_std::io;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tide::Server;

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Entity {
    name: String,
    age: i16,
    children: Vec<SubEntity>,
}

#[derive(Deserialize, Serialize)]
struct SubEntity {
    sub_name: String,
    sub_age: i16,
}

fn determineEntity<'a>(uri: &tide::http::Uri, entities: &'a Vec<Entity>) -> Option<&'a Entity>{
    let mut entityC = vec![];
    for (idx, c) in uri.path().chars().enumerate() {
        if idx == 0 {
            continue;
        }
        if c != '(' {
            entityC.push(c);
        }
        else {
            break;
        }
    }
    let entity: String = entityC.iter().collect();
    entities.iter().find(|e| e.name == entity)
}

fn main() -> io::Result<()> {
    let entities = vec![
        Entity {
            name: "entity1".to_string(),
            age: 1,
            children: vec![SubEntity { sub_name: "sub11".to_string(), sub_age: 11}, SubEntity { sub_name: "sub12".to_string(), sub_age: 12}]
        },
        Entity {
            name: "entity2".to_string(),
            age: 2,
            children: vec![SubEntity { sub_name: "sub21".to_string(), sub_age: 21}, SubEntity { sub_name: "sub22".to_string(), sub_age: 22}]
        },
        Entity {
            name: "entity3".to_string(),
            age: 3,
            children: vec![SubEntity { sub_name: "sub31".to_string(), sub_age: 31}, SubEntity { sub_name: "sub32".to_string(), sub_age: 32}]
        },
    ];
    task::block_on(async {
        let mut app = Server::with_state(entities);

        app.at("/")
            .get(|req: tide::Request<Vec<Entity>>| async move {
                tide::Response::new(200).body_string("Please use proper routes.".to_string())
            });

        app.at("*")
            .get(|req: tide::Request<Vec<Entity>>| async move {
                let uri = req.uri();

                let option_entity = determineEntity(uri, req.state());
                if option_entity.is_none() {
                    return tide::Response::new(404).body_string("404, not found".to_string());
                }
                tide::Response::new(200).body_json(option_entity.unwrap()).unwrap()
            });

        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}
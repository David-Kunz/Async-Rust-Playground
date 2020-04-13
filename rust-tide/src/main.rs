use async_std::io;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tide::Server;

#[derive(Deserialize, Serialize)]
struct Entity {
    name: String,
    elements: Vec<Element>,
}


#[derive(Deserialize, Serialize)]
struct Element {
    name: String,
}

fn determine_entity<'a>(uri: &tide::http::Uri, entities: &'a Vec<Entity>) -> Option<&'a Entity>{
    let mut entity_c = vec![];
    for (idx, c) in uri.path().chars().enumerate() {
        if idx == 0 {
            continue;
        }
        if c != '(' {
            entity_c.push(c);
        }
        else {
            break;
        }
    }
    let entity: String = entity_c.iter().collect();
    entities.iter().find(|e| e.name == entity)
}

fn main() -> io::Result<()> {
    let entities = vec![
        Entity {
            name: "entity1".to_string(),
            elements: vec![Element { name: "sub11".to_string()}, Element { name: "sub12".to_string()}]
        },
        Entity {
            name: "entity2".to_string(),
            elements: vec![Element { name: "sub21".to_string()}, Element { name: "sub22".to_string()}]
        },
        Entity {
            name: "entity3".to_string(),
            elements: vec![Element { name: "sub31".to_string()}, Element { name: "sub32".to_string()}]
        },
    ];
    task::block_on(async {
        let mut app = Server::with_state(entities);

        app.at("/")
            .get(|_req: tide::Request<Vec<Entity>>| async move {
                tide::Response::new(200).body_string("Please use proper routes.".to_string())
            });

        app.at("*")
            .get(|req: tide::Request<Vec<Entity>>| async move {
                let uri = req.uri();

                let option_entity = determine_entity(uri, req.state());
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

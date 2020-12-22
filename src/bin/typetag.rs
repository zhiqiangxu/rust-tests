use serde::{Deserialize, Serialize};

fn main() -> serde_json::Result<()> {
    let page_load = PageLoad;
    let page_event = &page_load as &dyn WebEvent;

    let json = serde_json::to_string(page_event)?;
    println!("PageLoad json: {}", json);

    let de: Box<dyn WebEvent> = serde_json::from_str(&json)?;
    de.inspect();

    println!();

    let click = Click { x: 10, y: 10 };
    let click_event = &click as &dyn WebEvent;
    let json = serde_json::to_string(click_event)?;
    println!("Click json: {}", json);
    let de: Box<dyn WebEvent> = serde_json::from_str(&json)?;
    de.inspect();

    let events = vec![page_event, click_event];
    let json = serde_json::to_string(&events)?;
    println!("events json: {}", json);

    let de: Vec<Box<dyn WebEvent>> = serde_json::from_str(&json)?;
    for e in de.iter() {
        e.inspect();
    }

    Ok(())
}

#[typetag::serde(tag = "type")]
trait WebEvent {
    fn inspect(&self);
}

#[derive(Serialize, Deserialize)]
struct PageLoad;

#[typetag::serde]
impl WebEvent for PageLoad {
    fn inspect(&self) {
        println!("200 milliseconds or bust");
    }
}

#[derive(Serialize, Deserialize)]
struct Click {
    x: i32,
    y: i32,
}

#[typetag::serde]
impl WebEvent for Click {
    fn inspect(&self) {
        println!("negative space between the ads: x={} y={}", self.x, self.y);
    }
}

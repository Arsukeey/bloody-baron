extern crate rand;

mod events;
mod character;
mod map;
mod abilities;
mod ai;
mod protag;

// events
mod packs;

fn main() -> Result<(), std::io::Error> {
    let mut game_data = events::GameData::new();
    let mut event_queue = events::EventQueue::new();

    print!("{}[2J", 27 as char);
    println!("WELCOME TO BLOODY BARON");

    loop {
        event_queue.execute_event(&mut game_data);
        event_queue.allow_next_event()?;
        let event = event_queue.poll_next_event();
        match event {
            None => {
                break;
            }
            _ =>  {
                ();
            }
        }
    }

    Ok(())
}

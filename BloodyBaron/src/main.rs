extern crate rand;

mod events;
mod character;
mod map;
mod abilities;
mod dialogue;

// events
mod movement;

fn main() -> Result<(), std::io::Error> {
    let mut event_queue = events::EventQueue::new();

    println!("WELCOME TO BLOODY BARON");
    println!("{:?}", event_queue.map);
    event_queue.allow_next_event()?;
    event_queue.poll_next_event();

    loop {
        event_queue.execute_event();
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

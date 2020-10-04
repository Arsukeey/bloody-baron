use crate::map::Map;
use crate::abilities::*;
use crate::ai::AI;

pub const NUMBER_OF_CHARS: usize = 9;

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub is_alive: bool,
    pub is_killer: bool,
    pub ability: fn(Vec<Character>, &Map) -> (),
    pub ai: AI,
    pub details: String
    // pub dialogue: fn(&Character, Vec<Character>, &Map) -> ()
}

impl Character {
    pub fn freya(map: &Map) -> Self {
        let name = "Freya".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = investigation;
        let ai = AI::new(map);
        let details = "Freya is one of the most prodigious logicians in the kingdom. 
        At the age of 19, she achieved several advancements in philosophy and mathematics, which cannot be overstated. 
        She has then approached the noble status as someone whose abilities were admired by many, 
        and has been living according to noble standards ever since. 
        While gazing at Freya's blue eyes, you wonder what goes on in her mind, especially in the situation she's in right now, 
        as a murder suspect. 
        You'd think that someone who's known to be so brilliant wouldn't commit a terrible murder without reason. Still...\n
        Personality: Freya is cold and is usually caught wandering in thoughts, even during conversations. 
        If you want to befriend her, you should be prepared to go through long moments of silence.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn ravi(map: &Map) -> Self {
        let name = "Ravi".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = celestial_call;
        let ai = AI::new(map);
        let details = "Even as the general of the royal army, Ravi is known to be a man of honor and a good heart. 
        His face shows scars from battle, but is by no means intimidating. What you feel by observing this man is actually, 
        a feeling of confort, and hope.
        His tanned skin and old, wise aura are persistent even when being a suspect of political murder, as is the case 
        right now. This could be a bless... or a curse. Because you never know what lies behind such an benevolent looking 
        person.\n
        Personality: Ravi is known to be very calm and gentle, and easy to befriend. But he's also known to be pretty sharp 
        when it comes to feelings. Expect he won't be fooled or manipulated easily.\n
        ".to_string();
        
        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn luna(map: &Map) -> Self {
        let name = "Luna".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = overhearing;
        let ai = AI::new(map);
        let details = "Ever since catgirls started being created in laboratories, they have been admired as loyal pets and very fun 
        entertainers. Luna is no different. She isn't known to be smart or very mature, but her charisma is exceptional, as expected 
        for someone who's been the royal family's favorite pet for as long as she lived. Even during these moments of despair, Luna 
        seems to be constantly having fun. You couldn't possibly imagine a creature like this killing in cold blood. Yet...\n
        Personality: expect cheerfullness and some blissful innocence from Luna. It's rather easy to gain her trust, but hurting 
        her feelings even so slightly can make her throw a tantrum.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn john(map: &Map) -> Self {
        let name = "John".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = perfect_bluff;
        let ai = AI::new(map);
        let details = "Life on the edge is what John experiences since he left the comfort of his home to become one of the most 
        famous gamblers in the kingdom. Not only a smart man, but someone who isn't afraid to take risks, this guy has gone a long 
        way by simply bluffing and persuading people... until he's suddenly a political murder suspect, like everyone else here. 
        Still, you imagine what kind of mind games he's planning on the others...\n
        Personality: John is known to be very hyped about competition and somewhat narcisistic. You'll have to go through some bragging 
        if you want to befriend him.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn amanda(map: &Map) -> Self {
        let name = "Amanda".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pick_lock;
        let ai = AI::new(map);
        let details = "The engineers of this kingdom have been known for their art of war. Amanda has showed in numerous 
        territorial disputes involving this land who's boss. Not only well versed in assembling powerful machines, Amanda is also 
        a pretty intimidating person: one of her eyes is nowhere to be seen, covered by an eyepatch, and the other grey, small, 
        penetrating gaze is... staring right back at you!\n
        Personality: Amanda won't probably be a fan of conversation, unless you wanna talk about she's good at. You could say she's 
        a simple person. Just don't piss her off, though.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn vincent(map: &Map) -> Self {
        let name = "Vincent".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pitiful_begging;
        let ai = AI::new(map);
        let details = "
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn larissa(map: &Map) -> Self {
        let name = "Larissa".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = exceptional_diplomacy;
        let ai = AI::new(map);
        let details = "
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn chio(map: &Map) -> Self {
        let name = "Chio".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = cold_execution;
        let ai = AI::new(map);
        let details = "
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }

    pub fn odette(map: &Map) -> Self {
        let name = "Odette".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = night_owl;
        let ai = AI::new(map);
        let details = "
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details
        }
    }
}
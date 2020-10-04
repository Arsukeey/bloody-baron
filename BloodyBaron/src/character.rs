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
    pub details: String,
    pub trust_line: String
    // pub dialogue: fn(&Character, Vec<Character>, &Map) -> ()
}

impl Character {
    pub fn freya(map: &Box<Map>) -> Self {
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
        let trust_line = "After spending some moments of awkward silence with Freya, composed of short intervals of conversation, 
        she seems to like you a little bit better.\n
        You gained Freya's trust!\n
        You can now use the ability Investigate. It lets you know where a person has been for the last hour and a half.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn ravi(map: &Box<Map>) -> Self {
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
        let trust_line = "After talking to Ravi about how life used to be before you two got locked up here, you two finally 
        seem to be getting closer.\n
        You have gained Ravi's trust!\n
        You can now use the ability Celestial Call. It lets you act like an angel and protect one person for an entire day. 
        This person won't die for the next 24 hours. You can't use this on yourself.\n
        ".to_string();
        
        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn luna(map: &Box<Map>) -> Self {
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
        let trust_line = "After some small talk with Luna and having to withstand her idiosyncrasies, you realize you two became 
        friends in basically no time at all.\n
        You gained Luna's trust!\n
        You can now use the ability Overhear. Sharp catgirl listening skills can help you determine who's in nearby rooms 
        in advance. Very useful when exploring at nighttime.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn john(map: &Box<Map>) -> Self {
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
        let trust_line = "John brags to you about his life achievements for about an hour. It was a tough time having to go through 
        this, but after some nodding, John seems to like the fact that you listened.\n
        You have gained John's trust!\n
        You have now the passive ability Perfect Bluff. From now on, the other characters will move and take decisions according to a 
        suboptimal pattern.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn amanda(map: &Box<Map>) -> Self {
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
        let trust_line = "Amanda looks down on you for some moments and is reluctant to talk at first, but after some time, she lets 
        it go and talks to you about warfare. You don't understand much she's saying, though.\n
        You have gained Amanda's trust!\n
        You can now use the ability Pick Lock. When exploring the Inn Hallway at night, you can check who is in their room and who isn't.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn vincent(map: &Box<Map>) -> Self {
        let name = "Vincent".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pitiful_begging;
        let ai = AI::new(map);
        let details = "
        ".to_string();
        let trust_line = "You hear Vincent whine about his current situation and express his entitlement to you for about an hour. 
        It was painful, but you did your best. He seems to be fond of you now.\n
        You have gained Vincent's trust!\n
        You can now use the ability Pitiful Begging. When alone with someone in your room, you can beg on your knees for your life. 
        If that someone is the killer, they'll spare you for the moment. Their identity won't be revealed, though.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn larissa(map: &Box<Map>) -> Self {
        let name = "Larissa".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = exceptional_diplomacy;
        let ai = AI::new(map);
        let details = "
        ".to_string();
        let trust_line = "You have a good conversation with Larissa. Despite her melancholy, you do the best to cheer her up, and 
        then finally you can see a faint smile on her face.\n
        You have gained Larissa's trust!\n
        You have now the passive ability Exceptional Diplomacy. During trials, people may follow your lead even if you haven't gained 
        their trust yet.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn chio(map: &Box<Map>) -> Self {
        let name = "Chio".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = cold_execution;
        let ai = AI::new(map);
        let details = "
        ".to_string();
        let trust_line = "It was hard to engage in a conversation with Chio and you felt personally insecure beside a man who's a 
        known war criminal. But in the end, he decided to teach you a bit of his ways.\n
        You have gained Chio's trust!\n
        You can now use the ability Cold Execution. When alone with someone in a room, you may choose to kill that person. A trial 
        will follow soon after. If you murder the killer, though, you win the game.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }

    pub fn odette(map: &Box<Map>) -> Self {
        let name = "Odette".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = night_owl;
        let ai = AI::new(map);
        let details = "
        ".to_string();
        let trust_line = "Odette talks to you in a codescending manner at first, and you can feel her smug smile penetrating your heart. 
        Despite feeling personally offended for some moments, you seem to have successfully strenghtened your bonds with your boss.\n
        You have gained Odette's trust!\n
        You have now the passive ability Night Owl. From now on, meeting the killer at nighttime doesn't mean instant death. On other 
        words, nighttime for you is the same as daytime.\n
        ".to_string();

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai,
            details,
            trust_line
        }
    }
}
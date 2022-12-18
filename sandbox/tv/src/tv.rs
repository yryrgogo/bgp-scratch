use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum State {
    PowerOn,
    PowerOff,
}

#[derive(Debug)]
pub enum Event {
    PushedPowerButton,
    PushedVolumeIncreaseButton,
    PushedVolumeDecreaseButton,
}

pub struct TV {
    pub now_state: State,
    pub event_queue: EventQueue,
    pub volume: u8,
}

impl TV {
    pub fn new() -> Self {
        let now_state = State::PowerOff;
        let event_queue = EventQueue::new();
        let volume = 10;
        Self {
            now_state,
            event_queue,
            volume,
        }
    }

    pub fn be_pushed_power_button(&mut self) {
        self.event_queue.enqueue(Event::PushedPowerButton);
    }

    pub fn be_pushed_volume_increase_button(&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeIncreaseButton);
    }

    pub fn be_pushed_volume_decrease_button(&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeDecreaseButton);
    }

    pub fn handle_event(&mut self, event: Event) {
        match &self.now_state {
            &State::PowerOn => match event {
                Event::PushedPowerButton => {
                    self.now_state = State::PowerOff;
                }
                Event::PushedVolumeIncreaseButton => {
                    self.volume += 1;
                }
                Event::PushedVolumeDecreaseButton => {
                    self.volume -= 1;
                }
            },
            &State::PowerOff => match event {
                Event::PushedPowerButton => {
                    self.now_state = State::PowerOn;
                }
                _ => (),
            },
        }
    }
}

pub struct EventQueue(VecDeque<Event>);

impl EventQueue {
    pub fn new() -> Self {
        let d = VecDeque::new();
        EventQueue(d)
    }

    pub fn dequeue(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    pub fn enqueue(&mut self, event: Event) {
        self.0.push_back(event);
    }
}

pub fn push_random_button_of_tv(tv: &mut TV) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        1 => tv.be_pushed_power_button(),
        2 => tv.be_pushed_volume_increase_button(),
        3 => tv.be_pushed_volume_decrease_button(),
        _ => (),
    }
}

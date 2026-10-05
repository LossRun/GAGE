use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// 4-State Digital Logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicState {
    Zero,
    One,
    Z, // High-Impedance
    X, // Unknown / Contention
}

impl LogicState {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => LogicState::Zero,
            '1' => LogicState::One,
            'z' | 'Z' => LogicState::Z,
            _ => LogicState::X,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            LogicState::Zero => '0',
            LogicState::One => '1',
            LogicState::Z => 'Z',
            LogicState::X => 'X',
        }
    }

    pub fn not(self) -> Self {
        match self {
            LogicState::Zero => LogicState::One,
            LogicState::One => LogicState::Zero,
            LogicState::Z => LogicState::X,
            LogicState::X => LogicState::X,
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (LogicState::Zero, _) | (_, LogicState::Zero) => LogicState::Zero,
            (LogicState::One, LogicState::One) => LogicState::One,
            _ => LogicState::X,
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (LogicState::One, _) | (_, LogicState::One) => LogicState::One,
            (LogicState::Zero, LogicState::Zero) => LogicState::Zero,
            _ => LogicState::X,
        }
    }

    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (LogicState::Zero, LogicState::Zero) | (LogicState::One, LogicState::One) => LogicState::Zero,
            (LogicState::Zero, LogicState::One) | (LogicState::One, LogicState::Zero) => LogicState::One,
            _ => LogicState::X,
        }
    }
}

/// Simulation Event queued in Priority Queue
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimEvent {
    pub timestamp: u64,
    pub wire_id: usize,
    pub new_state: LogicState,
}

impl Ord for SimEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: smaller timestamp = higher priority
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for SimEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Primitive Gate Types
#[derive(Debug, Clone)]
pub enum GateKind {
    Not,
    And,
    Or,
    Nand,
    Nor,
    Xor,
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub kind: GateKind,
    pub inputs: Vec<usize>,
    pub output: usize,
    pub delay: u64,
}

/// Circuit & Simulation Environment
pub struct CircuitSimulator {
    pub current_time: u64,
    pub wire_names: Vec<String>,
    pub wire_map: HashMap<String, usize>,
    pub wire_states: Vec<LogicState>,
    pub gates: Vec<Gate>,
    pub event_queue: BinaryHeap<SimEvent>,
    pub history: HashMap<usize, Vec<(u64, LogicState)>>,
}

impl CircuitSimulator {
    pub fn new() -> Self {
        Self {
            current_time: 0,
            wire_names: Vec::new(),
            wire_map: HashMap::new(),
            wire_states: Vec::new(),
            gates: Vec::new(),
            event_queue: BinaryHeap::new(),
            history: HashMap::new(),
        }
    }

    pub fn add_wire(&mut self, name: &str, initial: LogicState) -> usize {
        if let Some(&id) = self.wire_map.get(name) {
            return id;
        }
        let id = self.wire_names.len();
        self.wire_names.push(name.to_string());
        self.wire_map.insert(name.to_string(), id);
        self.wire_states.push(initial);
        self.history.insert(id, vec![(0, initial)]);
        id
    }

    pub fn add_gate(&mut self, kind: GateKind, inputs: Vec<&str>, output: &str, delay: u64) {
        let input_ids: Vec<usize> = inputs.iter().map(|&n| self.add_wire(n, LogicState::X)).collect();
        let output_id = self.add_wire(output, LogicState::X);
        self.gates.push(Gate {
            kind,
            inputs: input_ids,
            output: output_id,
            delay: if delay == 0 { 1 } else { delay },
        });
    }

    pub fn set_wire(&mut self, name: &str, state: LogicState, delay: u64) {
        let id = self.add_wire(name, state);
        self.event_queue.push(SimEvent {
            timestamp: self.current_time + delay,
            wire_id: id,
            new_state: state,
        });
    }

    pub fn step(&mut self) -> bool {
        if let Some(event) = self.event_queue.pop() {
            self.current_time = event.timestamp;
            let current = self.wire_states[event.wire_id];

            if current != event.new_state {
                self.wire_states[event.wire_id] = event.new_state;
                self.history.entry(event.wire_id).or_default().push((self.current_time, event.new_state));

                for gate in &self.gates {
                    if gate.inputs.contains(&event.wire_id) {
                        let evaluated = self.eval_gate(gate);
                        self.event_queue.push(SimEvent {
                            timestamp: self.current_time + gate.delay,
                            wire_id: gate.output,
                            new_state: evaluated,
                        });
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn run_ticks(&mut self, ticks: u64) {
        let target = self.current_time + ticks;
        while let Some(top) = self.event_queue.peek() {
            if top.timestamp <= target {
                self.step();
            } else {
                break;
            }
        }
        self.current_time = target;
    }

    fn eval_gate(&self, gate: &Gate) -> LogicState {
        let in_states: Vec<LogicState> = gate.inputs.iter().map(|&id| self.wire_states[id]).collect();
        match gate.kind {
            GateKind::Not => {
                in_states.get(0).copied().unwrap_or(LogicState::X).not()
            }
            GateKind::And => {
                in_states.iter().copied().fold(LogicState::One, |acc, s| acc.and(s))
            }
            GateKind::Or => {
                in_states.iter().copied().fold(LogicState::Zero, |acc, s| acc.or(s))
            }
            GateKind::Nand => {
                in_states.iter().copied().fold(LogicState::One, |acc, s| acc.and(s)).not()
            }
            GateKind::Nor => {
                in_states.iter().copied().fold(LogicState::Zero, |acc, s| acc.or(s)).not()
            }
            GateKind::Xor => {
                if in_states.len() == 2 {
                    in_states[0].xor(in_states[1])
                } else {
                    LogicState::X
                }
            }
        }
    }

    pub fn render_waveform(&self, duration: u64) -> String {
        let mut out = String::new();
        out.push_str("\n\x1b[1;36m  === GAGE LOGIC ANALYZER WAVEFORM ===\x1b[0m\n");
        out.push_str(&format!("  Time Scale: 0 -> {} ticks\n\n", duration));

        out.push_str("  Wire        | Waveform\n");
        out.push_str("  ------------+--------------------------------------------------------\n");

        for (wire_id, name) in self.wire_names.iter().enumerate() {
            let mut line_top = format!("  {:11} | ", name);
            let mut line_mid = format!("  {:11} | ", "");

            let transitions = self.history.get(&wire_id).cloned().unwrap_or_default();

            for t in 0..=duration.min(45) {
                let mut state = LogicState::Zero;
                for &(time, s) in &transitions {
                    if time <= t {
                        state = s;
                    } else {
                        break;
                    }
                }

                match state {
                    LogicState::One => {
                        line_top.push_str("\x1b[32m─\x1b[0m");
                        line_mid.push(' ');
                    }
                    LogicState::Zero => {
                        line_top.push(' ');
                        line_mid.push_str("\x1b[32m─\x1b[0m");
                    }
                    LogicState::Z => {
                        line_top.push(' ');
                        line_mid.push_str("\x1b[33m┄\x1b[0m");
                    }
                    LogicState::X => {
                        line_top.push_str("\x1b[31m✕\x1b[0m");
                        line_mid.push_str("\x1b[31m✕\x1b[0m");
                    }
                }
            }
            out.push_str(&line_top);
            out.push('\n');
            out.push_str(&line_mid);
            out.push('\n');
            out.push_str("              |\n");
        }

        out.push_str("  T-Ticks     | ");
        for t in 0..=(duration.min(45)) {
            if t % 5 == 0 {
                out.push_str(&format!("\x1b[90m{}\x1b[0m", t % 10));
            } else {
                out.push_str("\x1b[90m.\x1b[0m");
            }
        }
        out.push_str("\n\n");
        out
    }
}

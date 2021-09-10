use simdpath_core::bytes::*;
use simdpath_core::engine::result::*;
use simdpath_core::engine::runner::*;
use simdpath_core::query::*;

pub struct StackBasedRunner<'a, 'b> {
    query: &'b JsonPathQuery<'a>,
}

struct RunnerResult<'a> {
    pub count: usize,
    pub remaining_bytes: &'a [u8],
}

impl<'a, 'b> StackBasedRunner<'a, 'b> {
    pub fn compile_query(query: &'b JsonPathQuery<'a>) -> Self {
        StackBasedRunner { query }
    }
}

impl<'a, 'b> Runner for StackBasedRunner<'a, 'b> {
    fn count(&self, input: &str) -> CountResult {
        let bytes = input.as_bytes();
        let mut state = State::Initial(InitialState::new(self.query.root(), bytes));
        CountResult {
            count: state.run().count,
        }
    }
}

trait Runnable<'a> {
    fn run(&mut self) -> RunnerResult<'a>;
}

struct InitialState<'a, 'b, 'c> {
    node: &'b JsonPathQueryNode<'a>,
    bytes: &'c [u8],
}

struct RecursiveDescentState<'a, 'b, 'c> {
    node: &'b JsonPathQueryNode<'a>,
    pub bytes: &'c [u8],
}

struct RecurseInListState<'a, 'b, 'c> {
    node: &'b JsonPathQueryNode<'a>,
    pub bytes: &'c [u8],
}

impl<'a, 'b, 'c> InitialState<'a, 'b, 'c> {
    fn new(node: &'b JsonPathQueryNode<'a>, bytes: &'c [u8]) -> Self {
        InitialState { node, bytes }
    }
}

impl<'a, 'b, 'c> RecursiveDescentState<'a, 'b, 'c> {
    fn new(node: &'b JsonPathQueryNode<'a>, bytes: &'c [u8]) -> Self {
        debug_assert! {node.is_descendant()}
        RecursiveDescentState { node, bytes }
    }
}

impl<'a, 'b, 'c> RecurseInListState<'a, 'b, 'c> {
    fn new(node: &'b JsonPathQueryNode<'a>, bytes: &'c [u8]) -> Self {
        debug_assert! {node.is_descendant()}
        RecurseInListState { node, bytes }
    }
}

enum State<'a, 'b, 'c> {
    Initial(InitialState<'a, 'b, 'c>),
    RecursivelyFindLabel(RecursiveDescentState<'a, 'b, 'c>),
}

impl<'a, 'b, 'c> Runnable<'c> for State<'a, 'b, 'c> {
    fn run(&mut self) -> RunnerResult<'c> {
        match self {
            State::Initial(state) => state.run(),
            State::RecursivelyFindLabel(state) => state.run(),
        }
    }
}

impl<'a, 'b, 'c> Runnable<'c> for InitialState<'a, 'b, 'c> {
    fn run(&mut self) -> RunnerResult<'c> {
        debug_assert! {self.node.is_root()};

        let first_brace = find_byte(b'{', self.bytes);
        match first_brace {
            None => RunnerResult {
                count: 0,
                remaining_bytes: &[],
            },
            Some(first_brace) => match self.node.child() {
                None => RunnerResult {
                    count: 1,
                    remaining_bytes: &self.bytes[first_brace + 1..],
                },
                Some(child_node) => match child_node {
                    JsonPathQueryNode::Descendant(_) => State::RecursivelyFindLabel(
                        RecursiveDescentState::new(child_node, &self.bytes[first_brace + 1..]),
                    )
                    .run(),
                    JsonPathQueryNode::Label(_, _) => {
                        panic! {"Currently a Label expression can only be used after a Descendant expression."}
                    }
                    JsonPathQueryNode::Root(_) => {
                        panic! {"Root expression should not be reachable."}
                    }
                },
            },
        }
    }
}

impl<'a, 'b, 'c> Runnable<'c> for RecursiveDescentState<'a, 'b, 'c> {
    fn run(&mut self) -> RunnerResult<'c> {
        let label_node = self.node.child().unwrap();
        let label = *match label_node {
            JsonPathQueryNode::Label(label, _) => label,
            _ => panic!("RecursiveDescentState must be run on a Label node."),
        };

        // Inbound contract: we are inside a JSON object after its opening brace
        // and zero or more keys and values passed. Therefore there is either another
        // label in front or the closing brace.
        let mut bytes = self.bytes;
        let mut count = 0;

        loop {
            let next = find_unescaped_byte2(b'"', b'}', bytes)
                .expect("JSON is malformed: closing brace missing.");
            let byte = bytes[next];
            bytes = &bytes[next + 1..];

            if byte == b'}' {
                break;
            }

            // Here byte == '"' and bytes[0] is exactly the first byte of the label.
            let end_of_label = find_unescaped_byte(b'"', bytes)
                .expect("JSON is malformed: closing quote missing.");
            let current_label = &bytes[..end_of_label];
            bytes = &bytes[end_of_label + 1..];

            let colon = find_byte(b':', bytes).expect("JSON is malformed: colon missing.");
            bytes = &bytes[colon + 1..];

            let object_start =
                find_non_whitespace(bytes).expect("JSON is malformed: value missing.");
            let object_start_byte = bytes[object_start];
            bytes = &bytes[object_start + 1..];

            if label == current_label {
                if label_node.child().is_none() {
                    count += 1;
                }
                match object_start_byte {
                    b'{' => {
                        let mut next_state = match label_node.child() {
                            // No child means that we keep recursivelly searching for the same label.
                            None => RecursiveDescentState::new(self.node, bytes),
                            Some(child_node) => match child_node {
                                JsonPathQueryNode::Descendant(_) => {
                                    RecursiveDescentState::new(child_node, bytes)
                                }
                                JsonPathQueryNode::Label(_, _) => {
                                    panic! {"Currently a Label expression can only be used after a Descendant expression."}
                                }
                                JsonPathQueryNode::Root(_) => {
                                    panic! {"Root expression should not be reachable."}
                                }
                            },
                        };
                        let result = next_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'[' => {
                        let mut next_state = match label_node.child() {
                            // No child means that we keep recursivelly searching for the same label.
                            None => RecurseInListState::new(self.node, bytes),
                            Some(child_node) => match child_node {
                                JsonPathQueryNode::Descendant(_) => {
                                    RecurseInListState::new(child_node, bytes)
                                }
                                JsonPathQueryNode::Label(_, _) => {
                                    panic! {"Currently a Label expression can only be used after a Descendant expression."}
                                }
                                JsonPathQueryNode::Root(_) => {
                                    panic! {"Root expression should not be reachable."}
                                }
                            },
                        };
                        let result = next_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'"' => {
                        let next = find_unescaped_byte(b'"', bytes)
                            .expect("JSON is malformed: closing quote missing.");
                        bytes = &bytes[next + 1..];
                    }
                    _ => {
                        let next = find_byte2(b',', b'}', bytes)
                            .expect("JSON is malformed: closing brace missing.");
                        bytes = &bytes[next..];
                    }
                }
            } else {
                match object_start_byte {
                    b'{' => {
                        let mut recursive_state = RecursiveDescentState::new(self.node, bytes);
                        let result = recursive_state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'[' => {
                        let mut state = RecurseInListState::new(self.node, bytes);
                        let result = state.run();

                        bytes = result.remaining_bytes;
                        count += result.count;
                    }
                    b'"' => {
                        let next = find_unescaped_byte(b'"', bytes)
                            .expect("JSON is malformed: closing quote missing.");
                        bytes = &bytes[next + 1..];
                    }
                    _ => {
                        let next = find_byte2(b',', b'}', bytes)
                            .expect("JSON is malformed: closing brace missing.");
                        bytes = &bytes[next..];
                    }
                }
            }
        }

        // At the end bytes are exactly the bytes after the closing brace of the JSON object
        // we started in.
        RunnerResult {
            count,
            remaining_bytes: bytes,
        }
    }
}

impl<'a, 'b, 'c> Runnable<'c> for RecurseInListState<'a, 'b, 'c> {
    fn run(&mut self) -> RunnerResult<'c> {
        // Inbound contract: we are inside a JSON list after its opening bracket
        // and zero or more values passed. Therefore there is either another
        // value in front or the closing bracket.
        let mut bytes = self.bytes;
        let mut count = 0;

        loop {
            let next =
                find_non_whitespace(bytes).expect("JSON is malformed: closing bracket missing.");
            let byte = bytes[next];
            bytes = &bytes[next + 1..];

            if byte == b']' {
                break;
            }

            match byte {
                b'{' => {
                    let mut next_state = RecursiveDescentState::new(self.node, bytes);
                    let result = next_state.run();

                    bytes = result.remaining_bytes;
                    count += result.count;

                    let next = find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[next] == b',' {
                        bytes = &bytes[next + 1..];
                    } else {
                        bytes = &bytes[next..];
                    }
                }
                b'[' => {
                    let mut next_state = RecurseInListState::new(self.node, bytes);
                    let result = next_state.run();
                    bytes = result.remaining_bytes;
                    count += result.count;
                }
                b'"' => {
                    let next = find_unescaped_byte(b'"', bytes)
                        .expect("JSON is malformed: closing quote missing.");
                    bytes = &bytes[next + 1..];
                    let comma = find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[comma] == b',' {
                        bytes = &bytes[comma + 1..];
                    } else {
                        bytes = &bytes[comma..];
                    }
                }
                _ => {
                    let next = find_byte2(b',', b']', bytes)
                        .expect("JSON is malformed: closing bracket missing.");

                    if bytes[next] == b',' {
                        bytes = &bytes[next + 1..];
                    } else {
                        bytes = &bytes[next..];
                    }
                }
            }
        }

        // At the end bytes are exactly the bytes after the closing bracket of the JSON list
        // we started in.
        RunnerResult {
            count,
            remaining_bytes: bytes,
        }
    }
}
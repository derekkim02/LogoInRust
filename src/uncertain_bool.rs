pub enum UncertainBool {
    True,
    False,
    Unknown,
}

impl UncertainBool {
    pub fn is_true(&self) -> bool {
        matches!(self, UncertainBool::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, UncertainBool::False)
    }
}

pub fn is_option_eq<T: PartialEq>(a: Option<T>, b: Option<T>) -> UncertainBool {
    match (a, b) {
        (Some(a), Some(b)) => {
            if a == b {
                UncertainBool::True
            } else {
                UncertainBool::False
            }
        },
        (None, None) => UncertainBool::Unknown,
        _ => UncertainBool::False,
    }
}
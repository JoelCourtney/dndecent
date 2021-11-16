use std::cell::{Ref, RefCell};
use seed::log;
use serde::{Serialize, Deserialize};
use crate::{eval, ops};

/// Version of the Character struct that is stored as a json file for saving.
///
/// It contains the minimal amount of information to completely reconstruct the character.
/// Much of the information is stored in the content structs (like races, classes, etc),
/// and the content-creator should also try to be as minimal as possible. (de)Serialization
/// of the nested content information is handled automatically by Serde and Typetag.
#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    pub(crate) name: String,

    pub(crate) health: u32,
    pub(crate) temp_health: u32,

    // pub(crate) base_abilities: AbilityMap<u32>,

    // alignment: Alignment,

    inspiration: bool,

    // money: MoneyTypeMap<u32>,

    // pub(crate) race: Box<dyn Race>,
    // pub(crate) classes: Vec<(Box<dyn Class>, u32)>,
    // pub(crate) background: Box<dyn Background>,

    // inventory: Vec<(Box<dyn Item>, Equipped, bool)>,

    pub(crate) description: String
}

pub fn try_it() -> CharacterResult<()> {
    let char = Character::default();
    ops! { char
        x 1 => x.push_str(y?)
        x 0 => *x = "hello".to_string()
        y 0 => *y = " world!".to_string()
        z 1 => {
            z.push_str(y?);
            z.make_ascii_uppercase();
        }
    }
    let ref_char = &char;
    log!(eval!(ref_char.x));
    log!(eval!(ref_char.y));
    log!(eval!(ref_char.z));
    Ok(())
}

#[derive(Default)]
struct Character {
    x: LazyValue<String>,
    y: LazyValue<String>,
    z: LazyValue<String>
}

/// Special result for character operations.
type CharacterResult<T> = Result<T, CharacterError>;

/// Possible errors during character evaluation.
#[derive(Debug)]
pub enum CharacterError {
    /// Occurs when the engine encounters a cyclical dependency between values.
    ///
    /// Detected by the [RefCell] inside [LazyValue].
    Deadlock,

    #[allow(dead_code)]
    InvalidState(String)
}

/// An operation performed on the character by a piece of content.
type CharacterOperation<T> = Box<dyn FnOnce(&mut T, &Character) -> CharacterResult<()>>;

/// A value computed lazily with a list of operations to perform.
///
/// The [Character] uses a large collection of these; by wrapping the values
/// in [RefCell]s and computing them lazily, this implicitly performs a
/// depth-first search of the dependency graph.
///
/// For example, if an operation for `char.x` depends on the value of `char.y`,
/// the operation will have to call `char.y.evaluate()`. If nothing attempts to
/// access `x` or `y`, neither value will be computed. If `x` is evaluated, it will
/// trigger `y`'s evaluation first. `y` will only be computed once even if both `x`
/// and `y` are used explicitly.
#[derive(Default)]
#[repr(transparent)]
struct LazyValue<T: Default>(RefCell<(T, Vec<(u8,CharacterOperation<T>)>)>);

impl<T: Default> LazyValue<T> {
    /// Evaluate the value if needed, and return a [Ref] to it.
    fn evaluate(&self, character: &Character) -> CharacterResult<Ref<T>> {
        let mut immutable_borrow = self.0.borrow();
        if !immutable_borrow.1.is_empty() {
            std::mem::drop(immutable_borrow);
            if let Ok(mut mutable_borrow) = self.0.try_borrow_mut() {
                let (value, ops) = &mut *mutable_borrow;
                ops.sort_unstable_by(|(first_rank,_), (second_rank,_)| first_rank.cmp(second_rank));
                for (_,op) in ops.drain(..) {
                    op(value, character)?;
                }
            } else {
                return Err(CharacterError::Deadlock)
            }
            immutable_borrow = self.0.borrow();
        }
        Ok(Ref::map(immutable_borrow, |(value, _)| value))
    }

    /// Registers an operation to be performed later.
    fn register(&self, rank: u8, op: CharacterOperation<T>) {
        self.0.borrow_mut().1.push((rank, op));
    }
}

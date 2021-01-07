macros::register!("/");

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
///
/// Only used inside the auto-generated [Registry] struct below.
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

/// Registry contains hashmaps mapping pretty-print content names to tuples: (Registration, default constructor).
///
/// This struct is a singleton in usage, although there is no check to stop from instantiating it
/// multiple times. Don't.
///
/// This struct and some of its methods are generated by the [macros::registry] macro. I recognize
/// that makes it pretty hard to use, so here is an example generated code:
///
/// ```
/// #[derive(Debug)]
/// pub struct Registry {
///     races: HashMap<&'static str, (Registration, fn() -> Box<dyn Race>)>,
///     feats: HashMap<&'static str, (Registration, fn() -> Box<dyn Feat>)>,
/// }
///
/// impl Registry {
///     pub fn new() -> Self {
///         Registry {
///             races: hashmap! {
///                 official::players_handbook::races::human::CONTENT_NAME => (
///                     Registration {
///                         collection: official::COLLECTION_NAME,
///                         source: official::players_handbook::COLLECTION_NAME
///                     },
///                     official::players_handbook::races::human::new as fn() -> Box<dyn Race>
///                 )
///             },
///             feats: hashmap! {
///                 official::players_handbook::feats::alert::CONTENT_NAME => (
///                     Registration {
///                         collection: official::COLLECTION_NAME,
///                         source: official::players_handbook::COLLECTION_NAME
///                     },
///                     official::players_handbook::feats::alert::new as fn() -> Box<dyn Feat>
///                 )
///             }
///         }
///     }
///
///     pub fn race(&self, search_name: &str) -> Option<Box<dyn Race>> {
///         match self.races.get(search_name) {
///             Some((_, construct)) => Some(construct()),
///             None => None
///         }
///     }
///
///     pub fn feat(&self, search_name: &str) -> Option<Box<dyn Feat>> {
///         match self.feats.get(search_name) {
///             Some((_, construct)) => Some(construct()),
///             None => None
///         }
///     }
/// }
/// ```
///
/// Note that the `CONTENT_NAME` and `COLLECTION_NAME` constants are generated by macros in their
/// associated file. `COLLECTION_NAME` is generated by [macros::register]. 'CONTENT_NAME' is
/// generated by [macros::race], [macros::feat], etc. (the attribute macro used to declare the content).
/// These are the pretty-print names the dev wants to display to the user.
///
/// # Examples
///
/// The registry is be instantiated with:
///
/// ```
/// let registry = Registry::new();
/// ```
///
/// As mentioned before, this struct should be treated like a singleton. *Don't make your own*.
/// Use the one instantiated in main, if you have to use one at all. That said, if you do make
/// your own instance, it probably won't matter even a little bit. But I will be sad.
///
/// Instances of content structs can be requested with a function of the associated type. To create
/// a [Human], for example:
///
/// ```
/// let human = registry.race("Human");
/// ```
///
/// If the content's pretty-print and pascal case names are different, use the pretty-print name.
///
/// ```
/// let yuan_ti_pureblood = registry.race("Yuan-ti Pureblood");
/// ```
///
/// Content created this way will have default values for all of their fields. The only other way
/// to create these objects is through deserialization from a json file, which can contain
/// non-default values.
#[macros::registry(2)]
pub struct Registry;

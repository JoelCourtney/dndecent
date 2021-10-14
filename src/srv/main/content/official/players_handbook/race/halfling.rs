crate::name!("Halfling");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Halfling {
    subrace: Box<dyn HalflingSubrace>
}

#[content]
impl Race for Halfling {
    fn resolve(&mut self, c: &mut Character) {
        i! {
            c.size = CreatureSize::Small;
            c.speeds.walk = 25;
            c.saving_throw_notes <<= "**ADV** against frightened";
            c.languages >>= vec![Language::Common, Language::Halfling];
        }

        m! { c.abilities.dexterity += 2 }

        i! {
            c.race_traits >>= vec! [
                Element::Str (
                    "**Ability Score Increase:** Your `Dexterity` score increases by 2.",
                ),
                Element::Str (
                    "**Age:** A halfling reaches adulthood at the age of 20 and generally lives into the middle of his or her second century.",
                ),
                Element::Str (
                    "**Alignment:** Most halflings are lawful good. As a rule, they are good-hearted and kind, hate to see others in pain, and have no tolerance for oppression. They are also very orderly and traditional, leaning heavily on the support of their community and the comfort of their old ways.",
                ),
                Element::Str (
                    "**Size:** Halflings average about 3 feet tall and weigh about 40 pounds. Your size is `Small`.",
                ),
                Element::Str (
                    "**Speed:** Your base walking speed is `25 feet`.",
                ),
                Element::Str (
                    "**Lucky:** When you roll a 1 on an attack roll, ability check, or saving throw, you can reroll the die and must use the new roll.",
                ),
                Element::Str (
                    "**Brave:** You have `advantage` on saving throws against being `frightened`.",
                ),
                Element::Str (
                    "**Halfling Nimbleness:** You can move through the space of any creature that is of a size larger than yours.",
                ),
                Element::Str (
                    "**Languages:** You can speak, read, and write `Common` and `Halfling`. The Halfling language isn't secret, but halflings are loath to share it with others. They write very little, so they don't have a rich body of literature. Their oral tradition, however, is very strong. Almost all halflings speak `Common` to converse with the people in whose lands they dwell or through which they are traveling.",
                ),
                Element::Choice {
                    text: "**Subrace:** Choose a subrace.",
                    data: &mut self.subrace,
                    unique: false
                }
            ];
        }

        self.subrace.resolve(c);
    }

    description! {r#"
        # Halfling

        > Regis the halfling, the only one of his kind for hundreds of miles in any direction, locked his fingers behind his head and leaned back against the mossy blanket of the tree trunk. Regis was short, even by the standards of his diminutive race, with the fluff of his curly brown locks barely cresting the three-foot mark, but his belly was amply thickened by his love of a good meal, or several, as the opportunities presented themselves. The crooked stick that served as his fishing pole rose up above him, clenched between two of his toes, and hung out over the quiet lake, mirrored perfectly in the glassy surface of Maer Dualdon.
        >
        > — R.A. Salvatore, The Crystal Shard

        The comforts of home are the goals of most halflings’ lives: a place to settle in peace and quiet, far from marauding monsters and clashing armies; a blazing fire and a generous meal; fine drink and fine conversation. Though some halflings live out their days in remote agricultural communities, others form nomadic bands that travel constantly, lured by the open road and the wide horizon to discover the wonders of new lands and peoples. But even these wanderers love peace, food, hearth, and home, though home might be a wagon jostling along a dirt road or a raft floating downriver.

        ## Small and Practical

        The diminutive halflings survive in a world full of larger creatures by avoiding notice or, barring that, avoiding offense. Standing about 3 feet tall, they appear relatively harmless and so have managed to survive for centuries in the shadow of empires and on the edges of wars and political strife. They are inclined to be stout, weighing between 40 and 45 pounds.

        Halflings’ skin ranges from tan to pale with a ruddy cast, and their hair is usually brown or sandy brown and wavy. They have brown or hazel eyes. Halfling men often sport long sideburns, but beards are rare among them and mustaches even more so. They like to wear simple, comfortable, and practical clothes, favoring bright colors.

        Halfling practicality extends beyond their clothing. They’re concerned with basic needs and simple pleasures and have little use for ostentation. Even the wealthiest of halflings keep their treasures locked in a cellar rather than on display for all to see. They have a knack for finding the most straightforward solution to a problem, and have little patience for dithering.

        ## Kind and Curious

        Halflings are an affable and cheerful people. They cherish the bonds of family and friendship as well as the comforts of hearth and home, harboring few dreams of gold or glory. Even adventurers among them usually venture into the world for reasons of community, friendship, wanderlust, or curiosity. They love discovering new things, even simple things, such as an exotic food or an unfamiliar style of clothing.

        Halflings are easily moved to pity and hate to see any living thing suffer. They are generous, happily sharing what they have even in lean times.

        ## Blend into the Crowd

        Halflings are adept at fitting into a community of humans, dwarves, or elves, making themselves valuable and welcome. The combination of their inherent stealth and their unassuming nature helps halflings to avoid unwanted attention.

        Halflings work readily with others, and they are loyal to their friends, whether halfling or otherwise. They can display remarkable ferocity when their friends, families, or communities are threatened.

        ## Pastoral Pleasantries

        Most halflings live in small, peaceful communities with large farms and well-kept groves. They rarely build kingdoms of their own or even hold much land beyond their quiet shires. They typically don’t recognize any sort of halfling nobility or royalty, instead looking to family elders to guide them. Families preserve their traditional ways despite the rise and fall of empires.

        Many halflings live among other races, where the halflings’ hard work and loyal outlook offer them abundant rewards and creature comforts. Some halfling communities travel as a way of life, driving wagons or guiding boats from place to place and maintaining no permanent home.

        > AFFABLE AND POSITIVE
        >
        > Halflings try to get along with everyone else and are loath to make sweeping generalizations—especially negative ones.
        >
        > **Dwarves.** “Dwarves make loyal friends, and you can count on them to keep their word. But would it hurt them to smile once in a while?”
        >
        > **Elves.** “They’re so beautiful! Their faces, their music, their grace and all. It’s like they stepped out of a wonderful dream. But there’s no telling what’s going on behind their smiling faces—surely more than they ever let on.”
        >
        > **Humans.** “Humans are a lot like us, really. At least some of them are. Step out of the castles and keeps, go talk to the farmers and herders and you’ll find good, solid folk. Not that there’s anything wrong with the barons and soldiers—you have to admire their conviction. And by protecting their own lands, they protect us as well.”

        ## Exploring Opportunities

        Halflings usually set out on the adventurer’s path to defend their communities, support their friends, or explore a wide and wonder-filled world. For them, adventuring is less a career than an opportunity or sometimes a necessity.

        ## Halfling Names

        A halfling has a given name, a family name, and possibly a nickname. Family names are often nicknames that stuck so tenaciously they have been passed down through the generations.

        **Male Names:** Alton, Ander, Cade, Corrin, Eldon, Errich, Finnan, Garret, Lindal, Lyle, Merric, Milo, Osborn, Perrin, Reed, Roscoe, Wellby

        **Female Names:** Andry, Bree, Callie, Cora, Euphemia, Jillian, Kithri, Lavinia, Lidda, Merla, Nedda, Paela, Portia, Seraphina, Shaena, Trym, Vani, Verna

        **Family Names:** Brushgather, Goodbarrel, Greenbottle, High-hill, Hilltopple, Leagallow, Tealeaf, Thorngage, Tosscobble, Underbough

        ## Subrace

        The two main kinds of halfling, lightfoot and stout, are more like closely related families than true subraces. Choose one of these subraces or one from another source.

        ## Halfling Traits

        Your halfling character has a number of traits in common with all other halflings.

        ### Ability Score Increase

        Your Dexterity score increases by 2.

        ### Age

        A halfling reaches adulthood at the age of 20 and generally lives into the middle of his or her second century.

        ### Alignment

        Most halflings are lawful good. As a rule, they are good-hearted and kind, hate to see others in pain, and have no tolerance for oppression. They are also very orderly and traditional, leaning heavily on the support of their community and the comfort of their old ways.

        ### Size

        Halflings average about 3 feet tall and weigh about 40 pounds. Your size is Small.

        ### Speed

        Your base walking speed is 25 feet.

        ### Lucky

        When you roll a 1 on the d20 for an attack roll, ability check, or saving throw, you can reroll the die and must use the new roll.

        ### Brave

        You have advantage on saving throws against being frightened.

        ### Halfling Nimbleness

        You can move through the space of any creature that is of a size larger than yours.

        ### Languages

        You can speak, read, and write Common and Halfling. The Halfling language isn’t secret, but halflings are loath to share it with others. They write very little, so they don’t have a rich body of literature. Their oral tradition, however, is very strong. Almost all halflings speak Common to converse with the people in whose lands they dwell or through which they are traveling.
    "#}
}


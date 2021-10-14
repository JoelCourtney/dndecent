crate::name!("Dwarf");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dwarf {
    subrace: Box<dyn DwarfSubrace>
}

#[content]
impl Race for Dwarf {
    fn resolve(&mut self, c: &mut Character) {
        i! {
            c.size = CreatureSize::Medium;
            c.speeds.walk = 25;
            c.languages >>= vec![Language::Common, Language::Dwarvish];
            c.passive_notes <<= "Darkvision 60ft";
            c.saving_throw_notes <<= "**ADV** against poison";
            c.defenses <<= "**RES** poison";
        }

        m! { c.abilities.constitution += 2 }

        i! {
            c.race_traits >>= vec! [
                Element::Str (
                    "**Ability Score Increase:** Your `Constitution` score increases by 2.",
                ),
                Element::Str (
                    "**Age:** Dwarves mature at the same rate as humans, but they’re considered young until they reach the age of 50. On average, they live about 350 years.",
                ),
                Element::Str (
                    "**Alignment:** Most dwarves are lawful, believing firmly in the benefits of a well-ordered society. They tend toward good as well, with a strong sense of fair play and a belief that everyone deserves to share in the benefits of a just order.",
                ),
                Element::Str (
                    "**Size:** Dwarves stand between 4 and 5 feet tall and average about 150 pounds. Your size is `Medium`.",
                ),
                Element::Str (
                    "**Speed:** Your base walking speed is `25 feet`. Your speed is not reduced by wearing heavy armor.",
                ),
                Element::Str (
                    "**Darkvision:** Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray."
                ),
                Element::Str (
                    "**Dwarven Resilience:** You have `advantage` on saving throws against poison, and you have `resistance` against poison damage."
                ),
                Element::Str (
                    "**Dwarven Combat Training:** You have proficiency with the battleaxe, handaxe, light hammer, and warhammer."
                ),
                Element::Str (
                    "**Tool Proficiency:** You gain proficiency with the artisan’s tools of your choice: smith’s tools, brewer’s supplies, or mason’s tools."
                ),
                Element::Str (
                    "**Stonecunning:** Whenever you make an `Intelligence` (`History`) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus."
                ),
                Element::Str (
                    "**Languages:** You can speak, read, and write `Common` and `Dwarvish`. Dwarvish is full of hard consonants and guttural sounds, and those characteristics spill over into whatever other language a dwarf might speak."
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
        # Dwarf

        > “Yer late, elf!” came the rough edge of a familiar voice. Bruenor Battlehammer walked up the back of his dead foe, disregarding the fact that the heavy monster lay on top of his elven friend. In spite of the added discomfort, the dwarf’s long, pointed, often-broken nose and gray-streaked though still-fiery red beard came as a welcome sight to Drizzt. “Knew I’d find ye in trouble if I came out an’ looked for ye!”
        >
        > — R. A. Salvatore, The Crystal Shard

        Kingdoms rich in ancient grandeur, halls carved into the roots of mountains, the echoing of picks and hammers in deep mines and blazing forges, a commitment to clan and tradition, and a burning hatred of goblins and orcs—these common threads unite all dwarves.

        ## Short and Stout

        Bold and hardy, dwarves are known as skilled warriors, miners, and workers of stone and metal. Though they stand well under 5 feet tall, dwarves are so broad and compact that they can weigh as much as a human standing nearly two feet taller. Their courage and endurance are also easily a match for any of the larger folk.

        Dwarven skin ranges from deep brown to a paler hue tinged with red, but the most common shades are light brown or deep tan, like certain tones of earth. Their hair, worn long but in simple styles, is usually black, gray, or brown, though paler dwarves often have red hair. Male dwarves value their beards highly and groom them carefully.

        ## Long Memory, Long Grudges

        Dwarves can live to be more than 400 years old, so the oldest living dwarves often remember a very different world. For example, some of the oldest dwarves living in Citadel Felbarr (in the world of the Forgotten Realms) can recall the day, more than three centuries ago, when orcs conquered the fortress and drove them into an exile that lasted over 250 years. This longevity grants them a perspective on the world that shorter-lived races such as humans and halflings lack.

        Dwarves are solid and enduring like the mountains they love, weathering the passage of centuries with stoic endurance and little change. They respect the traditions of their clans, tracing their ancestry back to the founding of their most ancient strongholds in the youth of the world, and don’t abandon those traditions lightly. Part of those traditions is devotion to the gods of the dwarves, who uphold the dwarven ideals of industrious labor, skill in battle, and devotion to the forge.

        Individual dwarves are determined and loyal, true to their word and decisive in action, sometimes to the point of stubbornness. Many dwarves have a strong sense of justice, and they are slow to forget wrongs they have suffered. A wrong done to one dwarf is a wrong done to the dwarf’s entire clan, so what begins as one dwarf’s hunt for vengeance can become a full-blown clan feud.

        ## Clans and Kingdoms

        Dwarven kingdoms stretch deep beneath the mountains where the dwarves mine gems and precious metals and forge items of wonder. They love the beauty and artistry of precious metals and fine jewelry, and in some dwarves this love festers into avarice. Whatever wealth they can’t find in their mountains, they gain through trade. They dislike boats, so enterprising humans and halflings frequently handle trade in dwarven goods along water routes. Trustworthy members of other races are welcome in dwarf settlements, though some areas are off limits even to them.

        The chief unit of dwarven society is the clan, and dwarves highly value social standing. Even dwarves who live far from their own kingdoms cherish their clan identities and affiliations, recognize related dwarves, and invoke their ancestors’ names in oaths and curses. To be clanless is the worst fate that can befall a dwarf.

        Dwarves in other lands are typically artisans, especially weaponsmiths, armorers, and jewelers. Some become mercenaries or bodyguards, highly sought after for their courage and loyalty.

        ## Gods, Gold, and Clan

        Dwarves who take up the adventuring life might be motivated by a desire for treasure—for its own sake, for a specific purpose, or even out of an altruistic desire to help others. Other dwarves are driven by the command or inspiration of a deity, a direct calling or simply a desire to bring glory to one of the dwarf gods. Clan and ancestry are also important motivators. A dwarf might seek to restore a clan’s lost honor, avenge an ancient wrong the clan suffered, or earn a new place within the clan after having been exiled. Or a dwarf might search for the axe wielded by a mighty ancestor, lost on the field of battle centuries ago.

        > SLOW TO TRUST
        >
        > Dwarves get along passably well with most other races. “The difference between an acquaintance and a friend is about a hundred years,” is a dwarf saying that might be hyperbole, but certainly points to how difficult it can be for a member of a short-lived race like humans to earn a dwarf’s trust.
        >
        > **Elves.** “It’s not wise to depend on the elves. No telling what an elf will do next; when the hammer meets the orc’s head, they’re as apt to start singing as to pull out a sword. They’re flighty and frivolous. Two things to be said for them, though: They don’t have many smiths, but the ones they have do very fine work. And when orcs or goblins come streaming down out of the mountains, an elf’s good to have at your back. Not as good as a dwarf, maybe, but no doubt they hate the orcs as much as we do.”
        >
        > **Halflings.** “Sure, they’re pleasant folk. But show me a halfling hero. An empire, a triumphant army. Even a treasure for the ages made by halfling hands. Nothing. How can you take them seriously?”
        >
        > **Humans.** “You take the time to get to know a human, and by then the human’s on her deathbed. If you’re lucky, she’s got kin—a daughter or granddaughter, maybe—who’s got hands and heart as good as hers. That’s when you can make a human friend. And watch them go! They set their hearts on something, they’ll get it, whether it’s a dragon’s hoard or an empire’s throne. You have to admire that kind of dedication, even if it gets them in trouble more often than not.”

        ## Dwarf Names

        A dwarf’s name is granted by a clan elder, in accordance with tradition. Every proper dwarven name has been used and reused down through the generations. A dwarf’s name belongs to the clan, not to the individual. A dwarf who misuses or brings shame to a clan name is stripped of the name and forbidden by law to use any dwarven name in its place.

        **Male Names:** Adrik, Alberich, Baern, Barendd, Brottor, Bruenor, Dain, Darrak, Delg, Eberk, Einkil, Fargrim, Flint, Gardain, Harbek, Kildrak, Morgran, Orsik, Oskar, Rangrim, Rurik, Taklinn, Thoradin, Thorin, Tordek, Traubon, Travok, Ulfgar, Veit, Vondal

        **Female Names:** Amber, Artin, Audhild, Bardryn, Dagnal, Diesa, Eldeth, Falkrunn, Finellen, Gunnloda, Gurdis, Helja, Hlin, Kathra, Kristryd, Ilde, Liftrasa, Mardred, Riswynn, Sannl, Torbera, Torgga, Vistra

        **Clan Names:** Balderk, Battlehammer, Brawnanvil, Dankil, Fireforge, Frostbeard, Gorunn, Holderhek, Ironfist, Loderr, Lutgehr, Rumnaheim, Strakeln, Torunn, Ungart

        ## Subrace

        Two main subraces of dwarves populate the worlds of D&D: hill dwarves and mountain dwarves. Choose one of these subraces or one from another source.

        > DUERGAR
        >
        > In cities deep in the Underdark live the duergar, or gray dwarves. These vicious, stealthy slave traders raid the surface world for captives, then sell their prey to the other races of the Underdark. They have innate magical abilities to become invisible and to temporarily grow to giant size.

        ## Dwarf Traits

        Your dwarf character has an assortment of inborn abilities, part and parcel of dwarven nature.

        ### Ability Score Increase

        Your Constitution score increases by 2.

        ### Age

        Dwarves mature at the same rate as humans, but they’re considered young until they reach the age of 50. On average, they live about 350 years.

        ### Alignment

        Most dwarves are lawful, believing firmly in the benefits of a well-ordered society. They tend toward good as well, with a strong sense of fair play and a belief that everyone deserves to share in the benefits of a just order.

        ### Size

        Dwarves stand between 4 and 5 feet tall and average about 150 pounds. Your size is Medium.

        ### Speed

        Your base walking speed is 25 feet. Your speed is not reduced by wearing heavy armor.

        ### Darkvision

        Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.

        ### Dwarven Resiliance

        You have advantage on saving throws against poison, and you have resistance against poison damage (explained in the “Combat” section).

        ### Dwarven Combat Training

        You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.

        ### Tool Proficiency

        You gain proficiency with the artisan’s tools of your choice: smith’s tools, brewer’s supplies, or mason’s tools.

        ### Stonecunning

        Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.

        ### Languages

        You can speak, read, and write Common and Dwarvish. Dwarvish is full of hard consonants and guttural sounds, and those characteristics spill over into whatever other language a dwarf might speak.
    "#}
}
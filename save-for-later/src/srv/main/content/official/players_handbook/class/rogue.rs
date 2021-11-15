crate::name!("Rogue");

#[asi_or_feat_fields([4, 8, 10, 12, 16, 19])]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Rogue {
    subclass: Box<dyn RoguishArchetype>,

    skill_proficiencies: [RogueSkill; 4],
    first_expertise: [RogueExpertiseChoice; 2],
    sixth_expertise: [RogueExpertiseChoice; 2],

    luck: bool
}

#[content]
impl Class for Rogue {
    properties! {
        hit_dice: u32 = 8
    }

    fn resolve(&mut self, c: &mut Character, level: u32, index: usize) {

        // LEVEL 1

        let diebs_tools_proficiency = if self.first_expertise.contains(&RogueExpertiseChoice::DiebsTools)
            || self.sixth_expertise.contains(&RogueExpertiseChoice::DiebsTools) {
            ProficiencyType::Double
        } else {
            ProficiencyType::Single
        };

        i! {
            c.armor_proficiencies <<= "Light Armor";
            c.weapon_proficiencies >>= vec! [
                "Simple Weapons",
                "Hand Crossbows",
                "Longswords",
                "Rapiers",
                "Shortswords"
            ];
            c.tool_proficiencies <<= ("Thieves' Tools", diebs_tools_proficiency);
            c.save_proficiencies.dexterity = ProficiencyType::Single;
            c.save_proficiencies.intelligence = ProficiencyType::Single;
        }

        for skill in &self.skill_proficiencies {
            match c.skill_proficiencies.get_mut(skill.into()) {
                Some(s) => i!{ *s = ProficiencyType::Single },
                None => {}
            }
        }

        for expertise in &self.first_expertise {
            match expertise.into() {
                Some(skill) => {
                    match c.skill_proficiencies.get_mut(skill) {
                        Some(s) => m! { *s = ProficiencyType::Double },
                        None => {}
                    }
                }
                None => {}
            }
        }

        let sneak_dice = (level + 1) / 2;
        i! {
            c.moves <<= Move::Other {
                element: Element::String(format!("**Sneak Attack:** You can deal an extra `{}d6` damage to one creature you hit with a `ranged` or `finesse` attack if you have `advantage` on the attack roll. You don’t need `advantage` on the attack roll if another enemy of the target is within `5 ft` of it, that enemy isn’t `incapacitated`, and you don’t have `disadvantage` on the attack roll.", sneak_dice)),
                time: MoveTime::Other("Once per turn, during an attack")
            }
        }

        i! {
            c.class_features[index] >>= vec! [
                Element::Str(
                    indoc! { r#"
                        **Hit Points:**
                        - *Hit Dice:* 1d8 per rogue level
                        - *Hit Points at 1st Level:* 8 + your Constitution modifier
                        - *Hit Points at Higher Levels:* 1d8 (or 5) + your Constitution modifier per rogue level after 1st
                    "# }
                ),
                Element::Choice {
                    text: indoc! { r#"
                        **Proficiencies:**
                        - *Armor:* Light armor
                        - *Weapons:* Simple weapons, hand crossbows, longswords, rapiers, shortswords
                        - *Tools:* Thieves’ tools
                        - *Saving Throws:* Dexterity, Intelligence
                        - *Skills:* Choose four from Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Performance, Persuasion, Sleight of Hand, and Stealth
                    "# },
                    data: &mut self.skill_proficiencies,
                    unique: true
                },
                Element::Str(
                    indoc! { r#"
                        **Equipment:** You start with the following equipment, in addition to the equipment granted by your background:
                        - (a) a rapier or (b) a shortsword
                        - (a) a shortbow and quiver of 20 arrows or (b) a shortsword
                        - (a) a burglar’s pack, (b) a dungeoneer’s pack, or (c) an explorer’s pack
                        - Leather armor, two daggers, and thieves’ tools
                    "#}
                ),
                Element::Choice {
                    text: "**Expertise:** At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves’ tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.",
                    data: &mut self.first_expertise,
                    unique: true
                },
                Element::Str(
                    indoc! { r#"
                        **Sneak Attack:** Beginning at 1st level, you know how to strike subtly and exploit a foe’s distraction. Once per turn, you can deal an extra `1d6` damage to one creature you hit with an attack if you have `advantage` on the attack roll. The attack must use a `finesse` or a `ranged` weapon.

                        You don’t need `advantage` on the attack roll if another enemy of the target is within `5 feet` of it, that enemy isn’t `incapacitated`, and you don’t have `disadvantage` on the attack roll.

                        The amount of the extra damage increases as you gain levels in this class, as shown in the Sneak Attack column of the Rogue table.
                    "#}
                ),
                Element::Str(
                    indoc! {r#"
                        **Thieves' Cant**: During your rogue training you learned thieves' cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves' cant understands such messages. It takes four times longer to convey such a message than it does to speak the same idea plainly.

                        In addition, you understand a set of secret signs and symbols used to convey short, simple messages, such as whether an area is dangerous or the territory of a thieves' guild, whether loot is nearby, or whether the people in an area are easy marks or will provide a safe house for thieves on the run.
                    "# }
                )
            ];
        }

        // LEVEL 2

        if level >= 2 {
            i! {
                c.moves <<= Move::Other {
                    element: Element::Str("**Cunning Action:** `Dash`, `Disengage`, or `Hide`."),
                    time: MoveTime::BonusAction
                };
                c.class_features[index] <<= Element::Str(
                    "**Cunning Action:** Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the `Dash`, `Disengage`, or `Hide` action."
                );
            }
        }

        // LEVEL 3

        if level >= 3 {
            i! {
                c.class_features[index] <<= Element::Choice {
                    text: "**Roguish Archetype:** At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level.",
                    data: &mut self.subclass,
                    unique: false
                }
            }
        }

        // LEVEL 4

        asi_or_feat!(4);

        // LEVEL 5

        if level >= 5 {
            i! {
                c.moves <<= Move::Other {
                    element: Element::Str("**Uncanny Dodge:** When an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you."),
                    time: MoveTime::Reaction
                };
                c.class_features[index] <<= Element::Str(
                    "**Uncanny Dodge:** Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you."
                );
            }
        }

        // LEVEL 6

        if level >= 6 {
            for expertise in &self.sixth_expertise {
                match expertise.into() {
                    Some(skill) => {
                        match c.skill_proficiencies.get_mut(skill) {
                            Some(s) => m! { *s = ProficiencyType::Double },
                            None => {}
                        }
                    }
                    None => {}
                }
            }

            i! {
                c.class_features[index] <<= Element::Choice {
                    text: "**Expertise:** At 6th level, choose two more of your skill proficiencies, or one more of your skill proficiencies and your proficiency with thieves’ tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.",
                    data: &mut self.sixth_expertise,
                    unique: true
                };
            }
        }

        // LEVEL 7

        if level >= 7 {
            i! {
                c.saving_throw_notes <<= "**DEX:** hover\n\n*[hover]: When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail.";
                c.class_features[index] <<= Element::Str(
                    "**Evasion:** Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or an Ice Storm spell. When you are subjected to an effect that allows you to make a `Dexterity saving throw` to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail."
                )
            }
        }

        asi_or_feat!(8);

        asi_or_feat!(10);

        if level >= 11 {
            i! {
                c.moves <<= Move::Other {
                    element: Element::Str("**Reliable Talent:** You can treat a `d20` roll of 9 or lower as 10."),
                    time: MoveTime::Other("when you make an `ability check` that lets you add your `proficiency bonus`")
                };
                c.class_features[index] <<= Element::Str(
                    "**Reliable Talent:** By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10."
                );
            }
        }
        asi_or_feat!(12);

        // LEVEL 14

        if level >= 14 {
            i! {
                c.passive_notes <<= "**Blindsense:** hover\n\n*[hover]: If you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you.";
                c.class_features[index] <<= Element::Str(
                    "**Blindsense:** Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you."
                )
            }
        }

        // LEVEL 15

        if level >= 15 {
            i! {
                c.save_proficiencies.wisdom = ProficiencyType::Single;
                c.class_features[index] <<= Element::Str(
                    "**Slippery Mind:** By 15th level, you have acquired greater mental strength. You gain proficiency in `Wisdom saving throws`."
                );
            }
        }

        asi_or_feat!(16);

        // LEVEL 18

        if level >= 18 {
            i! {
                c.defenses <<= "Elusive\n\n*[Elusive]: No attack roll has advantage against you while you aren't incapacitated.";
                c.class_features[index] <<= Element::Str(
                    "**Elusive:** Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren't incapacitated."
                )
            }
        }

        asi_or_feat!(19);

        // LEVEL 20

        if level == 20 {
            i! {
                c.moves <<= Move::Other {
                    element: if self.luck {
                        Element::Trigger {
                            text: "**Stroke of Luck:** You can turn an attack miss into a hit, or you can treat a failed `d20` roll as a 20. Once you use this feature, you can't use it again until you finish a short or long rest.",
                            event: Event::Other("rogue stroke of luck"),
                            button: "Use"
                        }
                    } else {
                        Element::Str("**Stroke of Luck:** You can turn a missed attack into a hit, or you can treat a failed `d20` roll as a 20. You can't use this feature again until you finish a short or long rest.")
                    },
                    time: MoveTime::Other("When your attack misses a target within range, or you fail an ability check.")
                };
                c.class_features[index] <<= Element::Str(
                    indoc! { r#"
                        **Stroke of Luck:** At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20.

                        Once you use this feature, you can't use it again until you finish a short or long rest.
                    "# }
                )
            }
        }

        self.subclass.resolve(c, level, index);
    }

    fn event(&mut self, e: &Event, level: u32, index: usize) {
        match e {
            Event::ShortRest | Event::LongRest => self.luck = true,
            Event::Other("rogue stroke of luck") => self.luck = false,
            _ => {}
        }
        self.subclass.event(e, level, index);
    }

    description! {r#"
        # Rogue

        Signaling for her companions to wait, a halfling creeps forward through the dungeon hall. She presses an ear to the door, then pulls out a set of tools and picks the lock in the blink of an eye. Then she disappears into the shadows as her fighter friend moves forward to kick the door open.

        A human lurks in the shadows of an alley while his accomplice prepares for her part in the ambush. When their target — a notorious slaver — passes the alleyway, the accomplice cries out, the slaver comes to investigate, and the assassin’s blade cuts his throat before he can make a sound.

        Suppressing a giggle, a gnome waggles her fingers and magically lifts the key ring from the guard’s belt. In a moment, the keys are in her hand, the cell door is open, and she and her companions are free to make their escape.

        Rogues rely on skill, stealth, and their foes’ vulnerabilities to get the upper hand in any situation. They have a knack for finding the solution to just about any problem, demonstrating a resourcefulness and versatility that is the cornerstone of any successful adventuring party.

        ## Skill and Precision

        Rogues devote as much effort to mastering the use of a variety of skills as they do to perfecting their combat abilities, giving them a broad expertise that few other characters can match. Many rogues focus on stealth and deception, while others refine the skills that help them in a dungeon environment, such as climbing, finding and disarming traps, and opening locks.

        When it comes to combat, rogues prioritize cunning over brute strength. A rogue would rather make one precise strike, placing it exactly where the attack will hurt the target most, than wear an opponent down with a barrage of attacks. Rogues have an almost supernatural knack for avoiding danger, and a few learn magical tricks to supplement their other abilities.

        ## A Shady Living

        Every town and city has its share of rogues. Most of them live up to the worst stereotypes of the class, making a living as burglars, assassins, cutpurses, and con artists. Often, these scoundrels are organized into thieves’ guilds or crime families. Plenty of rogues operate independently, but even they sometimes recruit apprentices to help them in their scams and heists. A few rogues make an honest living as locksmiths, investigators, or exterminators, which can be a dangerous job in a world where dire rats—and wererats—haunt the sewers.

        As adventurers, rogues fall on both sides of the law. Some are hardened criminals who decide to seek their fortune in treasure hoards, while others take up a life of adventure to escape from the law. Some have learned and perfected their skills with the explicit purpose of infiltrating ancient ruins and hidden crypts in search of treasure.

        ## Creating a Rogue

        As you create your rogue character, consider the character’s relationship to the law. Do you have a criminal past—or present? Are you on the run from the law or from an angry thieves’ guild master? Or did you leave your guild in search of bigger risks and bigger rewards? Is it greed that drives you in your adventures, or some other desire or ideal?

        What was the trigger that led you away from your previous life? Did a great con or heist gone terribly wrong cause you to reevaluate your career? Maybe you were lucky and a successful robbery gave you the coin you needed to escape the squalor of your life. Did wanderlust finally call you away from your home? Perhaps you suddenly found yourself cut off from your family or your mentor, and you had to find a new means of support. Or maybe you made a new friend—another member of your adventuring party—who showed you new possibilities for earning a living and employing your particular talents.

        > **QUICK BUILD**
        >
        > You can make a rogue quickly by following these suggestions. First, Dexterity should be your highest ability score. Make Intelligence your next-highest if you want to excel at Investigation or plan to take up the Arcane Trickster archetype. Choose Charisma instead if you plan to emphasize deception and social interaction. Second, choose the charlatan background.

        ## The Rogue Table

        | Level | Proficiency Bonus | Sneak Attack | Features                   |
        | ----- | -------------------- | --------------- | -------------------------------------- |
        | 1st   | +2             | 1d6         | Expertise, Sneak Attack, Thieves’ Cant |
        | 2nd   | +2             | 1d6         | Cunning Action               |
        | 3rd   | +2             | 2d6         | Roguish Archetype              |
        | 4th   | +2             | 2d6         | Ability Score Improvement          |
        | 5th   | +3             | 3d6         | Uncanny Dodge                |
        | 6th   | +3             | 3d6         | Expertise                  |
        | 7th   | +3             | 4d6         | Evasion                    |
        | 8th   | +3             | 4d6         | Ability Score Improvement          |
        | 9th   | +4             | 5d6         | Roguish Archetype Feature          |
        | 10th  | +4             | 5d6         | Ability Score Improvement          |
        | 11th  | +4             | 6d6         | Reliable Talent                |
        | 12th  | +4             | 6d6         | Ability Score Improvement          |
        | 13th  | +5             | 7d6         | Roguish Archetype Feature          |
        | 14th  | +5             | 7d6         | Blindsense                   |
        | 15th  | +5             | 8d6         | Slippery Mind                |
        | 16th  | +5             | 8d6         | Ability Score Improvement          |
        | 17th  | +6             | 9d6         | Roguish Archetype Feature          |
        | 18th  | +6             | 9d6         | Elusive                    |
        | 19th  | +6             | 10d6        | Ability Score Improvement          |
        | 20th  | +6             | 10d6        | Stroke of Luck               |

        ## Class Features

        As a rogue, you have the following class features.

        ### Hit Points

        **Hit Dice:** 1d8 per rogue level
        **Hit Points at 1st Level:** 8 + your Constitution modifier
        **Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per rogue level after 1st

        ### Proficiencies

        **Armor:** Light armor
        **Weapons:** Simple weapons, hand crossbows, longswords, rapiers, shortswords
        **Tools:** Thieves’ tools
        **Saving Throws:** Dexterity, Intelligence
        **Skills:** Choose four from Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Performance, Persuasion, Sleight of Hand, and Stealth

        ### Equipment

        You start with the following equipment, in addition to the equipment granted by your background:

          (a) a rapier or (b) a shortsword
          (a) a shortbow and quiver of 20 arrows or (b) a shortsword
          (a) a burglar’s pack, (b) a dungeoneer’s pack, or (c) an explorer’s pack
          Leather armor, two daggers, and thieves’ tools

        ### Expertise

        At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves’ tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.

        At 6th level, you can choose two more of your proficiencies (in skills or with thieves’ tools) to gain this benefit.

        ### Sneak Attack

        Beginning at 1st level, you know how to strike subtly and exploit a foe’s distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack if you have advantage on the attack roll. The attack must use a finesse or a ranged weapon.

        You don’t need advantage on the attack roll if another enemy of the target is within 5 feet of it, that enemy isn’t incapacitated, and you don’t have disadvantage on the attack roll.

        The amount of the extra damage increases as you gain levels in this class, as shown in the Sneak Attack column of the Rogue table.

        ### Thieves’ Cant

        During your rogue training you learned thieves’ cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves’ cant understands such messages. It takes four times longer to convey such a message than it does to speak the same idea plainly.

        In addition, you understand a set of secret signs and symbols used to convey short, simple messages, such as whether an area is dangerous or the territory of a thieves’ guild, whether loot is nearby, or whether the people in an area are easy marks or will provide a safe house for thieves on the run.

        ### Cunning Action

        Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the Dash, Disengage, or Hide action.

        ### Roguish Archetype

        At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level.

        ### Ability Score Improvement

        When you reach 4th level, and again at 8th, 10th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can’t increase an ability score above 20 using this feature.

        Using the optional feats rule, you can forgo taking this feature to take a feat of your choice instead.

        ### Uncanny Dodge

        Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack’s damage against you.

        ### Expertise

        At 6th level, choose two more of your skill proficiencies, or one more of your skill proficiencies and your proficiency with thieves’ tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.

        ### Evasion

        Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as an ancient red dragon’s fiery breath or an ice storm spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail.

        ### Reliable Talent

        By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10.

        ### Blindsense

        Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you.

        ### Slippery Mind

        By 15th level, you have acquired greater mental strength. You gain proficiency in Wisdom saving throws.

        ### Elusive

        Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren’t incapacitated.

        ### Stroke of Luck

        At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20.

        Once you use this feature, you can’t use it again until you finish a short or long rest.
    "#}
}

#[choose]
pub enum RogueSkill {
    Acrobatics,
    Athletics,
    Deception,
    Insight,
    Intimidation,
    Investigation,
    Perception,
    Performance,
    Persuasion,
    SleightOfHand = "Sleight of Hand",
    Stealth,
    Unknown
}

impl From<&RogueSkill> for Skill {
    fn from(s: &RogueSkill) -> Self {
        match s {
            RogueSkill::Acrobatics => Skill::Acrobatics,
            RogueSkill::Athletics => Skill::Athletics,
            RogueSkill::Deception => Skill::Deception,
            RogueSkill::Insight => Skill::Insight,
            RogueSkill::Intimidation => Skill::Intimidation,
            RogueSkill::Investigation => Skill::Investigation,
            RogueSkill::Perception => Skill::Perception,
            RogueSkill::Performance => Skill::Performance,
            RogueSkill::Persuasion => Skill::Persuasion,
            RogueSkill::SleightOfHand => Skill::SleightOfHand,
            RogueSkill::Stealth => Skill::Stealth,
            RogueSkill::Unknown => Skill::Unknown
        }
    }
}

#[choose]
pub enum RogueExpertiseChoice {
    Acrobatics,
    AnimalHandling = "Animal Handling",
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand = "Sleight of Hand",
    Stealth,
    Survival,
    DiebsTools = "Thieves' Tools",
    Unknown
}

impl From<&RogueExpertiseChoice> for Option<Skill> {
    fn from(e: &RogueExpertiseChoice) -> Self {
        match e {
            RogueExpertiseChoice::Acrobatics => Some(Skill::Acrobatics),
            RogueExpertiseChoice::AnimalHandling => Some(Skill::AnimalHandling),
            RogueExpertiseChoice::Arcana => Some(Skill::Arcana),
            RogueExpertiseChoice::Athletics => Some(Skill::Athletics),
            RogueExpertiseChoice::Deception => Some(Skill::Deception),
            RogueExpertiseChoice::History => Some(Skill::History),
            RogueExpertiseChoice::Insight => Some(Skill::Insight),
            RogueExpertiseChoice::Intimidation => Some(Skill::Intimidation),
            RogueExpertiseChoice::Investigation => Some(Skill::Investigation),
            RogueExpertiseChoice::Medicine => Some(Skill::Medicine),
            RogueExpertiseChoice::Nature => Some(Skill::Nature),
            RogueExpertiseChoice::Perception => Some(Skill::Perception),
            RogueExpertiseChoice::Performance => Some(Skill::Performance),
            RogueExpertiseChoice::Persuasion => Some(Skill::Persuasion),
            RogueExpertiseChoice::Religion => Some(Skill::Religion),
            RogueExpertiseChoice::SleightOfHand => Some(Skill::SleightOfHand),
            RogueExpertiseChoice::Stealth => Some(Skill::Stealth),
            RogueExpertiseChoice::Survival => Some(Skill::Survival),
            RogueExpertiseChoice::DiebsTools => None,
            RogueExpertiseChoice::Unknown => Some(Skill::Unknown),
        }
    }
}
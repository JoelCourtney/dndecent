// TODO

crate::name!("Assassin");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Assassin;

#[content]
impl RoguishArchetype for Assassin {
    properties! {}

    fn declare(&self, c: &mut Character, lvl: u32) {
        if lvl >= 3 {
            c.tool_proficiencies.declare_modifier(NAME);
        }
    }
    fn iterate(&self, c: &mut Character, lvl: u32) {
        if lvl >= 3 {
            if c.tool_proficiencies.modify(NAME) {
                (*c.tool_proficiencies).push(("Disguise Kit", ProficiencyType::Single));
                (*c.tool_proficiencies).push(("Poisoner's Kit", ProficiencyType::Single));
            }
        }
    }
    fn last(&mut self, c: &mut Character, lvl: u32) {
        if lvl >= 3 {
            c.class_features.push(
                Feature (
                    "# Bonus Proficiencies\n\nWhen you choose this archetype at 3rd level, you gain proficiency with the disguise kit and poisoner's kit.",
                    Empty
                )
            );
        }
    }

    description! {r#"
        # Assassin

        You focus your training on the grim art of death. Those who adhere to this archetype are diverse: hired killers, spies, bounty hunters, and even specially anointed priests trained to exterminate the enemies of their deity. Stealth, poison, and disguise help you eliminate your foes with deadly efficiency.

        ## Bonus Proficiencies

        When you choose this archetype at 3rd level, you gain proficiency with the disguise kit and the poisoner's kit.

        ## Assassinate

        Starting at 3rd level, you are at your deadliest when you get the drop on your enemies. You have advantage on attack rolls against any creature that hasn't taken a turn in the combat yet. In addition, any hit you score against a creature that is surprised is a critical hit.

        ## Infiltration Expertise

        Starting at 9th level, you can unfailingly create false identities for yourself. You must spend seven days and 25 gp to establish the history, profession, and affiliations for an identity. You can't establish an identity that belongs to someone else. For example, you might acquire appropriate clothing, letters of introduction, and official- looking certification to establish yourself as a member of a trading house from a remote city so you can insinuate yourself into the company of other wealthy merchants.

        Thereafter, if you adopt the new identity as a disguise, other creatures believe you to be that person until given an obvious reason not to.

        ## Impostor

        At 13th level, you gain the ability to unerringly mimic another person's speech, writing, and behavior. You must spend at least three hours studying these three components of the person's behavior, listening to speech, examining handwriting, and observing mannerisms.

        Your ruse is indiscernible to the casual observer. If a wary creature suspects something is amiss, you have advantage on any Charisma (Deception) check you make to avoid detection.

        ## Death Strike

        Starting at 17th level, you become a master of instant death. When you attack and hit a creature that is surprised, it must make a Constitution saving throw (DC 8 + your Dexterity modifier + your proficiency bonus). On a failed save, double the damage of your attack against the creature.
    "#}
}


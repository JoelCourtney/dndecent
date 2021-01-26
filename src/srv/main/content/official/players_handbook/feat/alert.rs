crate::name!("Alert");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Alert;

#[content]
impl Feat for Alert {
    properties! {}

    fn declare(&self, c: &mut Character) {
        c.initiative.declare_modifier(NAME);
    }
    fn iterate(&self, c: &mut Character) {
        if c.initiative.modify(NAME) {
            *c.initiative += 5;
        }
    }
    fn last(&mut self, c: &mut Character) {
        c.feat_features.extend(vec! [
            Feature (
                self.description_no_title(),
                Empty
            )
        ]);
    }

    description! { r#"
        # Alert

        Always on the lookout for danger, you gain the following benefits:

        - You gain a +5 bonus to initiative.
        - You can't be surprised while you are conscious.
        - Other creatures don't gain advantage on attack rolls against you as a result of being unseen by you.
    "# }
}

